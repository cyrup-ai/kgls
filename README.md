<div align="center">
  <img src="assets/img/banner.png" alt="Kodegen AI Banner" width="100%" />
</div>

  # KGLS - a drop in `ls` replacement

  **A blazing-fast drop-in replacement for `lsd`**

  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
  [![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
</div>

---

## Overview

KGLS (Kodegen ls) is a high-performance file system tool that combines the best features of `lsd`. Built in Rust 2024, it provides beautiful human-friendly terminal output with modern features.

### Why KGLS?

- **🚀 Blazing Fast** - Parallel directory traversal with `jwalk` and `rayon`
- **🔄 Drop-in Replacement** - Compatible with `lsd` command-line options
- **🎨 Beautiful Output** - Rich formatting with icons, colors, and tree views
- **⚡ Smart Filtering** - Efficient glob-based exclusions during traversal
- **📊 Git Integration** - Display git status directly in listings

---

## Installation

```bash
cargo install --git https://github.com/cyrup-ai/kgls
```

---

## Quick Start

```bash
# Basic usage (drop-in replacement for lsd)
kgls

# Tree view with ignored directories
kgls --tree --ignore-glob 'node_modules' --ignore-glob '.git'

# Long format with git status
kgls -l --git
```

---

## Core Features

### Display Modes

| Mode | Flag | Description |
|------|------|-------------|
| Grid | *(default)* | Multi-column grid layout |
| Tree | `--tree` | Recursive tree visualization |
| Long | `-l, --long` | Extended metadata table |
| One Line | `-1, --oneline` | Single entry per line |

### Filtering & Display

```bash
# Show all files including hidden
kgls -a, --all

# Show almost all (exclude . and ..)
kgls -A, --almost-all

# Ignore patterns (supports multiple)
kgls -I, --ignore-glob '*.log' --ignore-glob 'tmp'

# Directory only view
kgls -d, --directory-only

# Literal names (no quoting)
kgls -N, --literal
```

### Recursion & Depth

```bash
# Recursive listing
kgls -R, --recursive

# Tree view with depth limit
kgls --tree --depth 3

# Unlimited depth tree
kgls --tree  # Uses max depth by default
```

### Sorting Options

```bash
# Time-based sorting
kgls -t, --timesort

# Size-based sorting
kgls -S, --sizesort

# Extension sorting
kgls -X, --extensionsort

# Git status sorting
kgls -G, --gitsort

# Natural version sorting
kgls -v, --versionsort

# Custom sort type
kgls --sort <TYPE>  # size|time|version|extension|git|none

# Disable sorting (directory order)
kgls -U, --no-sort

# Reverse order
kgls -r, --reverse

# Group directories
kgls --group-dirs <first|last|none>
kgls --group-directories-first  # Alias for --group-dirs=first
```

### Customization

```bash
# Color control
kgls --color <always|auto|never>

# Icon settings
kgls --icon <always|auto|never>
kgls --icon-theme <fancy|unicode>

# Permission display
kgls --permission <rwx|octal|attributes|disable>

# Size display format
kgls --size <default|short|bytes>

# Date format
kgls --date <date|locale|relative|+custom-format>

# Custom blocks (choose what to display)
kgls --blocks <permission,user,group,size,date,name,inode,links,git>

# Classic mode (ls-like output)
kgls --classic
```

### Advanced Features

```bash
# Display total directory sizes
kgls --total-size

# Show inode numbers
kgls -i, --inode

# Git status indicators
kgls -g, --git  # (requires --long)

# Dereference symlinks
kgls -L, --dereference

# Security context (SELinux)
kgls -Z, --context

# Hyperlinks to files
kgls --hyperlink <always|auto|never>

# Display column headers
kgls --header

# Truncate long owner names
kgls --truncate-owner-after <NUM>
kgls --truncate-owner-marker <STR>

# Don't display symlink targets
kgls --no-symlink
```

---

## Configuration

KGLS supports configuration files for persistent settings:

```bash
# Use custom config
kgls --config-file ~/.config/kgls/config.toml

# Ignore default config
kgls --ignore-config
```

### Config File Example

```toml
# ~/.config/kgls/config.toml
layout = "tree"
classic = false
display = "all"
no-symlink = false
total-size = false
header = false
literal = false

# Ignore patterns (applied during traversal for performance)
ignore_globs = ["node_modules", ".git", "target", "*.log", "*.swp"]

[recursion]
enabled = true
depth = 5

[icons]
when = "always"
theme = "fancy"
separator = " "

[color]
when = "always"

[sorting]
column = "name"
reverse = false
dir-grouping = "first"
```

---

## Performance Features

KGLS is built for speed:

- **Parallel Traversal** - Uses `jwalk` for multi-threaded directory walking
- **Streaming Architecture** - Process entries as they arrive, no buffering overhead
- **Smart Filtering** - Ignore patterns applied during traversal (not post-processing)
- **Efficient Git Integration** - Uses `gix` (pure Rust) instead of libgit2
- **Zero-Copy Where Possible** - Minimize allocations and data copying

### Benchmarks

```bash
# Typical speedup vs traditional ls
kgls --tree large_project/     # ~3-5x faster than lsd
kgls -R --ignore-glob 'node_modules'  # Filters during traversal
```

---

## Architecture Highlights

- **Rust 2024 Edition** - Latest language features and optimizations
- **Async Streaming** - Futures-based for efficient I/O
- **Parallel Processing** - Multi-threaded directory traversal
- **Git-Aware** - Native repository detection and status tracking

---

## Complete CLI Reference

### General Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-a` | `--all` | Show all entries including hidden (starting with .) |
| `-A` | `--almost-all` | Show all except . and .. |
| `-F` | `--classify` | Append indicator to filenames (*/=>@\|) |
| `-l` | `--long` | Long format with extended metadata |
| `-1` | `--oneline` | One entry per line |
| `-R` | `--recursive` | Recurse into directories |
| `-h` | `--human-readable` | Human-readable sizes (default) |
| `-d` | `--directory-only` | List directories themselves, not contents |
| `-i` | `--inode` | Show inode numbers |
| `-g` | `--git` | Show git status (requires -l) |
| `-L` | `--dereference` | Follow symbolic links |
| `-Z` | `--context` | Show security context |
| `-N` | `--literal` | Don't quote entry names |
| `-V` | `--version` | Show version |
|      | `--help` | Show help information |

### Layout Options

| Flag | Description |
|------|-------------|
| `--tree` | Tree view with hierarchical structure |
| `--depth <NUM>` | Maximum recursion depth |
| `--classic` | Classic ls-style output |

### Sort Options

| Flag | Long Form | Values | Description |
|------|-----------|--------|-------------|
| `-t` | `--timesort` | - | Sort by modification time |
| `-S` | `--sizesort` | - | Sort by file size |
| `-X` | `--extensionsort` | - | Sort by file extension |
| `-G` | `--gitsort` | - | Sort by git status |
| `-v` | `--versionsort` | - | Natural version number sort |
| `-U` | `--no-sort` | - | No sorting (directory order) |
| `-r` | `--reverse` | - | Reverse sort order |
|      | `--sort` | `size\|time\|version\|extension\|git\|none` | Specify sort type |
|      | `--group-dirs` | `first\|last\|none` | Group directories |
|      | `--group-directories-first` | - | Alias for --group-dirs=first |

### Display Customization

| Flag | Values | Description |
|------|--------|-------------|
| `--color` | `always\|auto\|never` | Color output control |
| `--icon` | `always\|auto\|never` | Icon display control |
| `--icon-theme` | `fancy\|unicode` | Icon style |
| `--permission` | `rwx\|octal\|attributes\|disable` | Permission format |
| `--size` | `default\|short\|bytes` | Size display format |
| `--date` | `date\|locale\|relative\|+format` | Date format |
| `--hyperlink` | `always\|auto\|never` | Hyperlink files |
| `--blocks` | `permission,user,group,size,date,name,inode,links,git` | Custom block order |
| `--header` | - | Display block headers |
| `--total-size` | - | Show total directory sizes |
| `--no-symlink` | - | Don't show symlink targets |
| `--truncate-owner-after` | `<NUM>` | Truncate owner names after N chars |
| `--truncate-owner-marker` | `<STR>` | Marker for truncated names |

### Filtering

| Flag | Description |
|------|-------------|
| `-I, --ignore-glob <PATTERN>` | Exclude files matching glob (repeatable) |

### Configuration

| Flag | Description |
|------|-------------|
| `--ignore-config` | Ignore configuration file |
| `--config-file <PATH>` | Use custom config file |

---

## Examples

### Basic Usage

```bash
# Simple listing
kgls

# Show hidden files
kgls -a

# Long format with icons
kgls -l --icon always

# Colored output even when piped
kgls --color always | less -R
```

### Tree Views

```bash
# Basic tree
kgls --tree

# Tree with depth limit
kgls --tree --depth 2

# Tree with filters
kgls --tree --ignore-glob 'node_modules' --ignore-glob '.git' --ignore-glob 'target'

# Tree with directories only
kgls --tree -d
```

### Sorting Examples

```bash
# Sort by size, largest first
kgls -S -r

# Sort by modification time (newest first)
kgls -t

# Natural version sorting
kgls -v

# Git status sorting with details
kgls -l -g -G

# Group directories first, sort by size
kgls --group-directories-first -S
```

### Advanced Filtering

```bash
# Ignore multiple patterns
kgls --tree -I '*.log' -I 'tmp' -I '.cache'

# Show only specific file types
kgls | grep '.rs$'

# Custom blocks
kgls --blocks permission,size,name
```

### Custom Formatting

```bash
# Octal permissions
kgls -l --permission octal

# Short size format
kgls -l --size short

# Relative dates
kgls -l --date relative

# Custom date format
kgls -l --date '+%Y-%m-%d %H:%M'

# Unicode icons
kgls --icon-theme unicode
```

---

## Comparison with lsd

KGLS is designed as a drop-in replacement for lsd with additional features:

| Feature | lsd | KGLS |
|---------|-----|-----|
| Tree view | ✅ | ✅ |
| Git integration | ✅ | ✅ |
| Icons & colors | ✅ | ✅ |
| Parallel traversal | ✅ | ✅ Enhanced |
| Smart filtering | ❌ | ✅ (during traversal) |
| Streaming architecture | ❌ | ✅ |
| Pure Rust Git (`gix`) | ❌ | ✅ |

### Migration from lsd

Simply alias or replace:

```bash
# In your shell config (.bashrc, .zshrc, etc.)
alias lsd='kgls'

# Or install as lsd replacement
ln -s $(which kgls) /usr/local/bin/lsd
```

All lsd commands work identically:

```bash
lsd -la --tree    # Works exactly the same with kgls
```

---

## Contributing

Contributions are welcome! Please see [ARCHITECTURE.md](ARCHITECTURE.md) for development guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/cyrup-ai/kgls
cd kgls

# Install cargo-nextest (if not already installed)
cargo install cargo-nextest

# Build
cargo build --release

# Run tests (using nextest)
cargo nextest run

# Run specific test
cargo nextest run test_name

# Run with development binary
./target/debug/kgls [args]

# Check code (clippy warnings as errors)
cargo clippy --all-targets -- -D warnings
```

### Project Structure

```
kgls/
├── src/
│   ├── core.rs           # Core orchestration logic
│   ├── stream/           # Streaming infrastructure
│   ├── meta/             # File metadata extraction
│   ├── display.rs        # Output rendering
│   ├── flags/            # CLI flag handling
│   ├── git.rs            # Git integration
│   ├── icon.rs           # Icon handling
│   ├── color.rs          # Color themes
│   └── ...
└── tests/
    ├── test_lib.rs       # Library tests
    ├── test_flags.rs     # Flag integration tests
    └── flags/            # Individual flag module tests
        ├── test_ignore_globs.rs
        ├── test_recursion.rs
        ├── test_display.rs
        └── ...
```

All tests are located in the `./tests` directory, mirroring the structure of `./src`. Tests are run using [cargo-nextest](https://nexte.st/) for faster parallel execution.

### Testing Guidelines

**Test Organization:**
- Integration tests are in `./tests/` with structure mirroring `./src/`
- Test files are prefixed with `test_` (e.g., `test_lib.rs`, `test_recursion.rs`)
- No tests should appear in `./src/**/*.rs` files

**Running Tests:**
```bash
# Run all tests
cargo nextest run

# Run specific test file
cargo nextest run test_lib

# Run specific test function
cargo nextest run test_exit_code_ordering

# Run with verbose output
cargo nextest run -v

# Run ignored tests (like performance benchmarks)
cargo nextest run --ignored
```

**Test Statistics:**
- **157 total tests** across all modules
- All tests pass on every commit
- Clippy clean (no warnings or errors)

---

## Roadmap

- [ ] AST metadata for code files (via tree-sitter)
- [ ] Cargo.toml intelligence for Rust projects
- [ ] Interactive TUI mode (via ratatui)
- [ ] Code-aware features (function/struct detection)
- [ ] Plugin system for custom transformers
- [ ] Performance profiling dashboard

---

## License

KGLS is dual-licensed under your choice of:

* **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

This means you can choose either license when using this software.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## Acknowledgments

- Built on top of the excellent [jwalk](https://github.com/byron/jwalk) library
- Inspired by [lsd](https://github.com/lsd-rs/lsd)
- Git support through [gix](https://github.com/Byron/gitoxide)

---

<div align="center">
  
  **[Documentation](https://github.com/cyrup-ai/kgls)** • 
  **[Report Issues](https://github.com/cyrup-ai/kgls/issues)** • 
  **[Contribute](https://github.com/cyrup-ai/kgls/pulls)**
  
  Made with ❤️ by [Cyrup AI](https://github.com/cyrup-ai)
  
</div>
