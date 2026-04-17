use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::config::ConfigStore;

pub fn run(store: &ConfigStore) -> Result<()> {
    let cfg = store.load_or_create()?;
    let mut checks = Vec::new();

    checks.push(check_claude_binary(&cfg.claude_path));
    checks.push(check_install_dir_in_path(&cfg.install_dir));
    checks.extend(check_wrappers_exist(&cfg.install_dir));
    checks.push(check_active_provider_config(&cfg));

    println!("capferry doctor");
    for check in &checks {
        println!(
            "[{}] {}: {}",
            check.status.as_str(),
            check.name,
            check.detail
        );
    }

    let has_error = checks.iter().any(|c| matches!(c.status, Status::Error));
    if has_error {
        println!("Result: ERROR");
    } else {
        let has_warn = checks.iter().any(|c| matches!(c.status, Status::Warn));
        if has_warn {
            println!("Result: WARN");
        } else {
            println!("Result: OK");
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Status {
    Ok,
    Warn,
    Error,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

#[derive(Debug)]
struct Check {
    status: Status,
    name: &'static str,
    detail: String,
}

fn check_claude_binary(claude_path: &str) -> Check {
    match find_executable(claude_path) {
        Some(path) => Check {
            status: Status::Ok,
            name: "claude binary",
            detail: format!("found at {}", path.display()),
        },
        None => Check {
            status: Status::Error,
            name: "claude binary",
            detail: format!("not found for path '{claude_path}'"),
        },
    }
}

fn check_install_dir_in_path(install_dir: &str) -> Check {
    let install = Path::new(install_dir);
    let path_env = env::var_os("PATH").unwrap_or_default();
    let in_path = env::split_paths(&path_env).any(|p| p == install);
    if in_path {
        Check {
            status: Status::Ok,
            name: "PATH",
            detail: format!("{install_dir} is present in PATH"),
        }
    } else {
        Check {
            status: Status::Warn,
            name: "PATH",
            detail: format!("{install_dir} is not present in PATH"),
        }
    }
}

fn check_wrappers_exist(install_dir: &str) -> Vec<Check> {
    let names = [
        "claude-sub",
        "claude-bedrock",
        "claude-zai",
        "claude-current",
    ];
    names
        .iter()
        .map(|name| {
            let path = Path::new(install_dir).join(name);
            if path.exists() {
                Check {
                    status: Status::Ok,
                    name: "wrapper",
                    detail: format!("{name} present at {}", path.display()),
                }
            } else {
                Check {
                    status: Status::Error,
                    name: "wrapper",
                    detail: format!("{name} missing at {}", path.display()),
                }
            }
        })
        .collect()
}

fn check_active_provider_config(cfg: &crate::config::CapferryConfig) -> Check {
    if matches!(cfg.active_provider, crate::providers::Provider::Bedrock)
        && cfg.bedrock.aws_region.as_deref().is_none_or(str::is_empty)
        && !aws_default_region_available()
    {
        return Check {
            status: Status::Warn,
            name: "active provider config",
            detail: "bedrock.aws_region is not set and no AWS default region detected (env or ~/.aws/config)"
                .to_owned(),
        };
    }

    let errors = cfg.active_provider_errors();
    if errors.is_empty() {
        Check {
            status: Status::Ok,
            name: "active provider config",
            detail: format!("{} configuration is coherent", cfg.active_provider),
        }
    } else {
        Check {
            status: Status::Error,
            name: "active provider config",
            detail: errors.join("; "),
        }
    }
}

fn aws_default_region_available() -> bool {
    if env::var("AWS_REGION")
        .ok()
        .is_some_and(|v| !v.trim().is_empty())
        || env::var("AWS_DEFAULT_REGION")
            .ok()
            .is_some_and(|v| !v.trim().is_empty())
    {
        return true;
    }

    let home = match directories_next::BaseDirs::new() {
        Some(base) => base.home_dir().to_path_buf(),
        None => return false,
    };
    let config_path = home.join(".aws").join("config");
    let raw = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let mut in_default = false;
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_default = matches!(trimmed, "[default]" | "[profile default]");
            continue;
        }
        if in_default && trimmed.starts_with("region") && trimmed.contains('=') {
            let region = trimmed
                .split_once('=')
                .map(|(_, value)| value.trim())
                .unwrap_or("");
            if !region.is_empty() {
                return true;
            }
        }
    }
    false
}

fn find_executable(binary: &str) -> Option<PathBuf> {
    let as_path = Path::new(binary);
    if as_path.components().count() > 1 || binary.starts_with('.') {
        return is_executable_file(as_path).then(|| as_path.to_path_buf());
    }

    let path_env = env::var_os("PATH")?;
    for dir in env::split_paths(&path_env) {
        let candidate = dir.join(binary);
        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return path
            .metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false);
    }
    #[cfg(not(unix))]
    {
        true
    }
}
