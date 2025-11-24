use clap::Parser;

use kgls::flags::header::Header;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_none");
    assert_eq!(None, Header::from_cli(&cli));
}

#[test]
fn test_from_cli_true() {
    let argv = ["lsd", "--header"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_true");
    assert_eq!(Some(Header(true)), Header::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, Header::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_true() {
    let mut c = Config::with_none();
    c.header = Some(true);
    assert_eq!(Some(Header(true)), Header::from_config(&c));
}

#[test]
fn test_from_config_false() {
    let mut c = Config::with_none();
    c.header = Some(false);
    assert_eq!(Some(Header(false)), Header::from_config(&c));
}
