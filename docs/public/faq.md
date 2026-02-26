---
status: stable
---

# FAQ

Frequently asked questions about AgenticTime.

## General

### What is AgenticTime?

AgenticTime is a temporal reasoning engine for AI agents. It provides structured types for deadlines, durations, schedules, sequences, and decay curves, persisted in a portable `.atime` file and exposed via MCP.

### Do I need other Agentra sisters to use AgenticTime?

No. AgenticTime is fully standalone. It works independently as a single MCP server binary. Integration with AgenticMemory, AgenticVision, AgenticCodebase, and AgenticIdentity is optional and additive.

### What file format does AgenticTime use?

AgenticTime uses the `.atime` binary format with magic bytes `ATIM`, version 1. Entities are MessagePack-encoded with a sorted index for fast lookups.

### Is my data stored locally?

Yes. All data is stored in a local `.atime` file. Nothing is sent to any remote server. The MCP server runs locally over stdio.

## Installation

### Which platforms are supported?

macOS (x86_64, aarch64), Linux (x86_64, aarch64), and Windows (x86_64).

### How do I update AgenticTime?

Re-run the installer. It will download the latest binary and update in place:

```bash
curl -fsSL https://agentralabs.tech/install/time | bash
```

### Can I install from source?

Yes. The installer falls back to `cargo install` if no pre-built binary is available. You can also build directly:

```bash
cargo install --git https://github.com/agentralabs/agentic-time --locked agentic-time-mcp
```

## MCP

### Which MCP clients work with AgenticTime?

Any MCP-compatible client: Claude Desktop, Claude Code, Cursor, Windsurf, VS Code, Codex, and any custom JSON-RPC 2.0 MCP client.

### How do I check if the MCP server is running?

```bash
agentic-time-mcp serve
# Press Ctrl+C after seeing the startup message
```

### Can I run multiple AgenticTime servers?

Yes. Each project gets its own `.atime` file, so multiple servers can run simultaneously for different projects.

## Temporal Reasoning

### How does decay work?

Decay models how information freshness decreases over time. The default exponential curve has a 168-hour (7-day) half-life: after one week, freshness is 50%. After two weeks, 25%. The curve and parameters are configurable.

### Can I use custom decay curves?

Yes. AgenticTime supports exponential, linear, and step curves with configurable parameters:

```bash
atime decay configure my-curve --curve exponential --halflife 336
```

### How are scheduling conflicts detected?

AgenticTime expands recurring schedules into individual occurrences and checks for time-window overlaps. Two events conflict if their time windows overlap by any amount.

### What happens when a deadline depends on another?

Deadline dependencies create a directed acyclic graph. If a prerequisite deadline is not completed, the dependent deadline is flagged as blocked. Cascade analysis shows the full impact of delays.

## Data

### How large can a .atime file get?

There is no hard limit. Files with 100,000 entities are approximately 9 MB. Performance remains sub-millisecond for most operations up to 10,000 entities.

### Can I export my data?

Yes. Export to JSON or iCal:

```bash
atime export --format json > timeline.json
atime export --type schedule --format ical > calendar.ics
```

### Can I merge two .atime files?

Yes, via import:

```bash
atime export --format json -f other.atime | atime import --merge
```

## Security

### Is authentication required?

Only in server profile mode. Desktop and terminal profiles do not require authentication. Server mode requires `AGENTIC_TOKEN` or `AGENTIC_TOKEN_FILE`.

### Is my data encrypted?

Not currently. The `.atime` file is stored in plaintext on disk. Encryption at rest is planned for a future release. Use filesystem-level encryption if needed.
