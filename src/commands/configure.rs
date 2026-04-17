use std::io::{self, Write};

use anyhow::{Result, bail};

use crate::config::{ConfigStore, validate_bedrock, validate_zai};
use crate::wrappers::install_all;

pub fn run_bedrock(store: &ConfigStore) -> Result<()> {
    let mut cfg = store.load_or_create()?;

    println!("Configure Bedrock");
    cfg.bedrock.aws_profile = prompt_optional(
        "AWS profile (optional, empty to keep current, '-' to use AWS default)",
        cfg.bedrock.aws_profile.as_deref(),
    )?;
    cfg.bedrock.aws_region = prompt_optional(
        "AWS region (optional, empty to keep current, '-' to use AWS default)",
        cfg.bedrock.aws_region.as_deref(),
    )?;
    cfg.bedrock.sonnet_model = prompt_optional(
        "Sonnet model (optional, '-' to clear)",
        cfg.bedrock.sonnet_model.as_deref(),
    )?;
    cfg.bedrock.opus_model = prompt_optional(
        "Opus model (optional, '-' to clear)",
        cfg.bedrock.opus_model.as_deref(),
    )?;
    cfg.bedrock.haiku_model = prompt_optional(
        "Haiku model (optional, '-' to clear)",
        cfg.bedrock.haiku_model.as_deref(),
    )?;

    let errors = validate_bedrock(&cfg.bedrock);
    if !errors.is_empty() {
        bail!("invalid Bedrock configuration:\n- {}", errors.join("\n- "));
    }

    store.save(&cfg)?;
    install_all(&cfg)?;
    println!("Bedrock configuration saved and wrappers regenerated.");
    Ok(())
}

pub fn run_zai(store: &ConfigStore) -> Result<()> {
    let mut cfg = store.load_or_create()?;

    println!("Configure Z.AI");
    cfg.zai.base_url = Some(prompt_required("Base URL", cfg.zai.base_url.as_deref())?);
    cfg.zai.auth_token = Some(prompt_required(
        "Auth token (stored in config for now; keychain later)",
        cfg.zai.auth_token.as_deref(),
    )?);
    cfg.zai.sonnet_model = Some(prompt_required(
        "Sonnet model",
        cfg.zai.sonnet_model.as_deref(),
    )?);
    cfg.zai.opus_model = Some(prompt_required(
        "Opus model",
        cfg.zai.opus_model.as_deref(),
    )?);
    cfg.zai.haiku_model = Some(prompt_required(
        "Haiku model",
        cfg.zai.haiku_model.as_deref(),
    )?);

    let errors = validate_zai(&cfg.zai);
    if !errors.is_empty() {
        bail!("invalid Z.AI configuration:\n- {}", errors.join("\n- "));
    }

    store.save(&cfg)?;
    install_all(&cfg)?;
    println!("Z.AI configuration saved and wrappers regenerated.");
    Ok(())
}

fn prompt_required(label: &str, current: Option<&str>) -> Result<String> {
    loop {
        let raw = prompt(label, current)?;
        if raw.trim().is_empty() {
            if let Some(existing) = current {
                if !existing.trim().is_empty() {
                    return Ok(existing.to_owned());
                }
            }
            println!("Value is required.");
            continue;
        }
        return Ok(raw);
    }
}

fn prompt_optional(label: &str, current: Option<&str>) -> Result<Option<String>> {
    let raw = prompt(label, current)?;
    if raw.trim().is_empty() {
        return Ok(current.map(ToOwned::to_owned));
    }
    if raw.trim() == "-" {
        return Ok(None);
    }
    Ok(Some(raw))
}

fn prompt(label: &str, current: Option<&str>) -> Result<String> {
    let current_hint = current
        .map(|v| format!(" [current: {v}]"))
        .unwrap_or_default();
    print!("{label}{current_hint}: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}
