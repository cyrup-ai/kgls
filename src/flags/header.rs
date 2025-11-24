use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to display block headers.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Header(pub bool);

impl Configurable<Self> for Header {
    /// Get a potential `Header` value from [Cli].
    ///
    /// If the "header" argument is passed, this returns a `Header` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.header {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Header` value from a [Config].
    ///
    /// If the `Config::header` has value,
    /// this returns it as the value of the `Header`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.header.map(Self)
    }
}
