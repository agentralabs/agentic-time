---
status: stable
---

# MCP Tools

AgenticTime exposes 19 tools through the MCP protocol via `agentic-time-mcp`.

## Deadline Tools

### `time_deadline_add`

Add a new deadline to the temporal graph.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `title` | string | Yes | Human-readable deadline title |
| `due_at` | string | Yes | ISO 8601 datetime (e.g., "2026-03-15T17:00:00Z") |
| `priority` | string | No | `low`, `medium`, `high`, `critical` (default: `medium`) |
| `tags` | array | No | List of string tags |
| `depends_on` | array | No | List of deadline IDs this depends on |

**Returns:** `{ "id": 42, "title": "...", "due_at": "..." }`

### `time_deadline_list`

List deadlines with optional filters.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter by status: `pending`, `in_progress`, `completed`, `missed`, `cancelled` |
| `priority` | string | No | Filter by priority |
| `limit` | number | No | Maximum results (default: 50) |
| `sort_by` | string | No | `due_at`, `priority`, `created_at` (default: `due_at`) |

### `time_deadline_update`

Update an existing deadline.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | Yes | Deadline ID |
| `title` | string | No | New title |
| `due_at` | string | No | New due date |
| `priority` | string | No | New priority |
| `status` | string | No | New status |

### `time_deadline_remove`

Remove a deadline from the graph.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | Yes | Deadline ID to remove |

## Duration Tools

### `time_duration_estimate`

Create a new duration estimate.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `label` | string | Yes | What is being estimated |
| `hours` | number | No | Estimated hours |
| `minutes` | number | No | Estimated minutes |
| `confidence` | number | No | Confidence level 0.0-1.0 (default: 0.8) |

### `time_duration_track`

Start or stop tracking actual elapsed time for an estimate.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | Yes | Duration estimate ID |
| `action` | string | Yes | `start` or `stop` |

### `time_duration_report`

Get a report comparing estimates to actual durations.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | number | No | Maximum results (default: 20) |

## Schedule Tools

### `time_schedule_add`

Add a schedule entry.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `title` | string | Yes | Schedule entry title |
| `recurrence` | string | Yes | Cron expression or `once` |
| `duration_minutes` | number | Yes | Duration in minutes |
| `timezone` | string | No | IANA timezone (default: UTC) |
| `start_date` | string | No | First occurrence date |
| `end_date` | string | No | Last occurrence date |

### `time_schedule_list`

List schedule entries.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `from` | string | No | Start of time window (ISO 8601) |
| `to` | string | No | End of time window (ISO 8601) |
| `limit` | number | No | Maximum results (default: 50) |

### `time_schedule_conflicts`

Detect scheduling conflicts within a time window.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `from` | string | Yes | Start of time window |
| `to` | string | Yes | End of time window |

## Sequence Tools

### `time_sequence_create`

Create a new sequence (ordered chain of steps).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `title` | string | Yes | Sequence title |
| `steps` | array | No | Initial steps as `[{"label": "...", "duration_minutes": N}]` |

### `time_sequence_step`

Add a step to an existing sequence or update step status.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `sequence_id` | number | Yes | Sequence ID |
| `action` | string | Yes | `add`, `complete`, `skip` |
| `label` | string | Conditional | Required for `add` |
| `step_index` | number | Conditional | Required for `complete`/`skip` |
| `duration_minutes` | number | No | Estimated duration for `add` |

### `time_sequence_status`

Get the current status of a sequence.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `sequence_id` | number | Yes | Sequence ID |

## Decay Tools

### `time_decay_configure`

Configure a named decay curve.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Decay configuration name |
| `curve` | string | No | `exponential`, `linear`, `step` (default: `exponential`) |
| `halflife_hours` | number | No | Half-life in hours (default: 168) |
| `window_hours` | number | No | Linear decay window in hours |
| `threshold_hours` | number | No | Step function threshold |
| `floor` | number | No | Step function floor value |

### `time_decay_query`

Query freshness for a given age.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Decay configuration name |
| `age_hours` | number | Yes | Age in hours |

**Returns:** `{ "freshness": 0.73, "curve": "exponential", "age_hours": 48 }`

### `time_decay_reset`

Reset a decay configuration to defaults.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Decay configuration name |

## Utility Tools

### `time_stats`

Get temporal graph statistics.

**Returns:** `{ "deadlines": 42, "schedules": 15, "sequences": 3, "decay_configs": 2, "total_entities": 62 }`

### `time_export`

Export temporal data in JSON format.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `format` | string | No | `json` (default), `ical` |
| `entity_type` | string | No | Filter by type: `deadline`, `schedule`, `sequence`, `decay` |

### `time_import`

Import temporal data from JSON.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `data` | string | Yes | JSON string of temporal entities |
| `merge` | boolean | No | Merge with existing (default: true) |

### `time_log`

Log a temporal observation or context note.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `content` | string | Yes | The temporal observation to log |
| `topic` | string | No | Category or topic |
