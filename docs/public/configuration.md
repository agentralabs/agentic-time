---
status: stable
---

# Configuration

AgenticTime configuration options for all runtime modes.

## Environment Variables

| Variable | Default | Allowed Values | Effect |
|----------|---------|----------------|--------|
| `ATIME_FILE` | Auto-detected | Path to `.atime` file | Explicit time graph file path |
| `ATIME_TIMEZONE` | `UTC` | Any IANA timezone | Default timezone for deadline display |
| `ATIME_DECAY_MODEL` | `exponential` | `linear`, `exponential`, `step` | Default decay curve type |
| `ATIME_DECAY_HALFLIFE` | `168` (hours) | Positive integer | Half-life for exponential decay in hours |
| `ATIME_MAX_DEADLINES` | `10000` | Positive integer | Maximum deadlines per graph |
| `ATIME_MAX_SCHEDULES` | `5000` | Positive integer | Maximum schedules per graph |
| `ATIME_LOG_LEVEL` | `info` | `trace`, `debug`, `info`, `warn`, `error` | Logging verbosity |
| `AGENTRA_WORKSPACE_ROOT` | Auto-detected | Directory path | Override workspace root detection |
| `AGENTRA_PROJECT_ROOT` | Auto-detected | Directory path | Override project root detection |
| `AGENTIC_TOKEN` | None | String | Auth token for server profile |
| `AGENTIC_TOKEN_FILE` | None | File path | Auth token file for server profile |

## MCP Server Configuration

The MCP server (`agentic-time-mcp`) accepts the following arguments:

```json
{
  "mcpServers": {
    "agentic-time": {
      "command": "~/.local/bin/agentic-time-mcp-agentra",
      "args": ["serve"]
    }
  }
}
```

## Decay Curve Configuration

### Exponential (default)

Freshness drops by 50% every half-life period:

```
freshness(t) = 0.5 ^ (t / halflife)
```

### Linear

Freshness drops linearly to zero over the configured window:

```
freshness(t) = max(0, 1 - t / window)
```

### Step

Freshness remains at 1.0 until threshold, then drops to a configured floor:

```
freshness(t) = 1.0 if t < threshold else floor
```

## File Location Resolution

AgenticTime resolves the `.atime` file in this order:

1. `ATIME_FILE` environment variable (if set)
2. `AGENTRA_ATIME_PATH` environment variable (if set)
3. `<repo_root>/.agentra/<slug>.atime`
4. `<repo_root>/.timeline.atime`
5. `$PWD/.timeline.atime`
6. `$PWD/timeline.atime`
7. `$HOME/.timeline.atime`
8. `$HOME/.agentra/time/default.atime`

## Runtime Modes

| Mode | Trigger | Behavior |
|------|---------|----------|
| `desktop` | Default | Full MCP server, auto-session |
| `terminal` | `--mode terminal` | CLI-optimized, no auto-session |
| `server` | `AGENTRA_RUNTIME_MODE=server` | Token-gated, no desktop config |
| `minimal` | `--mode minimal` | Core tools only, no auto-session |
