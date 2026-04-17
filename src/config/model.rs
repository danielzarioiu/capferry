use serde::{Deserialize, Serialize};

use crate::providers::Provider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapferryConfig {
    pub active_provider: Provider,
    pub claude_path: String,
    pub install_dir: String,
    #[serde(default)]
    pub bedrock: BedrockConfig,
    #[serde(default)]
    pub zai: ZaiConfig,
}

impl CapferryConfig {
    pub fn default_with_install_dir(install_dir: String) -> Self {
        Self {
            active_provider: Provider::Subscription,
            claude_path: "claude".to_owned(),
            install_dir,
            bedrock: BedrockConfig::default(),
            zai: ZaiConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BedrockConfig {
    pub aws_profile: Option<String>,
    pub aws_region: Option<String>,
    pub sonnet_model: Option<String>,
    pub opus_model: Option<String>,
    pub haiku_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZaiConfig {
    pub base_url: Option<String>,
    pub auth_token: Option<String>,
    pub sonnet_model: Option<String>,
    pub opus_model: Option<String>,
    pub haiku_model: Option<String>,
}

impl CapferryConfig {
    pub fn active_provider_errors(&self) -> Vec<String> {
        match self.active_provider {
            Provider::Subscription => Vec::new(),
            Provider::Bedrock => validate_bedrock(&self.bedrock),
            Provider::Zai => validate_zai(&self.zai),
        }
    }
}

pub fn validate_bedrock(cfg: &BedrockConfig) -> Vec<String> {
    let _ = cfg;
    Vec::new()
}

pub fn validate_zai(cfg: &ZaiConfig) -> Vec<String> {
    let mut errors = Vec::new();
    ensure_required(&mut errors, "zai.base_url", cfg.base_url.as_deref());
    ensure_required(&mut errors, "zai.auth_token", cfg.auth_token.as_deref());
    ensure_required(&mut errors, "zai.sonnet_model", cfg.sonnet_model.as_deref());
    ensure_required(&mut errors, "zai.opus_model", cfg.opus_model.as_deref());
    ensure_required(&mut errors, "zai.haiku_model", cfg.haiku_model.as_deref());

    if let Some(base_url) = cfg.base_url.as_deref()
        && !(base_url.starts_with("https://") || base_url.starts_with("http://"))
    {
        errors.push("zai.base_url must start with http:// or https://".to_owned());
    }

    errors
}

fn ensure_required(errors: &mut Vec<String>, name: &str, value: Option<&str>) {
    if value.map(str::trim).is_none_or(str::is_empty) {
        errors.push(format!("{name} is required"));
    }
}
