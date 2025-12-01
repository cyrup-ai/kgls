use crate::color::Colors;
use crate::display;
use crate::flags::{
    ColorOption, Flags, HyperlinkOption, Layout, Literal, SortOrder, ThemeOption,
};
use crate::icon::Icons;

use crate::meta::Meta;
use crate::{print_output, sort, ExitCode};
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::io;
#[cfg(not(target_os = "windows"))]
use std::os::unix::io::AsRawFd;

use crate::git_theme::GitTheme;
#[cfg(target_os = "windows")]
use terminal_size::terminal_size;

pub struct Core {
    flags: Flags,
    icons: Icons,
    colors: Colors,
    git_theme: GitTheme,
    sorters: Vec<(SortOrder, sort::SortFn)>,
    stdout_writer: Option<Box<dyn std::io::Write + Send>>,
    stderr_writer: Option<Box<dyn std::io::Write + Send>>,
}

impl Core {
    pub fn new(mut flags: Flags) -> Self {
        // Check through libc if stdout is a tty. Unix specific so not on windows.
        // Determine color output availability (and initialize color output (for Windows 10))
        #[cfg(not(target_os = "windows"))]
        let tty_available = unsafe { libc::isatty(io::stdout().as_raw_fd()) == 1 };

        #[cfg(not(target_os = "windows"))]
        let console_color_ok = true;

        #[cfg(target_os = "windows")]
        let tty_available = terminal_size().is_some(); // terminal_size allows us to know if the stdout is a tty or not.

        #[cfg(target_os = "windows")]
        let console_color_ok = crossterm::ansi_support::supports_ansi();

        let color_theme = match (tty_available && console_color_ok, flags.color.when) {
            (_, ColorOption::Never) | (false, ColorOption::Auto) => ThemeOption::NoColor,
            _ => flags.color.theme.clone(),
        };

        let icon_when = flags.icons.when;
        let icon_theme = flags.icons.theme.clone();

        // TODO: Rework this so that flags passed downstream does not
        // have Auto option for any (icon, color, hyperlink).
        if matches!(flags.hyperlink, HyperlinkOption::Auto) {
            flags.hyperlink = if tty_available {
                HyperlinkOption::Always
            } else {
                HyperlinkOption::Never
            }
        }

        let icon_separator = flags.icons.separator.0.clone();

        // The output is not a tty, this means the command is piped. e.g.
        //
        // lsd -l | less
        //
        // Most of the programs does not handle correctly the ansi colors
        // or require a raw output (like the `wc` command).
        if !tty_available {
            // we should not overwrite the tree layout
            if flags.layout != Layout::Tree {
                flags.layout = Layout::OneLine;
            }

            flags.literal = Literal(true);
        };

        let sorters = sort::assemble_sorters(&flags);

        Self {
            flags,
            colors: Colors::new(color_theme),
            icons: Icons::new(tty_available, icon_when, icon_theme, icon_separator),
            git_theme: GitTheme::new(),
            sorters,
            stdout_writer: None,
            stderr_writer: None,
        }
    }

    /// Configure custom output writers for library usage.
    /// 
    /// This allows integration with systems that redirect stdout/stderr
    /// (e.g., shell builtins, testing frameworks, logging systems).
    ///
    /// # Example
    /// ```no_run
    /// use kgls::Core;
    /// use std::io::Cursor;
    ///
    /// let mut stdout_buf = Cursor::new(Vec::new());
    /// let mut stderr_buf = Cursor::new(Vec::new());
    /// 
    /// let core = Core::new(flags)
    ///     .with_writers(stdout_buf, stderr_buf);
    /// ```
    pub fn with_writers(
        mut self,
        stdout: impl std::io::Write + Send + 'static,
        stderr: impl std::io::Write + Send + 'static,
    ) -> Self {
        self.stdout_writer = Some(Box::new(stdout));
        self.stderr_writer = Some(Box::new(stderr));
        self
    }

    pub async fn run(mut self, paths: Vec<PathBuf>) -> ExitCode {
        // Validate paths exist before processing
        let mut exit_code = ExitCode::OK;
        let mut valid_paths = Vec::new();
        
        for path in &paths {
            if !path.exists() {
                log::error!("Cannot access '{}': No such file or directory", path.display());
                self.write_error(format!("kgls: cannot access '{}': No such file or directory", path.display()));
                exit_code.set_if_greater(ExitCode::MinorIssue);
            } else {
                valid_paths.push(path.clone());
            }
        }
        
        // If no valid paths, return early
        if valid_paths.is_empty() {
            return exit_code;
        }
        
        // Determine traversal depth based on flags (copied from fetch() logic)
        let depth = match self.flags.layout {
            Layout::Tree => self.flags.recursion.depth,
            _ if self.flags.recursion.enabled => self.flags.recursion.depth,
            _ => 1,
        };

        // Build streaming pipeline
        let file_stream = crate::stream::FileStream::new(
            valid_paths.clone(),
            depth,
            &self.flags.ignore_globs,
            self.flags.display,
        );

        // Route to appropriate output mode
        let stream_exit_code = if self.flags.layout == Layout::Tree {
            self.display_tree_stream(file_stream, &valid_paths).await
        } else {
            // Grid/OneLine modes: buffer temporarily (can optimize with GridAccumulator later)
            self.display_buffered(file_stream).await
        };
        
        // Combine exit codes (take the greater error level)
        exit_code.set_if_greater(stream_exit_code);
        exit_code
    }

    async fn display_tree_stream(
        &mut self,
        file_stream: crate::stream::FileStream,
        _paths: &[PathBuf],
    ) -> ExitCode {
        use futures::StreamExt;
        use std::collections::HashMap;

        // Buffer all entries and organize hierarchically
        let mut entries = Vec::new();
        let mut exit_code = ExitCode::OK;

        let mut stream = Box::pin(file_stream);
        while let Some(result) = stream.next().await {
            match result {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    log::error!("Stream error: {}", e);
                    self.write_error(format!("Stream error: {}", e));
                    exit_code.set_if_greater(ExitCode::MinorIssue);
                }
            }
        }

        // Sort by depth descending so we process deepest children first
        // This ensures children have their descendants before being cloned to parents
        entries.sort_by(|a, b| b.depth.cmp(&a.depth));

        // Convert entries to Meta and build hierarchy
        let mut meta_map: HashMap<PathBuf, Meta> = HashMap::new();
        for entry in &entries {
            let meta = entry.to_meta(self.flags.permission);
            meta_map.insert(entry.path.clone(), meta);
        }

        // Build tree structure by attaching children to parents
        // First pass: identify which paths have parents in the map
        let mut child_paths = std::collections::HashSet::new();
        for entry in &entries {
            if let Some(parent_path) = entry.path.parent()
                && meta_map.contains_key(parent_path) {
                    child_paths.insert(entry.path.clone());
                }
        }

        // Second pass: build parent-child relationships
        for entry in &entries {
            if let Some(parent_path) = entry.path.parent()
                && child_paths.contains(&entry.path) {
                    let child_meta = meta_map.get(&entry.path).unwrap().clone();
                    if let Some(parent_meta) = meta_map.get_mut(parent_path) {
                        if parent_meta.content.is_none() {
                            parent_meta.content = Some(Vec::new());
                        }
                        if let Some(content) = &mut parent_meta.content {
                            content.push(child_meta);
                        }
                    }
                }
        }

        // Phase 2.5: Detect and warn about orphaned entries
        // Entries that have a parent path but weren't attached to any parent
        // This happens when parent directories are filtered but children aren't
        for entry in &entries {
            // Skip depth 1 entries - their parent is the scan root, not an orphaned case
            if entry.depth <= 1 {
                continue;
            }

            if let Some(parent_path) = entry.path.parent() {
                // Skip if successfully attached to parent
                if child_paths.contains(&entry.path) {
                    continue;
                }

                // Entry has parent but wasn't attached - parent was filtered
                log::warn!(
                    "Entry '{}' orphaned (parent '{}' was filtered)",
                    entry.path.display(),
                    parent_path.display()
                );
            }
        }

        // Third pass: collect root metas (those not in child_paths)
        let mut root_metas = Vec::new();
        for entry in &entries {
            if !child_paths.contains(&entry.path)
                && let Some(meta) = meta_map.get(&entry.path) {
                    root_metas.push(meta.clone());
                }
        }

        // Sort root metas
        self.sort(&mut root_metas);

        // Display using existing tree display logic
        let output = display::tree(
            &root_metas,
            &self.flags,
            &self.colors,
            &self.icons,
            &self.git_theme,
        );

        self.write_output(output);
        exit_code
    }

    async fn display_buffered(
        &mut self,
        file_stream: crate::stream::FileStream,
    ) -> ExitCode {
        use futures::StreamExt;

        // Buffer entries from stream
        let mut entries = Vec::new();
        let mut exit_code = ExitCode::OK;

        let mut stream = Box::pin(file_stream);
        while let Some(result) = stream.next().await {
            match result {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    log::error!("Stream error: {}", e);
                    self.write_error(format!("Stream error: {}", e));
                    exit_code.set_if_greater(ExitCode::MinorIssue);
                }
            }
        }

        // Convert FileEntry to Meta
        let mut metas: Vec<Meta> = entries
            .iter()
            .map(|entry| entry.to_meta(self.flags.permission))
            .collect();

        // Sort using configured sorters
        self.sort(&mut metas);

        // Display using existing grid/oneline display logic
        let output = display::grid(
            &metas,
            &self.flags,
            &self.colors,
            &self.icons,
            &self.git_theme,
        );

        self.write_output(output);
        exit_code
    }



    fn sort(&self, metas: &mut Vec<Meta>) {
        metas.sort_unstable_by(|a, b| sort::by_meta(&self.sorters, a, b));

        for meta in metas {
            if let Some(ref mut content) = meta.content {
                self.sort(content);
            }
        }
    }

    /// Write output to custom stdout writer if provided, otherwise use stdout
    fn write_output(&mut self, content: impl std::fmt::Display) {
        use std::io::Write;
        
        if let Some(writer) = &mut self.stdout_writer {
            // Custom writer provided - write and flush
            let _ = write!(writer, "{}", content);
            let _ = writer.flush();
        } else {
            // No custom writer - use print_output! macro for binary mode
            print_output!("{}", content);
        }
    }

    /// Write error to custom stderr writer if provided, otherwise use stderr
    fn write_error(&mut self, content: impl std::fmt::Display) {
        use std::io::Write;
        
        if let Some(writer) = &mut self.stderr_writer {
            // Custom writer provided
            let _ = writeln!(writer, "{}", content);
        } else {
            // No custom writer - use eprintln! for binary mode
            eprintln!("{}", content);
        }
    }
}
