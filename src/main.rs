mod cli;
mod commands;
mod config;
mod providers;
mod wrappers;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("Error: {err:#}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    commands::run(cli.command)
}
