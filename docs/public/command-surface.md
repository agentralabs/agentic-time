---
status: stable
---

# Command Surface

Complete listing of all AgenticTime commands across CLI, MCP, and FFI surfaces.

## CLI Commands (atime)

| Command | Description |
|---------|-------------|
| `atime init` | Initialize a new `.atime` file |
| `atime info` | Print file metadata |
| `atime query` | Query temporal entities by type/text/status |
| `atime export` | Export temporal data as JSON |
| `atime ground` | Verify a claim has temporal backing |
| `atime evidence` | Return supporting temporal evidence |
| `atime suggest` | Suggest similar temporal records |
| `atime workspace create` | Create a workspace label |
| `atime workspace add` | Add an `.atime` context to a workspace |
| `atime workspace list` | List workspace contexts |
| `atime workspace query` | Query across workspace contexts |
| `atime workspace compare` | Compare an item across contexts |
| `atime workspace xref` | Cross-reference context presence/absence |
| `atime runtime-sync` | Scan workspace `.atime` files and write sync snapshot |
| `atime deadline add` | Add a new deadline |
| `atime deadline list` | List deadlines |
| `atime deadline complete` | Mark a deadline complete |
| `atime deadline cancel` | Cancel a deadline |
| `atime deadline overdue` | Show overdue deadlines |
| `atime schedule create` | Create a schedule entry |
| `atime schedule today` | Show today's schedule window |
| `atime sequence create` | Create a new sequence |
| `atime sequence advance` | Advance sequence to next step |
| `atime sequence status` | View sequence status |
| `atime sequence list` | List all sequences |
| `atime decay create` | Configure a decay model |
| `atime decay value` | Read current decay value |
| `atime decay list` | List all decay configs |
| `atime stats` | Print graph statistics |
| `atime refresh` | Refresh temporal status projections |

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
| `time_ground` | Verify a claim has temporal backing |
| `time_evidence` | Get supporting evidence for a temporal query |
| `time_suggest` | Suggest near matches for a temporal query |
| `time_workspace_create` | Create a multi-time workspace |
| `time_workspace_add` | Add an `.atime` file to workspace |
| `time_workspace_list` | List workspace contexts |
| `time_workspace_query` | Query across workspace contexts |
| `time_workspace_compare` | Compare an item across contexts |
| `time_workspace_xref` | Cross-reference item presence across contexts |
| `session_start` | Start a time MCP interaction session |
| `session_end` | End a time MCP interaction session |
| `time_session_resume` | Resume recent session context |

### Advanced Tools

| Tool | Description |
|------|-------------|
| `time_timeline_fork` | Fork the timeline for what-if analysis at a decision point |
| `time_clone_race` | Create parallel exploration clones racing to find the best approach |
| `time_echo_detect` | Detect temporal echoes and ripple effects from an event |
| `time_deadline_prophecy` | Predict deadline outcomes based on current trajectory |
| `time_causal_archaeology` | Excavate cause-effect chains from temporal history |
| `time_future_memory` | Precompute future context for an upcoming point in time |
| `time_debt_analyze` | Analyze temporal debt from overdue and deferred items |
| `time_gravity_detect` | Detect chrono-gravity wells pulling temporal items toward them |
| `time_entanglement_create` | Create a temporal entanglement linking entities that must move together |
| `time_anomaly_scan` | Scan the temporal graph for anomalies and health issues |
| `time_decay_reversal` | Analyze reversal options for a decaying value |
| `time_dilation_analyze` | Analyze time dilation patterns and subjective time perception |
| `time_jump_create` | Jump to a specific point in the project timeline |
| `time_anchor_create` | Create a temporal anchor capturing current state as a savepoint |
| `time_loop_detect` | Detect time loop patterns from repeated similar events |
| `time_wormhole_create` | Connect two distant points in time with a causal wormhole |

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
