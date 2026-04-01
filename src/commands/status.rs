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
    println!("    region: {}", render_opt(cfg.bedrock.region.as_deref()));
    println!(
        "    profile: {}",
        render_opt(cfg.bedrock.profile.as_deref())
    );
    println!(
        "    model_id: {}",
        render_opt(cfg.bedrock.model_id.as_deref())
    );
    println!("  zai:");
    println!("    api_key: {}", mask_secret(cfg.zai.api_key.as_deref()));
    println!("    base_url: {}", render_opt(cfg.zai.base_url.as_deref()));
    println!("    model: {}", render_opt(cfg.zai.model.as_deref()));
}

fn render_opt(value: Option<&str>) -> &str {
    value.unwrap_or("<not set>")
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
