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

| Tool | Description |
|------|-------------|
| `time_deadline_add` | Add a deadline |
| `time_deadline_list` | List deadlines |
| `time_deadline_update` | Update a deadline |
| `time_deadline_remove` | Remove a deadline |
| `time_duration_estimate` | Create a duration estimate |
| `time_duration_track` | Start/stop duration tracking |
| `time_duration_report` | Duration estimate vs actual report |
| `time_schedule_add` | Add a schedule |
| `time_schedule_list` | List schedules |
| `time_schedule_conflicts` | Detect conflicts |
| `time_sequence_create` | Create a sequence |
| `time_sequence_step` | Manage sequence steps |
| `time_sequence_status` | Sequence progress |
| `time_decay_configure` | Configure decay |
| `time_decay_query` | Query freshness |
| `time_decay_reset` | Reset decay config |
| `time_stats` | Graph statistics |
| `time_export` | Export data |
| `time_import` | Import data |
| `time_log` | Log temporal observation |

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
