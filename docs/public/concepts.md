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
- **Statuses**: pending, in_progress, completed, missed, cancelled
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

- **Curves**: exponential (default), linear, step
- **Exponential**: `freshness = 0.5 ^ (age / halflife)` -- smooth, gradual decay
- **Linear**: `freshness = max(0, 1 - age / window)` -- steady decline to zero
- **Step**: `freshness = 1.0 if age < threshold else floor` -- binary fresh/stale

## Temporal Graph

All entities live in a temporal graph stored as a `.atime` file. The graph supports:

- Add, update, remove operations for all entity types
- Filtered queries with sorting and pagination
- Cross-entity references (e.g., deadlines linked to sequence steps)
- Export/import for backup and migration

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
