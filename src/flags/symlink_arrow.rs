use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

/// The flag showing how to display symbolic arrow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymlinkArrow(pub String);

impl Configurable<Self> for SymlinkArrow {
    /// `SymlinkArrow` can not be configured by [Cli]
    ///
    /// Return `None`
    fn from_cli(_: &Cli) -> Option<Self> {
        None
    }
    /// Get a potential `SymlinkArrow` value from a [Config].
    ///
    /// If the `Config::symlink-arrow` has value,
    /// returns its value as the value of the `SymlinkArrow`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config
            .symlink_arrow
            .as_ref()
            .map(|arrow| SymlinkArrow(arrow.to_string()))
    }
}

/// The default value for the `SymlinkArrow` is `\u{21d2}(⇒)`
impl Default for SymlinkArrow {
    fn default() -> Self {
        Self(String::from("\u{21d2}")) // ⇒
    }
}

use std::fmt;
impl fmt::Display for SymlinkArrow {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
