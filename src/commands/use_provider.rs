use anyhow::Result;

use crate::config::ConfigStore;
use crate::providers::Provider;
use crate::wrappers::install_current;

pub fn run(store: &ConfigStore, provider: Provider) -> Result<()> {
    let mut cfg = store.load_or_create()?;
    cfg.active_provider = provider;
    store.save(&cfg)?;

    let current_target = install_current(&cfg)?;

    println!("Active provider set to {}", provider);
    println!("Updated {}", current_target.display());
    Ok(())
}
