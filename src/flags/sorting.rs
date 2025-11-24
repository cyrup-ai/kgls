use super::Configurable;

use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

/// A collection of flags on how to sort the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Sorting {
    pub column: SortColumn,
    pub order: SortOrder,
    pub dir_grouping: DirGrouping,
}

impl Sorting {
    /// Get a `Sorting` struct from [Cli], a [Config] or the [Default] values.
    ///
    /// The [SortColumn], [SortOrder] and [DirGrouping] are configured with their respective
    /// [Configurable] implementation.
    pub fn configure_from(cli: &Cli, config: &Config) -> Self {
        let column = SortColumn::configure_from(cli, config);
        let order = SortOrder::configure_from(cli, config);
        let dir_grouping = DirGrouping::configure_from(cli, config);
        Self {
            column,
            order,
            dir_grouping,
        }
    }
}

/// The flag showing which column to use for sorting.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SortColumn {
    None,
    Extension,
    #[default]
    Name,
    Time,
    Size,
    Version,
    GitStatus,
}

impl Configurable<Self> for SortColumn {
    /// Get a potential `SortColumn` variant from [Cli].
    ///
    /// If either the "timesort" or "sizesort" arguments are passed, this returns the corresponding
    /// `SortColumn` variant in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        let sort = cli.sort.as_deref();

        if cli.timesort || sort == Some("time") {
            Some(Self::Time)
        } else if cli.sizesort || sort == Some("size") {
            Some(Self::Size)
        } else if cli.extensionsort || sort == Some("extension") {
            Some(Self::Extension)
        } else if cli.versionsort || sort == Some("version") {
            Some(Self::Version)
        } else if cli.gitsort || sort == Some("git") {
            Some(Self::GitStatus)
        } else if cli.no_sort || sort == Some("none") {
            Some(Self::None)
        } else {
            None
        }
    }

    /// Get a potential `SortColumn` variant from a [Config].
    ///
    /// If the `Config::sorting::column` has value and is one of "time", "size" or "name",
    /// this returns the corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.sorting.as_ref().and_then(|s| s.column)
    }
}

/// The flag showing which sort order to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    Default,
    Reverse,
}

impl Configurable<Self> for SortOrder {
    /// Get a potential `SortOrder` variant from [Cli].
    ///
    /// If the "reverse" argument is passed, this returns [SortOrder::Reverse] in a [Some].
    /// Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.reverse {
            Some(Self::Reverse)
        } else {
            None
        }
    }

    /// Get a potential `SortOrder` variant from a [Config].
    ///
    /// If the `Config::sorting::reverse` has value,
    /// this returns a mapped variant in a [Some].
    /// Otherwise [None] is returned.
    /// A `true` maps to [SortOrder::Reverse] while `false` maps to [SortOrder::Default].
    fn from_config(config: &Config) -> Option<Self> {
        config.sorting.as_ref().and_then(|s| match s.reverse {
            Some(true) => Some(Self::Reverse),
            Some(false) => Some(Self::Default),
            None => None,
        })
    }
}

/// The flag showing where to place directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum DirGrouping {
    #[default]
    None,
    First,
    Last,
}

impl DirGrouping {
    pub fn from_arg_str(value: &str) -> Self {
        match value {
            "first" => Self::First,
            "last" => Self::Last,
            "none" => Self::None,
            // Invalid value should be handled by `clap` when building an `Cli`
            other => unreachable!("Invalid value '{other}' for 'group-dirs'"),
        }
    }
}
impl Configurable<Self> for DirGrouping {
    /// Get a potential `DirGrouping` variant from [Cli].
    ///
    /// If the "classic" argument is passed, then this returns the [DirGrouping::None] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.classic {
            return Some(Self::None);
        }

        if cli.group_directories_first {
            return Some(Self::First);
        }

        if let Some(mode) = &cli.group_dirs {
            return Some(Self::from_arg_str(mode));
        }

        None
    }

    /// Get a potential `DirGrouping` variant from a [Config].
    ///
    /// If the `Config::classic` has value and is `true`,
    /// then this returns the the [DirGrouping::None] variant in a [Some].
    /// Otherwise if `Config::sorting::dir-grouping` has value and
    /// is one of "first", "last" or "none", this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if config.classic == Some(true) {
            Some(Self::None)
        } else {
            config.sorting.as_ref().and_then(|s| s.dir_grouping)
        }
    }
}
