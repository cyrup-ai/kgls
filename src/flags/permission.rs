use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing which file permissions units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionFlag {
    /// The variant to show file permissions in rwx format
    #[cfg_attr(not(target_os = "windows"), default)]
    Rwx,
    /// The variant to show file permissions in octal format
    Octal,
    /// (windows only): Attributes from powershell's `Get-ChildItem`
    #[cfg_attr(target_os = "windows", default)]
    Attributes,
    /// Disable the display of owner and permissions, may be used to speed up in Windows
    Disable,
}

impl PermissionFlag {
    pub fn from_arg_str(value: &str) -> Self {
        match value {
            "rwx" => Self::Rwx,
            "octal" => Self::Octal,
            "attributes" => Self::Attributes,
            "disable" => Self::Disable,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'permission'"),
        }
    }
}

impl Configurable<Self> for PermissionFlag {
    /// Get a potential `PermissionFlag` variant from [Cli].
    ///
    /// If any of the "rwx" or "octal" arguments is passed, the corresponding
    /// `PermissionFlag` variant is returned in a [Some]. If neither of them is passed,
    /// this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Rwx)
        } else {
            cli.permission.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `PermissionFlag` variant from a [Config].
    ///
    /// If the `Config::permissions` has value and is one of "rwx" or "octal",
    /// this returns the corresponding `PermissionFlag` variant in a [Some].
    /// Otherwise this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Rwx)
        } else {
            config.permission
        }
    }
}
