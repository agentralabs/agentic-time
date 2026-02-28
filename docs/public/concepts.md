---
status: stable
---

# Core Concepts

AgenticTime models temporal reasoning through five entity types and decay curves.

## Entity Types

### Deadline

A fixed point in time with an associated task or milestone.

- **Fields**: title, due_at (UTC), priority, status, tags, depends_on
- **Priority levels**: low, medium, high, critical
- **Statuses**: pending, in_progress, completed, completed_late, missed, cancelled
- **Dependencies**: deadlines can depend on other deadlines (cascade analysis)

### Duration

A measured or estimated span of time.

- **Fields**: label, estimate, confidence, actual, started_at, completed_at
- **Confidence**: 0.0 to 1.0, representing estimation certainty
- **Tracking**: start/stop actual time tracking to compare against estimates

### Schedule

A recurring or one-time calendar block.

- **Fields**: title, recurrence (cron), duration_minutes, timezone, start/end dates
- **Conflict detection**: overlapping schedules are flagged automatically
- **Expansion**: recurring schedules expand into individual occurrences for querying

### Sequence

An ordered chain of steps with dependency constraints.

- **Fields**: title, steps (ordered list of label + duration)
- **Progress**: each step tracks pending/completed/skipped status
- **Dependencies**: steps must be completed in order (or explicitly skipped)

### Decay

A function modeling how information freshness decreases over time.

- **Curves**: exponential (default), linear, half-life, step, custom
- **Exponential**: `value = initial * e^(-lambda * t)` -- smooth, continuous decay governed by a decay constant (lambda)
- **Linear**: `value = initial - (rate * t)` -- steady decline at a fixed rate per second until reaching the floor
- **Half-life**: `value = initial * 0.5^(t / half_life)` -- intuitive model where value halves at regular intervals
- **Step**: `value = initial - (drop * floor(t / interval))` -- discrete drops at regular intervals
- **Custom**: user-defined curve via interpolation points (time_offset_secs, value) with linear interpolation between points

All decay models support a configurable floor (minimum value the decay will never go below). The `refresh()` method resets the reference time back to now, restoring the value to its initial level. The `time_until(threshold)` method calculates how long before the value drops to a given level.

## Temporal Graph

All entities live in a temporal graph stored as a `.atime` file. The graph supports:

- Add, update, remove operations for all entity types
- Filtered queries with sorting and pagination
- Cross-entity references (e.g., deadlines linked to sequence steps)
- Export/import for backup and migration

## Query Engine

The query engine provides high-level temporal queries over the `.atime` graph:

- **Overdue deadlines**: Find all deadlines past their due date that are not completed or cancelled
- **Due within**: Find deadlines due within a specified duration from now
- **Schedule conflicts**: Detect overlapping schedule blocks
- **Decay queries**: Calculate current decay values at any point in time
- **Sequence progress**: Query step completion status across sequences

## Indexes

AgenticTime maintains in-memory indexes for fast query execution:

- **Deadline index**: Sorted by due date for efficient range queries
- **Decay index**: Tracks decay models for rapid freshness calculations
- **Sequence index**: Ordered step lookups for progress tracking

## File Format

The `.atime` binary format uses:

- Magic bytes: `ATIM`
- Version: 1
- MessagePack-encoded entity data
- Entity index for fast lookups

## Project Isolation

Each project gets its own `.atime` file. File location is resolved deterministically from the project root path. Same-name folders in different locations never share temporal state.

## Decay and Memory Integration

When used alongside AgenticMemory, decay curves can weight memory retrieval:

- Fresh memories (high decay score) are prioritized
- Stale memories (low decay score) are deprioritized or flagged for review
- Decay parameters are configurable per use case (code knowledge vs. user preferences)

## Timestamps

All timestamps are stored in UTC. Display timezone is configurable via `ATIME_TIMEZONE` environment variable. Deadline comparison and conflict detection always operate in UTC to avoid ambiguity.

## Write Engine

The write engine handles mutations to the `.atime` file with transactional semantics. All entity creation, updates, and deletions go through the write engine to ensure consistency between the in-memory graph and the persisted file. Concurrent writes are serialized through file-level locking.

## MCP Integration

`agentic-time-mcp` exposes temporal operations over stdio MCP transport. Tools include deadline management (`time_deadline_add`, `time_deadline_list`), schedule conflict detection (`time_schedule_conflicts`), decay queries (`time_decay_query`), and aggregate statistics (`time_stats`). The server supports the full MCP protocol including capability negotiation and tool listing.
