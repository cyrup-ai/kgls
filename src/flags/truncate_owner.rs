use super::Configurable;
use crate::app::Cli;

use crate::config_file::Config;

/// The flag showing how to truncate user and group names.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TruncateOwner {
    pub after: Option<usize>,
    pub marker: Option<String>,
}

impl Configurable<Self> for TruncateOwner {
    /// Get a potential `TruncateOwner` value from [Cli].
    ///
    /// If the "header" argument is passed, this returns a `TruncateOwner` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        match (cli.truncate_owner_after, cli.truncate_owner_marker.clone()) {
            (None, None) => None,
            (after, marker) => Some(Self { after, marker }),
        }
    }

    /// Get a potential `TruncateOwner` value from a [Config].
    ///
    /// If the `Config::truncate_owner` has value,
    /// this returns it as the value of the `TruncateOwner`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.truncate_owner.as_ref().map(|c| Self {
            after: c.after,
            marker: c.marker.clone(),
        })
    }
}
