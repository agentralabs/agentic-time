---
status: stable
---

# Installation

AgenticTime supports macOS, Linux, and Windows. Install profiles: `desktop`, `terminal`, `server`.

## One-liner (recommended)

```bash
# Default (desktop profile)
curl -fsSL https://agentralabs.tech/install/time | bash

# Explicit profiles
curl -fsSL https://agentralabs.tech/install/time/desktop | bash
curl -fsSL https://agentralabs.tech/install/time/terminal | bash
curl -fsSL https://agentralabs.tech/install/time/server | bash
```

## npm

```bash
npm install @agenticamem/time
```

## PyPI

```bash
pip install agentic-time
```

## Cargo (from source)

```bash
cargo install --git https://github.com/agentralabs/agentic-time --locked agentic-time-mcp
```

## What the installer does

1. Downloads the `agentic-time-mcp` binary to `~/.local/bin/`
2. Merges (never overwrites) MCP config into detected client configs
3. Leaves all existing MCP servers untouched

## Profiles

| Profile | Binary | MCP Config | Auth |
|---------|--------|------------|------|
| `desktop` | Installed | Auto-merged into Claude/Cursor/Windsurf/VS Code/Codex | None |
| `terminal` | Installed | Auto-merged (same as desktop) | None |
| `server` | Installed | No desktop config changes | `AGENTIC_TOKEN` required |

## Standalone guarantee

AgenticTime is fully standalone. It does not require any other Agentra sister to be installed. It works independently as a single MCP server binary.

## Post-install verification

```bash
# Check binary
agentic-time-mcp serve
# Ctrl+C after startup confirmation

# Check CLI
atime --help
```

## Requirements

- `curl` (for download)
- `jq` or `python3` (for MCP config merge)
- Rust toolchain (only if building from source)

## Platform support

| Platform | Architecture | Status |
|----------|-------------|--------|
| macOS | x86_64, aarch64 | Supported |
| Linux | x86_64, aarch64 | Supported |
| Windows | x86_64 | Supported |
