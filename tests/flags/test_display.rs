use clap::Parser;

use kgls::flags::display::Display;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_none");
    assert_eq!(None, Display::from_cli(&cli));
}

#[test]
fn test_from_cli_system_protected() {
    let argv = ["lsd", "--system-protected"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_system_protected");
    #[cfg(windows)]
    assert_eq!(Some(Display::SystemProtected), Display::from_cli(&cli));

    #[cfg(not(windows))]
    assert_eq!(Some(Display::All), Display::from_cli(&cli));
}

#[test]
fn test_from_cli_all() {
    let argv = ["lsd", "--all"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_all");
    assert_eq!(Some(Display::All), Display::from_cli(&cli));
}

#[test]
fn test_from_cli_almost_all() {
    let argv = ["lsd", "--almost-all"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_almost_all");
    assert_eq!(Some(Display::AlmostAll), Display::from_cli(&cli));
}

#[test]
fn test_from_cli_directory_only() {
    let argv = ["lsd", "--directory-only"];
    let cli = Cli::try_parse_from(argv).expect("Failed to parse CLI arguments for test_from_cli_directory_only");
    assert_eq!(Some(Display::DirectoryOnly), Display::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, Display::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_all() {
    let mut c = Config::with_none();
    c.display = Some(Display::All);
    assert_eq!(Some(Display::All), Display::from_config(&c));
}

#[test]
fn test_from_config_almost_all() {
    let mut c = Config::with_none();
    c.display = Some(Display::AlmostAll);
    assert_eq!(Some(Display::AlmostAll), Display::from_config(&c));
}

#[test]
fn test_from_config_directory_only() {
    let mut c = Config::with_none();
    c.display = Some(Display::DirectoryOnly);
    assert_eq!(Some(Display::DirectoryOnly), Display::from_config(&c));
}

#[test]
fn test_from_config_visible_only() {
    let mut c = Config::with_none();
    c.display = Some(Display::VisibleOnly);
    assert_eq!(Some(Display::VisibleOnly), Display::from_config(&c));
}
