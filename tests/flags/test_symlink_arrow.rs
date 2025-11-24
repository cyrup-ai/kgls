use clap::Parser;
use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::{Configurable, SymlinkArrow};

#[test]
fn test_symlink_arrow_from_config_utf8() {
    let mut c = Config::with_none();
    c.symlink_arrow = Some("↹".into());
    assert_eq!(
        Some(SymlinkArrow(String::from("\u{21B9}"))),
        SymlinkArrow::from_config(&c)
    );
}

#[test]
fn test_symlink_arrow_from_args_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, SymlinkArrow::from_cli(&cli));
}

#[test]
fn test_symlink_arrow_default() {
    assert_eq!(
        SymlinkArrow(String::from("\u{21d2}")),
        SymlinkArrow::default()
    );
}

#[test]
fn test_symlink_display() {
    assert_eq!("⇒", format!("{}", SymlinkArrow::default()));
}
