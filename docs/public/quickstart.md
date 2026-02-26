---
status: stable
---

# Quickstart

Get AgenticTime running in 30 seconds.

## One-liner install

```bash
curl -fsSL https://agentralabs.tech/install/time | bash
```

This downloads the `agentic-time-mcp` binary, installs it to `~/.local/bin/`, and auto-configures detected MCP clients (Claude Desktop, Cursor, Windsurf, VS Code, Codex).

Restart your MCP client after installation.

## Verify installation

```bash
agentic-time-mcp serve
# Press Ctrl+C after confirming startup
```

## First deadline

Once your MCP client restarts, ask your agent:

```
Set a deadline for "finish API review" on Friday at 5pm
```

The agent will call `time_deadline_add` and persist it to your `.atime` file.

## First duration estimate

```
How long will refactoring the auth module take?
```

The agent will call `time_duration_estimate` and store the estimate with confidence intervals.

## First schedule

```
Schedule a 2-hour code review block every Tuesday at 10am
```

The agent will call `time_schedule_add` with recurrence and persist the schedule.

## Python usage

```bash
pip install agentic-time
```

```python
from agentic_time import TimeGraph

tg = TimeGraph("project.atime")
tg.add_deadline("Ship v1.0", "2026-03-15T17:00:00Z", priority="high")
tg.add_duration_estimate("Auth refactor", hours=8, confidence=0.7)
tg.save()
```

## Next steps

- [Installation](installation.md) -- profiles and platform details
- [MCP Tools](mcp-tools.md) -- all 19 tools documented
- [Concepts](concepts.md) -- entity types and temporal model
