---
status: stable
---

# Integration Guide

How to integrate AgenticTime with other tools, frameworks, and Agentra sisters.

## MCP Integration

AgenticTime runs as an MCP server. Any MCP-compatible client can use it:

- Claude Desktop
- Claude Code
- Cursor
- Windsurf
- VS Code (with MCP extension)
- Codex
- Any JSON-RPC 2.0 MCP client

### Configuration

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

## Sister Integration

### AgenticTime + AgenticMemory

Link temporal context to memory nodes:

- Use decay curves to weight memory freshness when retrieving
- Attach deadline IDs to decision nodes for temporal context
- Use `time_log` to record temporal observations alongside `conversation_log`

### AgenticTime + AgenticVision

Schedule periodic visual captures:

- Create schedules for regular UI state snapshots
- Link vision observations to timeline events
- Track UI changes over time with temporal anchoring

### AgenticTime + AgenticCodebase

Track development timelines:

- Model deployment pipelines as sequences
- Estimate refactoring durations and track actuals
- Link code analysis sessions to deadlines

### AgenticTime + AgenticIdentity

Audit temporal operations:

- Sign deadline modifications with identity receipts
- Track schedule changes with action context
- Use competence records to improve duration estimates

## Python Integration

```python
from agentic_time import TimeGraph

# Open or create
tg = TimeGraph("project.atime")

# Add entities
deadline_id = tg.add_deadline("Ship v1.0", "2026-03-15T17:00:00Z", priority="high")
tg.add_duration_estimate("Auth refactor", hours=8, confidence=0.7)
tg.add_schedule("Daily standup", recurrence="0 9 * * MON-FRI", duration_minutes=15)

# Query
upcoming = tg.list_deadlines(status="pending", sort_by="due_at")
conflicts = tg.check_conflicts("2026-03-10", "2026-03-14")

# Save
tg.save()
```

## REST API (Server Profile)

When running in server mode, AgenticTime accepts JSON-RPC 2.0 requests:

```bash
curl -X POST http://localhost:8080 \
  -H "Authorization: Bearer $AGENTIC_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"time_stats"},"id":1}'
```

## Webhook Integration

AgenticTime can be configured to emit webhook notifications for:

- Deadline approaching (configurable threshold)
- Deadline missed
- Schedule conflict detected
- Sequence step completed

## Environment Variables

See [Configuration](configuration.md) for the full environment variable reference.
