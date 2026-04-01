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
    pub region: Option<String>,
    pub profile: Option<String>,
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZaiConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
}
