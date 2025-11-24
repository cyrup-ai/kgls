use clap::Parser;
use kgls::IgnoreGlobs;
use kgls::app::Cli;
use kgls::config_file::Config;

// The following tests are implemented using match expressions instead of the assert_eq macro,
// because clap::Error does not implement PartialEq.
//
// Further no tests for actually returned GlobSets are implemented, because GlobSet does not
// even implement PartialEq and thus can not be easily compared.

#[test]
fn test_configuration_from_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert!(matches!(
        IgnoreGlobs::configure_from(&cli, &Config::with_none()),
        Ok(..)
    ));
}

#[test]
fn test_configuration_from_args() {
    let argv = ["lsd", "--ignore-glob", ".git"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert!(matches!(
        IgnoreGlobs::configure_from(&cli, &Config::with_none()),
        Ok(..)
    ));
}

#[test]
fn test_configuration_from_config() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    let mut c = Config::with_none();
    c.ignore_globs = Some(vec![".git".into()]);
    assert!(matches!(IgnoreGlobs::configure_from(&cli, &c), Ok(..)));
}

#[test]
fn test_from_cli_none() {
    let argv = ["lsd"];
    let cli = Cli::try_parse_from(argv).unwrap();
    assert!(IgnoreGlobs::from_cli(&cli).is_none());
}

#[test]
fn test_from_config_none() {
    assert!(IgnoreGlobs::from_config(&Config::with_none()).is_none());
}

#[test]
fn test_pattern_classification() {
    use std::ffi::OsStr;

    let globs = IgnoreGlobs::default();

    // Test extension matching (should hit fast path)
    assert!(globs.is_match(OsStr::new("test.jpg")));
    assert!(globs.is_match(OsStr::new("file.PNG"))); // Case insensitive
    assert!(globs.is_match(OsStr::new("archive.tar.gz")));

    // Test exact name matching (should hit fast path)
    assert!(globs.is_match(OsStr::new(".git")));
    assert!(globs.is_match(OsStr::new("node_modules")));
    assert!(globs.is_match(OsStr::new("target")));

    // Test files that should NOT match
    assert!(!globs.is_match(OsStr::new("README.md")));
    assert!(!globs.is_match(OsStr::new("src")));
    assert!(!globs.is_match(OsStr::new("main.rs")));
}

#[test]
#[ignore] // Run with: cargo test test_performance_comparison -- --ignored --nocapture
fn test_performance_comparison() {
    use std::ffi::OsStr;
    use std::time::Instant;

    let globs = IgnoreGlobs::default();

    // Test files that hit different code paths
    let test_files = vec![
        OsStr::new("file.jpg"),      // Extension fast path
        OsStr::new("image.PNG"),      // Extension fast path (case insensitive)
        OsStr::new(".git"),           // Exact name fast path
        OsStr::new("node_modules"),   // Exact name fast path
        OsStr::new("test.rs"),        // No match (goes through all paths)
        OsStr::new("README.md"),      // No match
    ];

    let iterations = 100_000;
    let start = Instant::now();

    for _ in 0..iterations {
        for file in &test_files {
            let _ = globs.is_match(file);
        }
    }

    let duration = start.elapsed();
    let total_ops = iterations * test_files.len() as u128;
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();

    println!("\nPerformance Results:");
    println!("  Total operations: {}", total_ops);
    println!("  Total time: {:?}", duration);
    println!("  Operations/sec: {:.0}", ops_per_sec);
    println!("  Avg time per operation: {:.2?}", duration / total_ops as u32);

    // With the optimization, we expect > 5M ops/sec on modern hardware
    // Old implementation with 147 glob patterns would be much slower
    assert!(ops_per_sec > 1_000_000.0,
        "Performance too slow: {} ops/sec", ops_per_sec);
}
