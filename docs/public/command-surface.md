---
status: stable
---

# Command Surface

Complete listing of all AgenticTime commands across CLI, MCP, and FFI surfaces.

## CLI Commands (atime)

| Command | Description |
|---------|-------------|
| `atime deadline add` | Add a new deadline |
| `atime deadline list` | List deadlines with filters |
| `atime deadline update` | Update a deadline |
| `atime deadline remove` | Remove a deadline |
| `atime deadline upcoming` | Show deadlines due within 7 days |
| `atime deadline overdue` | Show overdue deadlines |
| `atime schedule add` | Add a schedule entry |
| `atime schedule list` | List schedule entries |
| `atime schedule conflicts` | Detect scheduling conflicts |
| `atime schedule remove` | Remove a schedule entry |
| `atime sequence create` | Create a new sequence |
| `atime sequence step add` | Add a step to a sequence |
| `atime sequence step complete` | Complete a sequence step |
| `atime sequence status` | View sequence status |
| `atime sequence list` | List all sequences |
| `atime decay configure` | Configure a decay curve |
| `atime decay query` | Query freshness at a given age |
| `atime decay reset` | Reset a decay config |
| `atime decay list` | List all decay configs |
| `atime stats` | Print graph statistics |
| `atime export` | Export temporal data |
| `atime import` | Import temporal data |
| `atime validate` | Validate a .atime file |
| `atime info` | Print file metadata |

## MCP Tools (agentic-time-mcp)

### Deadline Tools

| Tool | Description |
|------|-------------|
| `time_deadline_add` | Add a new deadline with optional warning threshold and consequences |
| `time_deadline_complete` | Mark a deadline as completed |
| `time_deadline_list` | List deadlines with optional filters |
| `time_deadline_overdue` | List all overdue deadlines |

### Schedule Tools

| Tool | Description |
|------|-------------|
| `time_schedule_create` | Create a new scheduled event |
| `time_schedule_reschedule` | Reschedule an event to a new time |
| `time_schedule_range` | Get schedule for a time range |
| `time_schedule_available` | Find available time slots in a range |
| `time_schedule_conflicts` | Check for scheduling conflicts |

### Sequence Tools

| Tool | Description |
|------|-------------|
| `time_sequence_create` | Create a sequence of ordered steps |
| `time_sequence_advance` | Complete the current step and advance |
| `time_sequence_status` | Get the current status of a sequence |

### Decay Tools

| Tool | Description |
|------|-------------|
| `time_decay_create` | Create a decay model tracking value over time |
| `time_decay_value` | Get the current value of a decay model |
| `time_decay_alert` | Get decay models approaching a threshold |

### Duration Tools

| Tool | Description |
|------|-------------|
| `time_duration_estimate` | Create a duration estimate with uncertainty (PERT model) |
| `time_duration_aggregate` | Aggregate multiple duration estimates |

### General Tools

| Tool | Description |
|------|-------------|
| `time_stats` | Get temporal statistics (counts by type and status) |
| `time_refresh` | Refresh all statuses (update overdue deadlines, recalculate decays) |

## MCP Resources (atime://)

| Resource | Description |
|----------|-------------|
| `atime://deadlines` | All deadlines |
| `atime://deadlines/{id}` | Single deadline |
| `atime://deadlines/upcoming` | Next 7 days |
| `atime://deadlines/overdue` | Past due |
| `atime://schedules` | All schedules |
| `atime://schedules/today` | Today's schedule |
| `atime://schedules/week` | This week |
| `atime://sequences` | All sequences |
| `atime://sequences/{id}` | Single sequence |
| `atime://decay/{name}` | Decay curve data |
| `atime://stats` | Graph statistics |
| `atime://timeline` | Merged timeline view |

## MCP Prompts

| Prompt | Description |
|--------|-------------|
| `time_plan` | Generate a temporal plan |
| `time_review` | Review temporal health |
| `time_estimate` | Estimate task duration |
| `time_schedule_day` | Plan a day's schedule |
