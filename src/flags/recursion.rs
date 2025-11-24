use crate::app::Cli;
use crate::config_file::Config;

/// The options relating to recursion.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Recursion {
    /// Whether the recursion into directories is enabled.
    pub enabled: bool,
    /// The depth for how far to recurse into directories.
    pub depth: usize,
}

impl Recursion {
    /// Get the Recursion from either [Cli], a [Config] or the [Default] value.
    ///
    /// The "enabled" value is determined by [enabled_from](Recursion::enabled_from) and the depth
    /// value is determined by [depth_from](Recursion::depth_from).
    ///
    /// # Errors
    ///
    /// If [depth_from](Recursion::depth_from) returns an [Error], this returns it.
    pub fn configure_from(cli: &Cli, config: &Config) -> Self {
        let enabled = Self::enabled_from(cli, config);
        let depth = Self::depth_from(cli, config);
        Self { enabled, depth }
    }

    /// Get the "enabled" boolean from [Cli], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [enabled_from_cli](Recursion::enabled_from_cli)
    /// - [Config.recursion.enabled]
    /// - [Default::default]
    pub(crate) fn enabled_from(cli: &Cli, config: &Config) -> bool {
        if let Some(value) = Self::enabled_from_cli(cli) {
            return value;
        }
        if let Some(recursion) = &config.recursion
            && let Some(enabled) = recursion.enabled {
                return enabled;
            }

        Default::default()
    }

    /// Get a potential "enabled" boolean from [Cli].
    ///
    /// If the "recursive" argument is passed, this returns `true` in a [Some]. Otherwise this
    /// returns [None].
    pub(crate) fn enabled_from_cli(cli: &Cli) -> Option<bool> {
        if cli.recursive {
            Some(true)
        } else {
            None
        }
    }

    /// Get the "depth" integer from [Cli], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - Cli::depth
    /// - [Config.recursion.depth]
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// If both configuration file and Args is error, this will return a Max-Uint value.
    pub(crate) fn depth_from(cli: &Cli, config: &Config) -> usize {
        if let Some(value) = cli.depth {
            return value;
        }

        use crate::config_file::Recursion;
        if let Some(Recursion {
            depth: Some(value), ..
        }) = &config.recursion
        {
            return *value;
        }

        usize::MAX
    }
}

/// The default values for `Recursion` are the boolean default and [prim@usize::max_value()].
impl Default for Recursion {
    fn default() -> Self {
        Self {
            depth: usize::MAX,
            enabled: false,
        }
    }
}
