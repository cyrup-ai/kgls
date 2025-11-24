use clap::Parser;

use kgls::flags::dereference::Dereference;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, Dereference::from_cli(&cli));
}

#[test]
fn test_from_cli_true() {
    let argv = ["lsd", "--dereference"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Dereference(true)), Dereference::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, Dereference::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_true() {
    let mut c = Config::with_none();
    c.dereference = Some(true);
    assert_eq!(Some(Dereference(true)), Dereference::from_config(&c));
}

#[test]
fn test_from_config_false() {
    let mut c = Config::with_none();
    c.dereference = Some(false);
    assert_eq!(Some(Dereference(false)), Dereference::from_config(&c));
}
