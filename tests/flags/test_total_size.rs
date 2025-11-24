use clap::Parser;

use kgls::flags::total_size::TotalSize;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, TotalSize::from_cli(&cli));
}

#[test]
fn test_from_cli_true() {
    let argv = ["lsd", "--total-size"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(TotalSize(true)), TotalSize::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, TotalSize::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_true() {
    let mut c = Config::with_none();
    c.total_size = Some(true);
    assert_eq!(Some(TotalSize(true)), TotalSize::from_config(&c));
}

#[test]
fn test_from_config_false() {
    let mut c = Config::with_none();
    c.total_size = Some(false);
    assert_eq!(Some(TotalSize(false)), TotalSize::from_config(&c));
}
