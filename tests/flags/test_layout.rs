use clap::Parser;

use kgls::flags::layout::Layout;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, Layout::from_cli(&cli));
}

#[test]
fn test_from_cli_tree() {
    let argv = ["lsd", "--tree"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Layout::Tree), Layout::from_cli(&cli));
}

#[test]
fn test_from_cli_oneline() {
    let argv = ["lsd", "--oneline"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Layout::OneLine), Layout::from_cli(&cli));
}

#[test]
fn test_from_cli_oneline_through_long() {
    let argv = ["lsd", "--long"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Layout::OneLine), Layout::from_cli(&cli));
}

#[test]
fn test_from_cli_oneline_through_blocks() {
    let argv = ["lsd", "--blocks", "permission,name"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(Layout::OneLine), Layout::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, Layout::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_tree() {
    let mut c = Config::with_none();
    c.layout = Some(Layout::Tree);
    assert_eq!(Some(Layout::Tree), Layout::from_config(&c));
}

#[test]
fn test_from_config_oneline() {
    let mut c = Config::with_none();
    c.layout = Some(Layout::OneLine);
    assert_eq!(Some(Layout::OneLine), Layout::from_config(&c));
}

#[test]
fn test_from_config_grid() {
    let mut c = Config::with_none();
    c.layout = Some(Layout::Grid);
    assert_eq!(Some(Layout::Grid), Layout::from_config(&c));
}
