# capferry

`capferry` manages provider-specific Claude wrapper commands (`claude-sub`, `claude-bedrock`, `claude-zai`) and keeps a `claude-current` alias synced to your selected provider.

## Installation Guide

### 1. Prerequisites

- macOS or Linux
- Rust toolchain (`rustup`, `cargo`)
- Claude CLI installed and reachable from terminal

Check:

```bash
rustc --version
cargo --version
claude --version
```

### 2. Install `capferry`

From this repository:

```bash
cargo install --path .
```

This installs the `capferry` binary in `~/.cargo/bin`.

Alternative (dev run without install):

```bash
cargo run -- --help
```

### 3. Ensure PATH is configured

Your shell must include:

- `~/.cargo/bin` (for `capferry`)
- `~/.local/bin` (for generated wrapper scripts)

For `zsh`, add to `~/.zshrc`:

```bash
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
export PATH="$HOME/.local/bin:$PATH"
```

Then reload:

```bash
source ~/.zshrc
```

### 4. First-time setup

Run:

```bash
capferry configure
```

Then verify:

```bash
capferry doctor
capferry status
```

## Basic Usage

- `capferry status` - show active config
- `capferry install` - install wrapper scripts
- `capferry use bedrock|zai|sub` - switch provider and refresh `claude-current`
- `capferry configure` - interactive config update + reinstall wrappers
- `capferry doctor` - run environment and config checks

## Troubleshooting

- `capferry: command not found`
  - Ensure `~/.cargo/bin` is in PATH, then restart shell.
- Wrapper commands not found (`claude-sub`, etc.)
  - Ensure `~/.local/bin` is in PATH.
  - Run `capferry install`.
- Claude binary check fails
  - Confirm `claude` is installed and runnable.
  - Update `claude_path` in `~/.capferry/config.toml` if needed.

## Build and Test

```bash
cargo build --release
cargo test
```
