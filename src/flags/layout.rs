use crate::app::Cli;
use crate::config_file::Config;

use super::Configurable;

use serde::Deserialize;

/// The flag showing which output layout to print.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    #[default]
    Grid,
    Tree,
    OneLine,
}

impl Configurable<Layout> for Layout {
    /// Get a potential `Layout` variant from [Cli].
    ///
    /// If any of the "tree", "long" or "oneline" arguments is passed, this returns the
    /// corresponding `Layout` variant in a [Some]. Otherwise if the number of passed "blocks"
    /// arguments is greater than 1, this also returns the [OneLine](Layout::OneLine) variant.
    /// Finally if neither of them is passed, this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.tree {
            Some(Self::Tree)
        } else if cli.long || cli.oneline || cli.inode || cli.context || cli.blocks.len() > 1
        // TODO: handle this differently
        {
            Some(Self::OneLine)
        } else {
            None
        }
    }

    /// Get a potential Layout variant from a [Config].
    ///
    /// If the `Config::layout` has value and is one of "tree", "oneline" or "grid",
    /// this returns the corresponding `Layout` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.layout
    }
}
