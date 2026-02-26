---
status: stable
---

# Primary Problem Coverage

AgenticTime solves the following primary temporal reasoning problems for AI agents.

## Problem 1: Deadline Amnesia

**Before**: Agents forget deadlines between conversations.
**After**: Deadlines persist in `.atime` files and are queryable across sessions.

## Problem 2: No Duration Tracking

**Before**: Task duration estimates are lost; no comparison to actual time spent.
**After**: Duration estimates stored with confidence intervals; actual time tracked.

## Problem 3: Schedule Blindness

**Before**: Agents cannot detect overlapping commitments.
**After**: `time_schedule_conflicts` detects all overlapping schedule entries.

## Problem 4: No Temporal Dependencies

**Before**: Multi-step workflows have no structured dependency model.
**After**: Sequences model ordered steps with status tracking and dependency constraints.

## Problem 5: Information Treated as Equally Fresh

**Before**: Week-old and minute-old information weighted equally.
**After**: Decay curves quantify freshness with configurable half-life parameters.

## Problem 6: Cross-Project Timeline Contamination

**Before**: Temporal state from one project leaks into another.
**After**: Per-project `.atime` files with deterministic path-based isolation.

## Problem 7: No Temporal Context in Memory

**Before**: Memory retrieval ignores when information was stored.
**After**: Decay-weighted retrieval integrates with AgenticMemory for freshness-aware recall.
