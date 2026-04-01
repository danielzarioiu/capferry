use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::ConfigStore;
use crate::providers::Provider;
use crate::wrappers::current_wrapper;

pub fn run(store: &ConfigStore, provider: Provider) -> Result<()> {
    let mut cfg = store.load_or_create()?;
    cfg.active_provider = provider;
    store.save(&cfg)?;

    let install_dir = Path::new(&cfg.install_dir);
    fs::create_dir_all(install_dir)
        .with_context(|| format!("failed to create install dir {}", install_dir.display()))?;

    let current = current_wrapper(&cfg);
    let current_target = install_dir.join(current.name);
    fs::write(&current_target, current.content)
        .with_context(|| format!("failed to write wrapper {}", current_target.display()))?;
    mark_executable(&current_target)?;

    println!("Active provider set to {}", provider);
    println!("Updated {}", current_target.display());
    Ok(())
}

#[cfg(unix)]
fn mark_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).with_context(|| {
        format!(
            "failed to set executable permissions for {}",
            path.display()
        )
    })
}

#[cfg(not(unix))]
fn mark_executable(_path: &Path) -> Result<()> {
    Ok(())
}
