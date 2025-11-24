use clap::Parser;

use kgls::flags::hyperlink::HyperlinkOption;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, HyperlinkOption::from_cli(&cli));
}

#[test]
fn test_from_cli_always() {
    let argv = ["lsd", "--hyperlink", "always"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(HyperlinkOption::Always),
        HyperlinkOption::from_cli(&cli)
    );
}

#[test]
fn test_from_cli_auto() {
    let argv = ["lsd", "--hyperlink", "auto"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(HyperlinkOption::Auto), HyperlinkOption::from_cli(&cli));
}

#[test]
fn test_from_cli_never() {
    let argv = ["lsd", "--hyperlink", "never"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(HyperlinkOption::Never),
        HyperlinkOption::from_cli(&cli)
    );
}

#[test]
fn test_from_cli_classic_mode() {
    let argv = ["lsd", "--hyperlink", "always", "--classic"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(HyperlinkOption::Never),
        HyperlinkOption::from_cli(&cli)
    );
}

#[test]
fn test_from_cli_hyperlink_when_multi() {
    let argv = ["lsd", "--hyperlink", "always", "--hyperlink", "never"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(HyperlinkOption::Never),
        HyperlinkOption::from_cli(&cli)
    );
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, HyperlinkOption::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_always() {
    let mut c = Config::with_none();
    c.hyperlink = Some(HyperlinkOption::Always);
    assert_eq!(
        Some(HyperlinkOption::Always),
        HyperlinkOption::from_config(&c)
    );
}

#[test]
fn test_from_config_auto() {
    let mut c = Config::with_none();
    c.hyperlink = Some(HyperlinkOption::Auto);
    assert_eq!(
        Some(HyperlinkOption::Auto),
        HyperlinkOption::from_config(&c)
    );
}

#[test]
fn test_from_config_never() {
    let mut c = Config::with_none();
    c.hyperlink = Some(HyperlinkOption::Never);
    assert_eq!(
        Some(HyperlinkOption::Never),
        HyperlinkOption::from_config(&c)
    );
}

#[test]
fn test_from_config_classic_mode() {
    let mut c = Config::with_none();
    c.classic = Some(true);
    c.hyperlink = Some(HyperlinkOption::Always);
    assert_eq!(
        Some(HyperlinkOption::Never),
        HyperlinkOption::from_config(&c)
    );
}
