use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing which file system nodes to display.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Display {
    /// windows only, used to show files with system protected flag
    SystemProtected,
    All,
    AlmostAll,
    DirectoryOnly,
    #[default]
    VisibleOnly,
}

impl Configurable<Self> for Display {
    /// Get a potential `Display` variant from [Cli].
    ///
    /// If any of the "all", "almost-all" or "directory-only" arguments is passed, this returns the
    /// corresponding `Display` variant in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.directory_only {
            Some(Self::DirectoryOnly)
        } else if cli.almost_all {
            Some(Self::AlmostAll)
        } else if cli.all {
            Some(Self::All)
        } else if cli.system_protected {
            #[cfg(windows)]
            return Some(Self::SystemProtected);

            #[cfg(not(windows))]
            return Some(Self::All);
        } else {
            None
        }
    }

    /// Get a potential `Display` variant from a [Config].
    ///
    /// If the `Config::display` has value and is one of
    /// "all", "almost-all", "directory-only" or `visible-only`,
    /// this returns the corresponding `Display` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.display
    }
}
