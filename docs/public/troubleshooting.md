---
status: stable
---

# Troubleshooting

Common issues and solutions for AgenticTime.

## Installation Issues

### Binary not found after install

Ensure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
# Add to ~/.bashrc or ~/.zshrc for persistence
```

### Install script fails with "jq not found"

The installer needs `jq` or `python3` for MCP config merging:

```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt install jq

# Or use python3 (usually pre-installed)
python3 --version
```

### Cargo build fails

Ensure you have the latest stable Rust toolchain:

```bash
rustup update stable
```

## MCP Server Issues

### Server not appearing in MCP client

1. Verify the binary exists: `ls ~/.local/bin/agentic-time-mcp`
2. Check config was merged: look for `agentic-time` in your MCP client config
3. Restart your MCP client completely (not just reload)
4. Run manually to check for errors: `agentic-time-mcp serve`

### "AGENTIC_TOKEN required" error

This occurs in server profile mode. Set the token:

```bash
export AGENTIC_TOKEN="$(openssl rand -hex 32)"
```

### Server crashes on startup

Check for stale lock files:

```bash
ls -la ~/.agentra/*.atime.lock
# Remove stale locks (check PID first)
rm ~/.agentra/*.atime.lock
```

## File Format Issues

### "Invalid magic bytes" error

The file is not a valid `.atime` file. Check:

```bash
xxd -l 4 project.atime
# Should show: 4154 494d (ATIM)
```

### "Version mismatch" error

The file was created with a newer version of AgenticTime. Update your installation:

```bash
curl -fsSL https://agentralabs.tech/install/time | bash
```

### File appears empty after save

Ensure `save()` is called explicitly in Python/FFI usage. The MCP server auto-saves.

## Temporal Logic Issues

### Deadlines not showing as overdue

Check timezone settings. Deadlines use UTC by default:

```bash
atime deadline list --format json | jq '.[].due_at'
```

### Schedule conflicts not detected

Conflict detection requires overlapping time windows. Ensure schedules have duration set:

```bash
atime schedule list --format json | jq '.[].duration_minutes'
```

### Decay curve returning unexpected values

Verify the decay configuration:

```bash
atime decay list
atime decay query my-config --age 0    # Should be 1.0
atime decay query my-config --age 168  # Should be ~0.5 for default exponential
```

## Performance Issues

### Slow startup with large .atime files

For graphs with more than 10,000 entities, consider:

1. Archiving completed deadlines: `atime export --status completed > archive.json`
2. Removing old entries: `atime deadline remove --status completed --before 2025-01-01`

### High memory usage

The entire graph is loaded into memory. For very large temporal datasets, split by project.

## Getting Help

- GitHub Issues: https://github.com/agentralabs/agentic-time/issues
- Documentation: https://agentralabs.tech/docs/time
