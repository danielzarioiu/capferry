use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::CapferryConfig;

pub struct WrapperScript {
    pub name: &'static str,
    pub content: String,
}

pub fn all_wrappers(cfg: &CapferryConfig) -> Vec<WrapperScript> {
    vec![
        WrapperScript {
            name: "claude-sub",
            content: subscription_wrapper(cfg),
        },
        WrapperScript {
            name: "claude-bedrock",
            content: bedrock_wrapper(cfg),
        },
        WrapperScript {
            name: "claude-zai",
            content: zai_wrapper(cfg),
        },
    ]
}

pub fn current_wrapper(cfg: &CapferryConfig) -> WrapperScript {
    let target = cfg.active_provider.wrapper_name();
    let content = format!(
        r#"#!/usr/bin/env sh
set -eu
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
exec "$DIR/{target}" "$@"
"#
    );

    WrapperScript {
        name: "claude-current",
        content,
    }
}

pub fn install_all(cfg: &CapferryConfig) -> Result<Vec<PathBuf>> {
    let install_dir = Path::new(&cfg.install_dir);
    fs::create_dir_all(install_dir)
        .with_context(|| format!("failed to create install dir {}", install_dir.display()))?;

    let mut written = Vec::new();
    for wrapper in all_wrappers(cfg) {
        let target = install_dir.join(wrapper.name);
        fs::write(&target, wrapper.content)
            .with_context(|| format!("failed to write wrapper {}", target.display()))?;
        mark_executable(&target)?;
        written.push(target);
    }

    let target = write_current(cfg)?;
    written.push(target);
    Ok(written)
}

pub fn install_current(cfg: &CapferryConfig) -> Result<PathBuf> {
    let install_dir = Path::new(&cfg.install_dir);
    fs::create_dir_all(install_dir)
        .with_context(|| format!("failed to create install dir {}", install_dir.display()))?;
    write_current(cfg)
}

fn write_current(cfg: &CapferryConfig) -> Result<PathBuf> {
    let install_dir = Path::new(&cfg.install_dir);
    let current = current_wrapper(cfg);
    let current_target = install_dir.join(current.name);
    fs::write(&current_target, current.content)
        .with_context(|| format!("failed to write wrapper {}", current_target.display()))?;
    mark_executable(&current_target)?;
    Ok(current_target)
}

fn subscription_wrapper(cfg: &CapferryConfig) -> String {
    let claude = shell_single_quote(&cfg.claude_path);

    format!(
        r#"#!/usr/bin/env sh
set -eu
unset AWS_REGION AWS_PROFILE AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_SESSION_TOKEN
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN ANTHROPIC_MODEL
unset ZAI_API_KEY ZAI_BASE_URL ZAI_MODEL
export CLAUDE_CODE_PROVIDER=subscription
exec {claude} "$@"
"#
    )
}

fn bedrock_wrapper(cfg: &CapferryConfig) -> String {
    let claude = shell_single_quote(&cfg.claude_path);
    let mut lines = vec![
        "#!/usr/bin/env sh".to_owned(),
        "set -eu".to_owned(),
        "unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN ANTHROPIC_MODEL".to_owned(),
        "unset ZAI_API_KEY ZAI_BASE_URL ZAI_MODEL".to_owned(),
        "export CLAUDE_CODE_PROVIDER=bedrock".to_owned(),
    ];

    if let Some(region) = &cfg.bedrock.aws_region {
        lines.push(format!(
            "export AWS_REGION={}",
            shell_single_quote(region.as_str())
        ));
    }

    if let Some(profile) = &cfg.bedrock.aws_profile {
        lines.push(format!(
            "export AWS_PROFILE={}",
            shell_single_quote(profile.as_str())
        ));
    }

    if let Some(model_id) = &cfg.bedrock.sonnet_model {
        lines.push(format!(
            "export BEDROCK_SONNET_MODEL={}",
            shell_single_quote(model_id.as_str())
        ));
        lines.push(format!(
            "export ANTHROPIC_MODEL={}",
            shell_single_quote(model_id.as_str())
        ));
    }

    if let Some(model_id) = &cfg.bedrock.opus_model {
        lines.push(format!(
            "export BEDROCK_OPUS_MODEL={}",
            shell_single_quote(model_id.as_str())
        ));
    }

    if let Some(model_id) = &cfg.bedrock.haiku_model {
        lines.push(format!(
            "export BEDROCK_HAIKU_MODEL={}",
            shell_single_quote(model_id.as_str())
        ));
    }

    lines.push(format!("exec {claude} \"$@\""));
    format!("{}\n", lines.join("\n"))
}

fn zai_wrapper(cfg: &CapferryConfig) -> String {
    let claude = shell_single_quote(&cfg.claude_path);
    let mut lines = vec![
        "#!/usr/bin/env sh".to_owned(),
        "set -eu".to_owned(),
        "unset AWS_REGION AWS_PROFILE AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_SESSION_TOKEN"
            .to_owned(),
        "export CLAUDE_CODE_PROVIDER=zai".to_owned(),
    ];

    if let Some(api_key) = &cfg.zai.auth_token {
        lines.push(format!(
            "export ZAI_API_KEY={}",
            shell_single_quote(api_key.as_str())
        ));
        lines.push(format!(
            "export ANTHROPIC_AUTH_TOKEN={}",
            shell_single_quote(api_key.as_str())
        ));
    }

    if let Some(base_url) = &cfg.zai.base_url {
        lines.push(format!(
            "export ZAI_BASE_URL={}",
            shell_single_quote(base_url.as_str())
        ));
        lines.push(format!(
            "export ANTHROPIC_BASE_URL={}",
            shell_single_quote(base_url.as_str())
        ));
    }

    if let Some(model) = &cfg.zai.sonnet_model {
        lines.push(format!(
            "export ZAI_SONNET_MODEL={}",
            shell_single_quote(model.as_str())
        ));
        lines.push(format!(
            "export ANTHROPIC_MODEL={}",
            shell_single_quote(model.as_str())
        ));
    }

    if let Some(model) = &cfg.zai.opus_model {
        lines.push(format!(
            "export ZAI_OPUS_MODEL={}",
            shell_single_quote(model.as_str())
        ));
    }

    if let Some(model) = &cfg.zai.haiku_model {
        lines.push(format!(
            "export ZAI_HAIKU_MODEL={}",
            shell_single_quote(model.as_str())
        ));
    }

    lines.push(format!("exec {claude} \"$@\""));
    format!("{}\n", lines.join("\n"))
}

fn shell_single_quote(value: &str) -> String {
    let escaped = value.replace('\'', "'\"'\"'");
    format!("'{escaped}'")
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
