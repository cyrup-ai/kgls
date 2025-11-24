use clap::error::ErrorKind;
use clap::Parser;
use kgls::flags::Recursion;
use kgls::app::Cli;
use kgls::config_file::{self, Config};

#[test]
fn test_enabled_from_cli_empty() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_enabled_from_cli_empty");
    assert_eq!(None, Recursion::enabled_from_cli(&cli));
}

#[test]
fn test_enabled_from_cli_true() {
    let argv = ["lsd", "--recursive"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_enabled_from_cli_true");
    assert_eq!(Some(true), Recursion::enabled_from_cli(&cli));
}

#[test]
fn test_enabled_from_empty_matches_and_config() {
    let argv = ["lsd"];
    assert!(!Recursion::enabled_from(
        &Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_enabled_from_empty_matches_and_config"),
        &Config::with_none()
    ));
}

#[test]
fn test_enabled_from_matches_empty_and_config_true() {
    let argv = ["lsd"];
    let mut c = Config::with_none();
    c.recursion = Some(config_file::Recursion {
        enabled: Some(true),
        depth: None,
    });
    assert!(Recursion::enabled_from(
        &Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_enabled_from_matches_empty_and_config_true"),
        &c
    ));
}

#[test]
fn test_enabled_from_matches_empty_and_config_false() {
    let argv = ["lsd"];
    let mut c = Config::with_none();
    c.recursion = Some(config_file::Recursion {
        enabled: Some(false),
        depth: None,
    });
    assert!(!Recursion::enabled_from(
        &Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_enabled_from_matches_empty_and_config_false"),
        &c
    ));
}

// The following depth_from_cli tests are implemented using match expressions instead
// of the assert_eq macro, because clap::Error does not implement PartialEq.

#[test]
fn test_depth_from_cli_empty() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_depth_from_cli_empty");
    assert!(cli.depth.is_none());
}

#[test]
fn test_depth_from_cli_integer() {
    let argv = ["lsd", "--depth", "42"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_depth_from_cli_integer");
    assert!(matches!(cli.depth, Some(42)));
}

#[test]
fn test_depth_from_cli_depth_multi() {
    let argv = ["lsd", "--depth", "4", "--depth", "2"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_depth_from_cli_depth_multi");
    assert!(matches!(cli.depth, Some(2)));
}

#[test]
fn test_depth_from_cli_neg_int() {
    let argv = ["lsd", "--depth", "\\-42"];
    let cli = Cli::try_parse_from(argv);
    assert!(matches!(cli, Err(e) if e.kind() == ErrorKind::ValueValidation));
}

#[test]
fn test_depth_from_cli_non_int() {
    let argv = ["lsd", "--depth", "foo"];
    let cli = Cli::try_parse_from(argv);
    assert!(matches!(cli, Err(e) if e.kind() == ErrorKind::ValueValidation));
}

#[test]
fn test_depth_from_config_none_max() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_depth_from_config_none_max");
    assert_eq!(
        usize::MAX,
        Recursion::depth_from(&cli, &Config::with_none())
    );
}

#[test]
fn test_depth_from_config_pos_integer() {
    let argv = ["lsd"];
    let mut c = Config::with_none();
    c.recursion = Some(config_file::Recursion {
        enabled: None,
        depth: Some(42),
    });
    assert_eq!(
        42,
        Recursion::depth_from(&Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_depth_from_config_pos_integer"), &c)
    );
}
