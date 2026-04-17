use anyhow::Result;

use crate::config::{CapferryConfig, ConfigStore};

pub fn run(store: &ConfigStore) -> Result<()> {
    let cfg = store.load_or_create()?;
    print_status(store, &cfg);
    Ok(())
}

fn print_status(store: &ConfigStore, cfg: &CapferryConfig) {
    println!("capferry status");
    println!("  config_file: {}", store.path().display());
    println!("  active_provider: {}", cfg.active_provider);
    println!("  claude_path: {}", cfg.claude_path);
    println!("  install_dir: {}", cfg.install_dir);
    println!("  wrappers:");
    println!("    - claude-sub");
    println!("    - claude-bedrock");
    println!("    - claude-zai");
    println!(
        "    - claude-current -> {}",
        cfg.active_provider.wrapper_name()
    );
    println!("  bedrock:");
    println!(
        "    aws_profile: {}",
        render_opt_with_fallback(
            cfg.bedrock.aws_profile.as_deref(),
            "<inherit AWS default profile>"
        )
    );
    println!(
        "    aws_region: {}",
        render_opt_with_fallback(
            cfg.bedrock.aws_region.as_deref(),
            "<inherit AWS default region>"
        )
    );
    println!(
        "    sonnet_model: {}",
        render_opt(cfg.bedrock.sonnet_model.as_deref())
    );
    println!(
        "    opus_model: {}",
        render_opt(cfg.bedrock.opus_model.as_deref())
    );
    println!(
        "    haiku_model: {}",
        render_opt(cfg.bedrock.haiku_model.as_deref())
    );
    println!("  zai:");
    println!("    base_url: {}", render_opt(cfg.zai.base_url.as_deref()));
    println!(
        "    auth_token: {}",
        mask_secret(cfg.zai.auth_token.as_deref())
    );
    println!(
        "    sonnet_model: {}",
        render_opt(cfg.zai.sonnet_model.as_deref())
    );
    println!(
        "    opus_model: {}",
        render_opt(cfg.zai.opus_model.as_deref())
    );
    println!(
        "    haiku_model: {}",
        render_opt(cfg.zai.haiku_model.as_deref())
    );
}

fn render_opt(value: Option<&str>) -> &str {
    value.unwrap_or("<not set>")
}

fn render_opt_with_fallback<'a>(value: Option<&'a str>, fallback: &'a str) -> &'a str {
    value.unwrap_or(fallback)
}

fn mask_secret(value: Option<&str>) -> String {
    match value {
        None => "<not set>".to_owned(),
        Some(v) if v.is_empty() => "<empty>".to_owned(),
        Some(v) => {
            let chars: Vec<char> = v.chars().collect();
            if chars.len() <= 4 {
                "*".repeat(chars.len())
            } else {
                let tail: String = chars[chars.len() - 4..].iter().collect();
                format!("{}{}", "*".repeat(chars.len() - 4), tail)
            }
        }
    }
}
