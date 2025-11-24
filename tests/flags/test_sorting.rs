use clap::Parser;

use kgls::flags::sorting::{SortColumn, SortOrder, DirGrouping};

use kgls::app::Cli;
use kgls::config_file::{Config, Sorting};
use kgls::flags::Configurable;

mod test_sort_column {
    use super::*;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_extension() {
        let argv = ["lsd", "--extensionsort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_time() {
        let argv = ["lsd", "--timesort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_size() {
        let argv = ["lsd", "--sizesort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Size), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_git() {
        let argv = ["lsd", "--gitsort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::GitStatus), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_version() {
        let argv = ["lsd", "--versionsort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Version), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_no_sort() {
        let argv = ["lsd", "--no-sort"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::None), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_sort() {
        let argv = ["lsd", "--sort", "time"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "size"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Size), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "extension"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "version"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Version), SortColumn::from_cli(&cli));

        let argv = ["lsd", "--sort", "none"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::None), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_arg_cli_sort_git() {
        let argv = ["lsd", "--sort", "git"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::GitStatus), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_multi_sort() {
        let argv = ["lsd", "--sort", "size", "--sort", "time"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_multi_sort_use_last() {
        let argv = ["lsd", "--sort", "size", "-t", "-S", "-X", "--sort", "time"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortColumn::Time), SortColumn::from_cli(&cli));
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, SortColumn::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty_column() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });

        assert_eq!(None, SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_extension() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Extension),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_name() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Name),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Name), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_time() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Time),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Time), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_size() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Size),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Size), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_version() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Version),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Version), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_git_status() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::GitStatus),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::GitStatus), SortColumn::from_config(&c));
    }
}

mod test_sort_order {
    use super::*;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, SortOrder::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_reverse() {
        let argv = ["lsd", "--reverse"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(SortOrder::Reverse), SortOrder::from_cli(&cli));
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, SortOrder::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_default_config() {
        assert_eq!(
            Some(SortOrder::default()),
            SortOrder::from_config(&Config::builtin())
        );
    }

    #[test]
    fn test_from_config_empty_reverse() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(None, SortOrder::from_config(&c));
    }

    #[test]
    fn test_from_config_reverse_true() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: Some(true),
            dir_grouping: None,
        });
        assert_eq!(Some(SortOrder::Reverse), SortOrder::from_config(&c));
    }

    #[test]
    fn test_from_config_reverse_false() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: Some(false),
            dir_grouping: None,
        });
        assert_eq!(Some(SortOrder::Default), SortOrder::from_config(&c));
    }
}

mod test_dir_grouping {
    use super::*;

    #[test]
    #[should_panic]
    fn test_from_str_bad_value() {
        DirGrouping::from_arg_str("bad value");
    }

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_first() {
        let argv = ["lsd", "--group-dirs", "first"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_last() {
        let argv = ["lsd", "--group-dirs", "last"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_explicit_none() {
        let argv = ["lsd", "--group-dirs", "none"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--group-dirs", "first", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_group_dirs_multi() {
        let argv = ["lsd", "--group-dirs", "first", "--group-dirs", "last"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_group_directories_first() {
        let argv = ["lsd", "--group-directories-first"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_cli(&cli));
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, DirGrouping::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_first() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::First),
        });
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_last() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::Last),
        });
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_explicit_empty() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(None, DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::Last),
        });
        c.classic = Some(true);
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_config(&c));
    }
}
