use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use directories_next::BaseDirs;

use crate::config::CapferryConfig;

pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    pub fn discover() -> Result<Self> {
        let home = capferry_home_dir()?;
        Ok(Self {
            path: home.join("config.toml"),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn load_or_create(&self) -> Result<CapferryConfig> {
        if self.path.exists() {
            return self.load();
        }

        let default_install_dir = default_install_dir()?;
        let cfg = CapferryConfig::default_with_install_dir(default_install_dir);
        self.save(&cfg)?;
        Ok(cfg)
    }

    pub fn load(&self) -> Result<CapferryConfig> {
        let raw = fs::read_to_string(&self.path)
            .with_context(|| format!("failed to read config file at {}", self.path.display()))?;

        toml::from_str::<CapferryConfig>(&raw)
            .with_context(|| format!("invalid config format at {}", self.path.display()))
    }

    pub fn save(&self, config: &CapferryConfig) -> Result<()> {
        let parent = self
            .path
            .parent()
            .context("invalid config path without parent directory")?;

        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config dir {}", parent.display()))?;

        let serialized = toml::to_string_pretty(config).context("failed to serialize config")?;
        fs::write(&self.path, serialized)
            .with_context(|| format!("failed to write config file at {}", self.path.display()))
    }
}

fn default_install_dir() -> Result<String> {
    let home = BaseDirs::new()
        .context("unable to resolve user home directory")?
        .home_dir()
        .to_path_buf();

    if home.as_os_str().is_empty() {
        bail!("resolved home directory is empty");
    }

    Ok(home.join(".local").join("bin").display().to_string())
}

fn capferry_home_dir() -> Result<PathBuf> {
    let home = BaseDirs::new()
        .context("unable to resolve user home directory")?
        .home_dir()
        .to_path_buf();

    if home.as_os_str().is_empty() {
        bail!("resolved home directory is empty");
    }

    Ok(home.join(".capferry"))
}
