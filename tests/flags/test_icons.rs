use clap::Parser;

use kgls::flags::icons::{IconOption, IconTheme, IconSeparator};

use kgls::app::Cli;
use kgls::config_file::{Config, Icons};
use kgls::flags::Configurable;

mod test_icon_option {
    use super::*;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_always() {
        let argv = ["lsd", "--icon", "always"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Always), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_auto() {
        let argv = ["lsd", "--icon", "auto"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Auto), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_never() {
        let argv = ["lsd", "--icon", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Never), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_classic_mode() {
        let argv = ["lsd", "--icon", "always", "--classic"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Never), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_icon_when_multi() {
        let argv = ["lsd", "--icon", "always", "--icon", "never"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconOption::Never), IconOption::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Always),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Always), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Auto),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Auto), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Never),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Never), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        c.icons = Some(Icons {
            when: Some(IconOption::Always),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Never), IconOption::from_config(&c));
    }
}

mod test_icon_theme {
    use super::*;

    #[test]
    fn test_from_cli_none() {
        let argv = ["lsd"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(None, IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_fancy() {
        let argv = ["lsd", "--icon-theme", "fancy"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconTheme::Fancy), IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_unicode() {
        let argv = ["lsd", "--icon-theme", "unicode"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_cli_icon_multi() {
        let argv = ["lsd", "--icon-theme", "fancy", "--icon-theme", "unicode"];
        let cli = Cli::try_parse_from(argv).unwrap();
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_cli(&cli));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_fancy() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: Some(IconTheme::Fancy),
            separator: None,
        });
        assert_eq!(Some(IconTheme::Fancy), IconTheme::from_config(&c));
    }

    #[test]
    fn test_from_config_unicode() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: Some(IconTheme::Unicode),
            separator: None,
        });
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_config(&c));
    }
}

mod test_icon_separator {
    use super::*;

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: None,
            separator: Some(" ".to_string()),
        });
        let expected = Some(IconSeparator(" ".to_string()));
        assert_eq!(expected, IconSeparator::from_config(&c));
    }

    #[test]
    fn test_from_config_custom() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: None,
            separator: Some(" |".to_string()),
        });
        let expected = Some(IconSeparator(" |".to_string()));
        assert_eq!(expected, IconSeparator::from_config(&c));
    }
}
