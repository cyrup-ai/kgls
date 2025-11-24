use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing which file size units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SizeFlag {
    /// The variant to show file size with SI unit prefix and a B for bytes.
    #[default]
    Default,
    /// The variant to show file size with only the SI unit prefix.
    Short,
    /// The variant to show file size in bytes.
    Bytes,
}

impl SizeFlag {
    pub fn from_arg_str(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "short" => Self::Short,
            "bytes" => Self::Bytes,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'size'"),
        }
    }
}

impl Configurable<Self> for SizeFlag {
    /// Get a potential `SizeFlag` variant from [Cli].
    ///
    /// If any of the "default", "short" or "bytes" arguments is passed, the corresponding
    /// `SizeFlag` variant is returned in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Bytes)
        } else {
            cli.size.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `SizeFlag` variant from a [Config].
    ///
    /// If the `Config::size` has value and is one of "default", "short" or "bytes",
    /// this returns the corresponding `SizeFlag` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Bytes)
        } else {
            config.size
        }
    }
}
