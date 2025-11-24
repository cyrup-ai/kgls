use clap::Parser;

use kgls::flags::symlinks::NoSymlink;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_none");
    assert_eq!(None, NoSymlink::from_cli(&cli));
}

#[test]
fn test_from_cli_true() {
    let argv = ["lsd", "--no-symlink"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_true");
    assert_eq!(Some(NoSymlink(true)), NoSymlink::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, NoSymlink::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_true() {
    let mut c = Config::with_none();
    c.no_symlink = Some(true);
    assert_eq!(Some(NoSymlink(true)), NoSymlink::from_config(&c));
}

#[test]
fn test_from_config_false() {
    let mut c = Config::with_none();
    c.no_symlink = Some(false);
    assert_eq!(Some(NoSymlink(false)), NoSymlink::from_config(&c));
}
