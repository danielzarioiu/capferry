use clap::{Parser, Subcommand};

use crate::providers::Provider;

#[derive(Debug, Parser)]
#[command(
    name = "capferry",
    version,
    about = "Provider fallback wrapper manager for Claude Code"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Status,
    Install,
    Use {
        provider: Provider,
    },
    Configure {
        #[command(subcommand)]
        target: ConfigureTarget,
    },
    Doctor,
}

#[derive(Debug, Subcommand)]
pub enum ConfigureTarget {
    Bedrock,
    Zai,
}
