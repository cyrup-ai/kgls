use clap::Parser;
use kgls::{config_file::Config, core::Core, flags::Flags, Cli, ExitCode};

fn main() {
    // Initialize logging infrastructure
    env_logger::init();
    
    let cli = Cli::parse_from(wild::args_os());

    let config = if cli.ignore_config {
        Config::with_none()
    } else if let Some(path) = &cli.config_file {
        Config::from_file(path).unwrap_or_else(|| {
            log::error!("Invalid config file path '{}'", path.display());
            eprintln!("kgls: invalid config file path '{}'", path.display());
            std::process::exit(ExitCode::MajorIssue as i32);
        })
    } else {
        Config::default()
    };
    let flags = Flags::configure_from(&cli, &config).unwrap_or_else(|err| err.exit());
    let core = Core::new(flags);

    let exit_code = tokio::runtime::Runtime::new()
        .expect("Failed to create async runtime")
        .block_on(core.run(cli.inputs));
    std::process::exit(exit_code as i32);
}
