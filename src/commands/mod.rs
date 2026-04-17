mod configure;
mod doctor;
mod install;
mod status;
mod use_provider;

use anyhow::Result;

use crate::cli::{Commands, ConfigureTarget};
use crate::config::ConfigStore;

pub fn run(command: Commands) -> Result<()> {
    let store = ConfigStore::discover()?;

    match command {
        Commands::Status => status::run(&store),
        Commands::Install => install::run(&store),
        Commands::Use { provider } => use_provider::run(&store, provider),
        Commands::Configure { target } => match target {
            ConfigureTarget::Bedrock => configure::run_bedrock(&store),
            ConfigureTarget::Zai => configure::run_zai(&store),
        },
        Commands::Doctor => doctor::run(&store),
    }
}
