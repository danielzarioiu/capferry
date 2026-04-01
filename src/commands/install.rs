use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::ConfigStore;
use crate::wrappers::{all_wrappers, current_wrapper};

pub fn run(store: &ConfigStore) -> Result<()> {
    let cfg = store.load_or_create()?;
    let install_dir = Path::new(&cfg.install_dir);

    fs::create_dir_all(install_dir)
        .with_context(|| format!("failed to create install dir {}", install_dir.display()))?;

    for wrapper in all_wrappers(&cfg) {
        let target = install_dir.join(wrapper.name);
        fs::write(&target, wrapper.content)
            .with_context(|| format!("failed to write wrapper {}", target.display()))?;
        mark_executable(&target)?;
    }

    let current = current_wrapper(&cfg);
    let current_target = install_dir.join(current.name);
    fs::write(&current_target, current.content)
        .with_context(|| format!("failed to write wrapper {}", current_target.display()))?;
    mark_executable(&current_target)?;

    println!("Wrappers installed in {}", install_dir.display());
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
