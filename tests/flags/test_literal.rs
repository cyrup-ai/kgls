use clap::Parser;

use kgls::flags::literal::Literal;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, Literal::from_cli(&cli));
}

#[test]
fn test_from_cli_literal() {
    let argv = ["lsd", "--literal"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Literal(true)), Literal::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, Literal::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_true() {
    let mut c = Config::with_none();
    c.literal = Some(true);
    assert_eq!(Some(Literal(true)), Literal::from_config(&c));
}

#[test]
fn test_from_config_false() {
    let mut c = Config::with_none();
    c.literal = Some(false);
    assert_eq!(Some(Literal(false)), Literal::from_config(&c));
}
