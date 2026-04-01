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

    if let Some(region) = &cfg.bedrock.region {
        lines.push(format!(
            "export AWS_REGION={}",
            shell_single_quote(region.as_str())
        ));
    }

    if let Some(profile) = &cfg.bedrock.profile {
        lines.push(format!(
            "export AWS_PROFILE={}",
            shell_single_quote(profile.as_str())
        ));
    }

    if let Some(model_id) = &cfg.bedrock.model_id {
        lines.push(format!(
            "export BEDROCK_MODEL_ID={}",
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

    if let Some(api_key) = &cfg.zai.api_key {
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

    if let Some(model) = &cfg.zai.model {
        lines.push(format!(
            "export ZAI_MODEL={}",
            shell_single_quote(model.as_str())
        ));
        lines.push(format!(
            "export ANTHROPIC_MODEL={}",
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
