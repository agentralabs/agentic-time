---
status: stable
---

# Architecture

AgenticTime is a 4-crate Rust workspace with additional language bindings.

## Workspace Structure

```
agentic-time/
  Cargo.toml                    (workspace root)
  crates/
    agentic-time-core/          (core library)
    agentic-time-mcp/           (MCP server)
    agentic-time-cli/           (CLI binary: atime)
    agentic-time-ffi/           (C FFI shared library)
  python/                       (Python SDK via PyO3)
  npm/wasm/                     (npm WASM package)
```

## Crate Responsibilities

### agentic-time-core

The core library. All temporal logic lives here.

- Entity types: Deadline, Duration, Schedule, Sequence, Decay
- File format: `.atime` binary format (magic `ATIM`, version 1)
- Graph operations: add, update, remove, query, export, import
- Decay curve math: exponential, linear, step
- Conflict detection: schedule overlap analysis
- Dependency resolution: deadline and sequence dependency graphs
- No I/O dependencies beyond file system access
- No MCP, CLI, or FFI concerns

### agentic-time-mcp

The MCP server binary (`agentic-time-mcp`).

- JSON-RPC 2.0 over stdio
- 19 MCP tools (deadline, duration, schedule, sequence, decay, utility)
- MCP resources via `atime://` URI scheme
- 4 MCP prompts (time_plan, time_review, time_estimate, time_schedule_day)
- Auto-session lifecycle management
- Content-Length framing with 8 MiB frame limit
- Input validation: no silent fallback for invalid parameters

### agentic-time-cli

The command-line interface binary (`atime`).

- Human-friendly terminal output
- All core operations exposed as subcommands
- Table, JSON, and text output formats
- File validation and info commands
- Import/export workflows

### agentic-time-ffi

C-compatible shared library for cross-language integration.

- Opaque handle pattern for graph instances
- JSON-based data exchange for complex types
- Thread-safe per-handle access
- Error propagation via return codes and `atime_last_error()`

## Data Flow

```
Agent (Claude/GPT/etc.)
  |
  | MCP protocol (JSON-RPC 2.0 over stdio)
  v
agentic-time-mcp
  |
  | Rust function calls
  v
agentic-time-core
  |
  | Binary I/O
  v
project.atime (file)
```

## File Format

The `.atime` binary format:

| Offset | Size | Field |
|--------|------|-------|
| 0 | 4 | Magic bytes: `ATIM` |
| 4 | 2 | Version: `0x0001` |
| 6 | 2 | Flags (reserved) |
| 8 | 8 | Entity count |
| 16 | 8 | Index offset |
| 24 | ... | Entity data (MessagePack encoded) |
| ... | ... | Entity index |

## Cross-Sister Integration

AgenticTime integrates with other Agentra sisters:

- **AgenticMemory**: Decay curves inform memory freshness. Deadlines link to decision nodes.
- **AgenticVision**: Schedule entries can trigger periodic visual captures.
- **AgenticCodebase**: Sequences model deployment pipelines. Duration estimates track refactoring effort.
- **AgenticIdentity**: Temporal operations are signed with identity receipts for audit trails.

## Runtime Isolation

Each project gets its own `.atime` file, resolved by deterministic path hashing. Same-name folders in different locations never share temporal state. File-level locking with stale lock recovery ensures safe concurrent access.
