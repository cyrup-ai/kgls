use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to print file type indicators.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Indicators(pub bool);

impl Configurable<Self> for Indicators {
    /// Get a potential `Indicators` value from [Cli].
    ///
    /// If the "indicators" argument is passed, this returns an `Indicators` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.indicators {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Indicators` value from a [Config].
    ///
    /// If the `Config::indicators` has value,
    /// this returns its value as the value of the `Indicators`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.indicators.as_ref().map(|ind| Self(*ind))
    }
}
