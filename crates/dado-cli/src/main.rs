//! `dado-cli` command-line executable.

#![forbid(unsafe_code)]

mod args;
mod handlers;
mod init;

use args::{Cli, Commands};
use clap::Parser;
use dado_core::PatternRegistry;
use handlers::*;
use init::handle_init;
use std::process::ExitCode;
use tracing_subscriber::EnvFilter;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    // One span per invocation (`GB-339`), opened here so it covers the registry setup as well as the dispatch:
    // registry initialisation returns `ExitCode::FAILURE` directly, and a span opened after it would record that
    // failure not at all. The subcommand is the operation the panel groups by.
    let command = std::env::args().nth(1).filter(|arg| !arg.starts_with('-'));
    let invocation = gaugeblock_workspace::InvocationSpan::start("dado", command.as_deref());

    let mut registry = match PatternRegistry::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing pattern registry: {e}");
            invocation.finish(false);
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = registry.load_from_dir(&cli.dir) {
        tracing::warn!(
            "Failed to load local patterns from {}: {e}",
            cli.dir.display()
        );
    }

    let result = match cli.command {
        Commands::List(ref a) => handle_list(a, &registry),
        Commands::Inspect(ref a) => handle_inspect(a, &registry),
        Commands::Render(ref a) => handle_render(a, &registry),
        Commands::Contract(ref a) => handle_contract(a),
        Commands::VerifyPack(ref a) => handle_verify_pack(a, &registry),
        Commands::Init(_) => handle_init(&cli.dir),
        Commands::Mcp(_) => dado_mcp::run_stdio_server(&cli.dir),
    };

    // Status follows the same exit code the caller sees, so the log and the shell agree on what happened.
    invocation.finish(result.is_ok());

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
