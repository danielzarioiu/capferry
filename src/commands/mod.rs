mod install;
mod status;
mod use_provider;

use anyhow::Result;

use crate::cli::Commands;
use crate::config::ConfigStore;

pub fn run(command: Commands) -> Result<()> {
    let store = ConfigStore::discover()?;

    match command {
        Commands::Status => status::run(&store),
        Commands::Install => install::run(&store),
        Commands::Use { provider } => use_provider::run(&store, provider),
    }
}
