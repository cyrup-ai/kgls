use clap::Parser;

use kgls::flags::size::SizeFlag;

use kgls::app::Cli;
use kgls::config_file::Config;
use kgls::flags::Configurable;

#[test]
fn test_default() {
    assert_eq!(SizeFlag::Default, SizeFlag::default());
}

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(None, SizeFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_default() {
    let argv = ["lsd", "--size", "default"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(SizeFlag::Default), SizeFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_short() {
    let argv = ["lsd", "--size", "short"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(SizeFlag::Short), SizeFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_bytes() {
    let argv = ["lsd", "--size", "bytes"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_cli(&cli));
}

#[test]
#[should_panic]
fn test_from_cli_unknown() {
    let argv = ["lsd", "--size", "unknown"];
    let _ = Cli::try_parse_from(argv).unwrap();
}
#[test]
fn test_from_cli_size_multi() {
    let argv = ["lsd", "--size", "bytes", "--size", "short"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(SizeFlag::Short), SizeFlag::from_cli(&cli));
}

#[test]
fn test_from_cli_size_classic() {
    let argv = ["lsd", "--size", "short", "--classic"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_cli(&cli));
}

#[test]
fn test_from_config_none() {
    assert_eq!(None, SizeFlag::from_config(&Config::with_none()));
}

#[test]
fn test_from_config_default() {
    let mut c = Config::with_none();
    c.size = Some(SizeFlag::Default);
    assert_eq!(Some(SizeFlag::Default), SizeFlag::from_config(&c));
}

#[test]
fn test_from_config_short() {
    let mut c = Config::with_none();
    c.size = Some(SizeFlag::Short);
    assert_eq!(Some(SizeFlag::Short), SizeFlag::from_config(&c));
}

#[test]
fn test_from_config_bytes() {
    let mut c = Config::with_none();
    c.size = Some(SizeFlag::Bytes);
    assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_config(&c));
}

#[test]
fn test_from_config_classic_mode() {
    let mut c = Config::with_none();
    c.classic = Some(true);
    assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_config(&c));
}
