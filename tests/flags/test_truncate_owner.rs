use clap::Parser;

use kgls::flags::truncate_owner::TruncateOwner;

use kgls::app::Cli;
use kgls::config_file::{self, Config};
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, TruncateOwner::from_cli(&cli));
}

#[test]
fn test_from_cli_after_some() {
    let argv = ["lsd", "--truncate-owner-after", "1"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(TruncateOwner {
            after: Some(1),
            marker: None,
        }),
        TruncateOwner::from_cli(&cli)
    );
}

#[test]
fn test_from_cli_marker_some() {
    let argv = ["lsd", "--truncate-owner-marker", "…"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(TruncateOwner {
            after: None,
            marker: Some("…".to_string()),
        }),
        TruncateOwner::from_cli(&cli)
    );
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, TruncateOwner::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_all_fields_none() {
    let mut c = Config::with_none();
    c.truncate_owner = Some(config_file::TruncateOwner {
        after: None,
        marker: None,
    });
    assert_eq!(
        Some(TruncateOwner {
            after: None,
            marker: None,
        }),
        TruncateOwner::from_config(&c)
    );
}

#[test]
fn test_from_config_all_fields_some() {
    let mut c = Config::with_none();
    c.truncate_owner = Some(config_file::TruncateOwner {
        after: Some(1),
        marker: Some(">".to_string()),
    });
    assert_eq!(
        Some(TruncateOwner {
            after: Some(1),
            marker: Some(">".to_string()),
        }),
        TruncateOwner::from_config(&c)
    );
}
