use clap::Parser;

use kgls::flags::permission::PermissionFlag;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_default() {
    let expected = if cfg!(target_os = "windows") {
        PermissionFlag::Attributes
    } else {
        PermissionFlag::Rwx
    };
    assert_eq!(expected, PermissionFlag::default());
}

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, PermissionFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_default() {
    let argv = ["lsd", "--permission", "rwx"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_short() {
    let argv = ["lsd", "--permission", "octal"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(PermissionFlag::Octal), PermissionFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_attributes() {
    let argv = ["lsd", "--permission", "attributes"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(PermissionFlag::Attributes),
        PermissionFlag::from_cli(&cli)
    );
}

#[test]
fn test_from_cli_permissions_disable() {
    let argv = ["lsd", "--permission", "disable"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(
        Some(PermissionFlag::Disable),
        PermissionFlag::from_cli(&cli)
    );
}

#[test]
#[should_panic]
fn test_from_cli_unknown() {
    let argv = ["lsd", "--permission", "unknown"];
    let _ = Cli::try_parse_from(argv).unwrap();
}
#[test]
fn test_from_cli_permissions_multi() {
    let argv = ["lsd", "--permission", "octal", "--permission", "rwx"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_permissions_classic() {
    let argv = ["lsd", "--permission", "rwx", "--classic"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, PermissionFlag::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_rwx() {
    let mut c = Config::with_none();
    c.permission = Some(PermissionFlag::Rwx);
    assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
}

#[test]
fn test_from_config_octal() {
    let mut c = Config::with_none();
    c.permission = Some(PermissionFlag::Octal);
    assert_eq!(Some(PermissionFlag::Octal), PermissionFlag::from_config(&c));
}

#[test]
fn test_from_config_classic_mode() {
    let mut c = Config::with_none();
    c.classic = Some(true);
    assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
}
