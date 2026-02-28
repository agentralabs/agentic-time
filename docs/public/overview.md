---
status: stable
---

# AgenticTime Overview

AgenticTime provides temporal reasoning for AI agents. It stores deadlines, durations, schedules, sequences, and decay curves in a single portable `.atime` file.

## What you can do

- Model deadlines, durations, schedules, sequences, and temporal decay as structured entities.
- Query temporal state through MCP with `agentic-time-mcp`.
- Reason about time-dependent priorities, scheduling conflicts, and deadline proximity.
- Track how information freshness decays over time with configurable decay curves.

## Why teams adopt AgenticTime

Teams adopt AgenticTime because it closes the temporal reasoning gap in AI agents:

- Foundational problems already solved: agents lose track of deadlines, cannot reason about duration estimates, have no concept of scheduling conflicts, and treat all information as equally fresh regardless of age.
- New high-scale problems now solved: multi-project timeline coordination, deadline cascading and dependency tracking, schedule optimization across workstreams, and temporal decay modeling for memory freshness.
- Practical outcome for teams: agents track deadlines, schedule work intelligently, reason about time dependencies, and automatically deprioritize stale information.

## Five Entity Types

1. **Duration** -- A measured span of time (e.g., "2 hours", "3 days"). Used for estimation, budgeting, and elapsed-time tracking.
2. **Deadline** -- A fixed point in time with an associated task or milestone. Supports priority, status, and dependency edges.
3. **Schedule** -- A recurring or one-time calendar block. Supports cron-like recurrence, conflict detection, and resource binding.
4. **Sequence** -- An ordered chain of temporal events with dependency constraints. Models workflows, pipelines, and phased rollouts.
5. **Decay** -- A function that models information freshness over time. Supports linear, exponential, and step decay curves.

## 16 Advanced Capabilities

AgenticTime introduces 16 novel capabilities for temporal reasoning:

1. Portable binary temporal graph (.atime format)
2. Deadline cascade analysis
3. Duration estimation with confidence intervals
4. Schedule conflict detection
5. Sequence dependency resolution
6. Temporal decay curves for memory freshness
7. Cross-sister temporal linking (Memory + Time)
8. Deadline proximity alerting
9. Time-budget tracking per project
10. Recurring schedule expansion
11. Temporal workspace comparison
12. Schedule optimization suggestions
13. Decay-aware memory retrieval
14. Timeline visualization data export
15. Multi-timezone deadline normalization
16. Temporal pattern recognition

## MCP Tools

AgenticTime exposes 19 MCP tools through `agentic-time-mcp`:

- `time_deadline_add`, `time_deadline_list`, `time_deadline_update`, `time_deadline_remove`
- `time_duration_estimate`, `time_duration_track`, `time_duration_report`
- `time_schedule_add`, `time_schedule_list`, `time_schedule_conflicts`
- `time_sequence_create`, `time_sequence_step`, `time_sequence_status`
- `time_decay_configure`, `time_decay_query`, `time_decay_reset`
- `time_stats`, `time_export`, `time_import`, `time_log`

## File Format

- Primary artifact: `.atime`
- Binary format with magic bytes `ATIM`, version 1
- Cross-sister server workflows can pair `.atime` with `.amem`, `.acb`, and `.avis`

## Start here

- [Quickstart](quickstart.md) -- get running in 30 seconds
- [Installation](installation.md) -- all platforms and profiles
- [MCP Tools](mcp-tools.md) -- full tool reference
- [Architecture](architecture.md) -- 4-crate workspace structure
