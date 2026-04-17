use anyhow::Result;

use crate::config::ConfigStore;
use crate::wrappers::install_all;

pub fn run(store: &ConfigStore) -> Result<()> {
    let cfg = store.load_or_create()?;
    install_all(&cfg)?;

    println!("Wrappers installed in {}", cfg.install_dir);
    Ok(())
}
