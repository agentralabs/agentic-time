---
status: stable
---

# Experience With vs Without AgenticTime

A before-and-after view of AI agent temporal reasoning capabilities.

## Without AgenticTime

| Scenario | What Happens |
|----------|-------------|
| User sets a deadline | Agent acknowledges it but forgets in the next conversation |
| User asks "what's due this week?" | Agent has no persistent timeline to query |
| User estimates a task will take 2 hours | Estimate is lost; no tracking of actual vs estimated |
| Two meetings overlap | Agent cannot detect scheduling conflicts |
| User asks about a week-old decision | Agent treats it with the same weight as a fresh one |
| Multi-step deployment | Agent cannot model ordered dependencies |
| User switches projects | Timeline state from previous project bleeds through |

## With AgenticTime

| Scenario | What Happens |
|----------|-------------|
| User sets a deadline | Persisted to `.atime` file, survives across conversations |
| User asks "what's due this week?" | Agent queries `time_deadline_list` with date filter |
| User estimates a task will take 2 hours | Stored with confidence; actual time tracked against it |
| Two meetings overlap | `time_schedule_conflicts` detects and reports the overlap |
| User asks about a week-old decision | Decay curve quantifies freshness at 0.91 (still relevant) |
| Multi-step deployment | Modeled as a Sequence with ordered steps and dependencies |
| User switches projects | Each project has its own `.atime` file, fully isolated |

## Key Improvements

1. **Persistence**: Temporal data survives across conversations and model restarts.
2. **Structure**: Five entity types (Deadline, Duration, Schedule, Sequence, Decay) model all common temporal patterns.
3. **Reasoning**: Agents can detect conflicts, track progress, and quantify freshness.
4. **Isolation**: Per-project temporal state prevents cross-contamination.
5. **Portability**: `.atime` files are portable across models, clients, and deployments.
