use clap::Parser;

use kgls::flags::indicators::Indicators;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, Indicators::from_cli(&cli));
}

#[test]
fn test_from_cli_true() {
    let argv = ["lsd", "--classify"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Indicators(true)), Indicators::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, Indicators::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_true() {
    let mut c = Config::with_none();
    c.indicators = Some(true);
    assert_eq!(Some(Indicators(true)), Indicators::from_config(&c));
}

#[test]
fn test_from_config_false() {
    let mut c = Config::with_none();
    c.indicators = Some(false);
    assert_eq!(Some(Indicators(false)), Indicators::from_config(&c));
}
