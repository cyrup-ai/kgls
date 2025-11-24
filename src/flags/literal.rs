use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag to set in order to show literal file names without quotes.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Literal(pub bool);

impl Configurable<Self> for Literal {
    /// Get a potential `Literal` value from [Cli].
    ///
    /// If the "literal" argument is passed, this returns a `Literal` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.literal {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Literal` value from a [Config].
    ///
    /// If the `Config::indicators` has value,
    /// this returns its value as the value of the `Literal`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.literal.map(Self)
    }
}
