# Changelog

All notable changes to AgenticTime will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-26

Initial release of AgenticTime -- temporal reasoning for AI agents.

### Added

**Temporal Primitives (`agentic-time` core library)**
- Deadline management with hard/soft/target/recurring types and consequence tracking
- Duration estimation using PERT three-point model (optimistic, expected, pessimistic)
- Schedule management with recurrence patterns and conflict detection
- Sequence workflows with ordered steps, dependencies, and parallel execution
- Decay models: linear, exponential, half-life, and step functions
- `.atime` binary file format (magic `ATIM`, version 1) with atomic writes

**Query Engine**
- Overdue deadline detection
- Due-within time window queries
- Schedule conflict detection
- Available time slot finder
- Temporal statistics (counts by type and status)

**MCP Server (`agentic-time-mcp`)**
- 19 MCP tools exposing all temporal operations to AI agents
- Deadline tools: `time_deadline_add`, `time_deadline_complete`, `time_deadline_list`, `time_deadline_overdue`
- Schedule tools: `time_schedule_create`, `time_schedule_reschedule`, `time_schedule_range`, `time_schedule_available`, `time_schedule_conflicts`
- Sequence tools: `time_sequence_create`, `time_sequence_advance`, `time_sequence_status`
- Decay tools: `time_decay_create`, `time_decay_value`, `time_decay_alert`
- Duration tools: `time_duration_estimate`, `time_duration_aggregate`
- General tools: `time_stats`, `time_refresh`

**CLI (`agentic-time-cli`)**
- `atime deadline` -- manage deadlines (add, list, update, remove, upcoming, overdue)
- `atime schedule` -- manage schedules (add, list, conflicts, remove)
- `atime sequence` -- manage sequences (create, step, status, list)
- `atime decay` -- manage decay curves (configure, query, reset, list)
- `atime stats` -- print graph statistics

**FFI (`agentic-time-ffi`)**
- C-compatible API for all core operations
- Opaque handle types with explicit free

**Python (`agentic-time` Python package)**
- Python SDK wrapping the CLI binary via subprocess
- `TimeGraph` class for deadline, schedule, sequence, and decay operations
- Python 3.10+ compatibility

**Testing and Benchmarks**
- 83 tests across all crates (36 unit + 47 stress/edge-case)
- Stress tests: 500 deadlines, concurrent operations, Unicode labels, boundary dates
- Edge cases: empty strings, far-future dates, negative durations, corrupted files
- Criterion benchmarks for all temporal operations

**Documentation**
- Quickstart guide
- Installation guide (desktop, terminal, server profiles)
- Command surface reference (CLI + MCP + Resources + Prompts)
- Architecture overview
- Benchmark analysis
- FAQ

[0.1.0]: https://github.com/agentralabs/agentic-time/releases/tag/v0.1.0
