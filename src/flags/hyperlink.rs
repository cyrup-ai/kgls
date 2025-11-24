use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// The flag showing when to use hyperlink in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum HyperlinkOption {
    Always,
    Auto,
    #[default]
    Never,
}

impl HyperlinkOption {
    pub fn from_arg_str(value: &str) -> Self {
        match value {
            "always" => Self::Always,
            "auto" => Self::Auto,
            "never" => Self::Never,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'hyperlink'"),
        }
    }
}

impl Configurable<Self> for HyperlinkOption {
    /// Get a potential `HyperlinkOption` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [HyperlinkOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            Some(Self::Never)
        } else {
            cli.hyperlink.as_deref().map(Self::from_arg_str)
        }
    }

    /// Get a potential `HyperlinkOption` variant from a [Config].
    ///
    /// If the `Configs::classic` has value and is "true" then this returns Some(HyperlinkOption::Never).
    /// Otherwise if the `Config::hyperlink::when` has value and is one of "always", "auto" or "never",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::Never)
        } else {
            config.hyperlink
        }
    }
}
