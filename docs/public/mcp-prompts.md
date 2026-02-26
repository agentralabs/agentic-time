---
status: stable
---

# MCP Prompts

AgenticTime provides 4 built-in MCP prompts that agents can invoke for structured temporal reasoning.

## `time_plan`

Generate a temporal plan for a project or task.

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `goal` | string | Yes | What needs to be accomplished |
| `deadline` | string | No | Target completion date (ISO 8601) |
| `constraints` | string | No | Known constraints or blockers |

### Behavior

The prompt instructs the agent to:

1. Break the goal into sequential phases
2. Estimate durations for each phase with confidence intervals
3. Identify dependencies between phases
4. Flag any scheduling conflicts with existing commitments
5. Produce a structured timeline with milestones

### Example

```json
{
  "name": "time_plan",
  "arguments": {
    "goal": "Ship authentication module v2",
    "deadline": "2026-04-01T00:00:00Z",
    "constraints": "Team of 2, one week vacation in March"
  }
}
```

## `time_review`

Review temporal health of the current project.

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `scope` | string | No | `all`, `deadlines`, `schedules`, `sequences` (default: `all`) |
| `window_days` | number | No | Look-ahead window in days (default: 14) |

### Behavior

The prompt instructs the agent to:

1. List upcoming deadlines sorted by urgency
2. Identify overdue items
3. Check for scheduling conflicts in the window
4. Report on sequence progress (percent complete)
5. Flag any deadlines at risk based on remaining time vs. estimated effort
6. Suggest priority adjustments

### Example

```json
{
  "name": "time_review",
  "arguments": {
    "scope": "deadlines",
    "window_days": 7
  }
}
```

## `time_estimate`

Estimate the duration of a task using historical data and heuristics.

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `task` | string | Yes | Description of the task to estimate |
| `complexity` | string | No | `trivial`, `simple`, `moderate`, `complex`, `unknown` (default: `unknown`) |
| `similar_to` | string | No | Reference to a previous task for anchoring |

### Behavior

The prompt instructs the agent to:

1. Analyze the task description
2. Compare against historical duration estimates if available
3. Apply complexity multipliers
4. Produce a three-point estimate (optimistic, likely, pessimistic)
5. Calculate a confidence-weighted estimate
6. Store the estimate in the temporal graph

### Example

```json
{
  "name": "time_estimate",
  "arguments": {
    "task": "Migrate database from MySQL to PostgreSQL",
    "complexity": "complex"
  }
}
```

## `time_schedule_day`

Plan and optimize the current day's schedule.

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `date` | string | No | Date to plan (default: today, ISO 8601 date) |
| `work_hours` | number | No | Available work hours (default: 8) |
| `priorities` | string | No | Comma-separated priority focus areas |

### Behavior

The prompt instructs the agent to:

1. Load all schedules and deadlines for the target date
2. Identify fixed commitments (meetings, recurring schedules)
3. Calculate available free blocks
4. Suggest task assignments for free blocks based on deadline urgency
5. Flag if the day is overcommitted
6. Produce a minute-by-minute schedule recommendation

### Example

```json
{
  "name": "time_schedule_day",
  "arguments": {
    "date": "2026-03-01",
    "work_hours": 6,
    "priorities": "api-review, testing"
  }
}
```
