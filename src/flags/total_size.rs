//! This module defines the [TotalSize] flag. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing whether to show the total size for directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct TotalSize(pub bool);

impl Configurable<Self> for TotalSize {
    /// Get a potential `TotalSize` value from [Cli].
    ///
    /// If the "total-size" argument is passed, this returns a `TotalSize` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.total_size {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `TotalSize` value from a [Config].
    ///
    /// If the `Config::total-size` has value,
    /// this returns it as the value of the `TotalSize`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.total_size.map(Self)
    }
}
