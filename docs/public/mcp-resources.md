---
status: stable
---

# MCP Resources

AgenticTime exposes temporal data through the `atime://` URI scheme in the MCP Resources API.

## URI Scheme

All resources use the `atime://` prefix.

## Available Resources

### `atime://deadlines`

Returns all deadlines in the current temporal graph.

**Format:** JSON array of deadline objects.

```json
[
  {
    "id": 1,
    "title": "Ship v1.0",
    "due_at": "2026-03-15T17:00:00Z",
    "priority": "high",
    "status": "pending",
    "tags": ["release"],
    "depends_on": []
  }
]
```

### `atime://deadlines/{id}`

Returns a single deadline by ID.

### `atime://deadlines/upcoming`

Returns deadlines due within the next 7 days, sorted by proximity.

### `atime://deadlines/overdue`

Returns deadlines that are past due and not completed or cancelled.

### `atime://schedules`

Returns all schedule entries.

```json
[
  {
    "id": 1,
    "title": "Weekly code review",
    "recurrence": "0 10 * * TUE",
    "duration_minutes": 120,
    "timezone": "America/New_York"
  }
]
```

### `atime://schedules/{id}`

Returns a single schedule entry by ID.

### `atime://schedules/today`

Returns schedule entries for the current day.

### `atime://schedules/week`

Returns schedule entries for the current week.

### `atime://sequences`

Returns all sequences.

### `atime://sequences/{id}`

Returns a single sequence with all steps and their statuses.

### `atime://sequences/{id}/next`

Returns the next pending step in a sequence.

### `atime://decay/{name}`

Returns the decay configuration and current freshness curve data for a named decay config.

```json
{
  "name": "memory-freshness",
  "curve": "exponential",
  "halflife_hours": 168,
  "sample_points": [
    {"age_hours": 0, "freshness": 1.0},
    {"age_hours": 24, "freshness": 0.91},
    {"age_hours": 168, "freshness": 0.5},
    {"age_hours": 336, "freshness": 0.25}
  ]
}
```

### `atime://stats`

Returns temporal graph statistics.

```json
{
  "deadlines": 42,
  "schedules": 15,
  "sequences": 3,
  "decay_configs": 2,
  "total_entities": 62,
  "file_path": "/path/to/project.atime",
  "file_size_bytes": 8192
}
```

### `atime://timeline`

Returns a merged chronological view of all temporal entities (deadlines, schedule occurrences, sequence steps) within a configurable window.

Query parameters:
- `from` -- start of window (default: now)
- `to` -- end of window (default: 7 days from now)

## Resource Templates

### `atime://deadlines?priority={priority}`

Filter deadlines by priority level.

### `atime://schedules?from={iso8601}&to={iso8601}`

Filter schedules within a time window.

## Cross-Sister Resources

When running alongside other Agentra sisters, AgenticTime resources can be referenced in memory nodes and codebase analysis:

- Memory nodes can link to `atime://deadlines/{id}` for temporal context
- Codebase analysis can reference `atime://sequences/{id}` for deployment pipelines
- Vision captures can link to `atime://schedules/{id}` for observation scheduling
