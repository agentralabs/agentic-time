---
status: stable
---

# Runtime and Install Sync

How AgenticTime installation, runtime detection, and file resolution work together.

## Install Flow

The installer (`scripts/install.sh`) performs these steps:

1. Detect platform (OS + architecture)
2. Validate install profile (desktop/terminal/server)
3. Download release binary or build from source
4. Install MCP launcher entrypoint
5. Merge MCP config into detected clients (desktop/terminal profiles only)
6. Print completion summary with verification steps

## MCP Launcher Entrypoint

The installer creates a launcher script at `~/.local/bin/agentic-time-mcp-agentra` that:

1. Resolves the project root (git root, env var, or cwd)
2. Locates the `.atime` file using the resolution chain
3. Passes the resolved file path to the actual `agentic-time-mcp` binary
4. Enforces auth token in server mode

## File Resolution Chain

The `.atime` file is resolved in order:

1. `ATIME_FILE` env var
2. `AGENTRA_ATIME_PATH` env var
3. `<repo_root>/.agentra/<slug>.atime`
4. `<repo_root>/.timeline.atime`
5. `$PWD/.timeline.atime`
6. `$PWD/timeline.atime`
7. `$HOME/.timeline.atime`
8. `$HOME/.agentra/time/default.atime`

If no file is found, a new one is created at option 3.

## Profile Behavior

| Profile | Binary | MCP Config | Auth Gate |
|---------|--------|------------|-----------|
| desktop | `~/.local/bin/` | Auto-merged | None |
| terminal | `~/.local/bin/` | Auto-merged | None |
| server | `~/.local/bin/` | Untouched | Required |

## Runtime Modes

- **Desktop**: Full MCP server with auto-session lifecycle
- **Terminal**: CLI-focused, no auto-session
- **Server**: Token-gated, no desktop config changes
- **Minimal**: Core tools only, no auto features

## Sync Guarantees

- Install is idempotent: re-running updates the binary without breaking config
- MCP config merge is additive: existing servers are never removed
- File resolution is deterministic: same project always resolves to the same `.atime`
- Project isolation: same-name folders in different paths get different `.atime` files
