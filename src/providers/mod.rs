use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Provider {
    #[value(alias = "sub")]
    Subscription,
    Bedrock,
    Zai,
}

impl Provider {
    pub fn wrapper_name(self) -> &'static str {
        match self {
            Self::Subscription => "claude-sub",
            Self::Bedrock => "claude-bedrock",
            Self::Zai => "claude-zai",
        }
    }
}

impl Display for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Subscription => write!(f, "subscription"),
            Self::Bedrock => write!(f, "bedrock"),
            Self::Zai => write!(f, "zai"),
        }
    }
}
