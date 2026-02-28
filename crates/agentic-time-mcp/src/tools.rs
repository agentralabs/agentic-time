//! MCP tool definitions for AgenticTime.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde_json::{json, Value};

/// A tool definition for the MCP protocol.
pub struct ToolDefinition {
    /// Tool name.
    pub name: &'static str,
    /// Tool description.
    pub description: &'static str,
    /// JSON Schema for the tool input.
    pub input_schema: &'static str,
}

/// All AgenticTime MCP tools.
pub const TOOLS: &[ToolDefinition] = &[
    // Deadline tools
    ToolDefinition {
        name: "time_deadline_add",
        description: "Add a new deadline with optional warning threshold and consequences",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string","description":"What this deadline is for"},"due_at":{"type":"string","format":"date-time","description":"Deadline timestamp (ISO 8601)"},"warn_at":{"type":"string","format":"date-time","description":"Optional warning threshold"},"deadline_type":{"type":"string","enum":["hard","soft","target","recurring"],"default":"soft"},"consequence":{"type":"string","description":"What happens if missed"},"tags":{"type":"array","items":{"type":"string"}}},"required":["label","due_at"]}"#,
    },
    ToolDefinition {
        name: "time_deadline_complete",
        description: "Mark a deadline as completed",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string","description":"Deadline ID"}},"required":["id"]}"#,
    },
    ToolDefinition {
        name: "time_deadline_list",
        description: "List deadlines with optional filters",
        input_schema: r#"{"type":"object","properties":{"status":{"type":"string","enum":["pending","warning","overdue","completed","cancelled"]},"due_within":{"type":"string","description":"ISO 8601 duration (e.g. P7D for 7 days)"},"tags":{"type":"array","items":{"type":"string"}}}}"#,
    },
    ToolDefinition {
        name: "time_deadline_overdue",
        description: "List all overdue deadlines",
        input_schema: r#"{"type":"object","properties":{}}"#,
    },
    // Schedule tools
    ToolDefinition {
        name: "time_schedule_create",
        description: "Create a new scheduled event",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string"},"start_at":{"type":"string","format":"date-time"},"duration_minutes":{"type":"integer","description":"Duration in minutes"},"priority":{"type":"string","enum":["critical","high","medium","low","optional"],"default":"medium"},"flexible":{"type":"boolean","default":true},"recurrence":{"type":"object","properties":{"pattern":{"type":"string","enum":["daily","weekly","monthly"]},"interval":{"type":"integer","default":1}}},"tags":{"type":"array","items":{"type":"string"}}},"required":["label","start_at","duration_minutes"]}"#,
    },
    ToolDefinition {
        name: "time_schedule_reschedule",
        description: "Reschedule an event to a new time",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string"},"new_start_at":{"type":"string","format":"date-time"}},"required":["id","new_start_at"]}"#,
    },
    ToolDefinition {
        name: "time_schedule_range",
        description: "Get schedule for a time range",
        input_schema: r#"{"type":"object","properties":{"start":{"type":"string","format":"date-time"},"end":{"type":"string","format":"date-time"}},"required":["start","end"]}"#,
    },
    ToolDefinition {
        name: "time_schedule_available",
        description: "Find available time slots in a range",
        input_schema: r#"{"type":"object","properties":{"start":{"type":"string","format":"date-time"},"end":{"type":"string","format":"date-time"},"min_duration_minutes":{"type":"integer","default":30}},"required":["start","end"]}"#,
    },
    ToolDefinition {
        name: "time_schedule_conflicts",
        description: "Check for scheduling conflicts",
        input_schema: r#"{"type":"object","properties":{"start_at":{"type":"string","format":"date-time"},"duration_minutes":{"type":"integer"}},"required":["start_at","duration_minutes"]}"#,
    },
    // Sequence tools
    ToolDefinition {
        name: "time_sequence_create",
        description: "Create a sequence of ordered steps",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string"},"steps":{"type":"array","items":{"type":"object","properties":{"label":{"type":"string"},"duration_minutes":{"type":"integer"},"depends_on":{"type":"array","items":{"type":"integer"}}},"required":["label"]}},"allow_parallel":{"type":"boolean","default":false}},"required":["label","steps"]}"#,
    },
    ToolDefinition {
        name: "time_sequence_advance",
        description: "Complete the current step and advance",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string"}},"required":["id"]}"#,
    },
    ToolDefinition {
        name: "time_sequence_status",
        description: "Get the current status of a sequence",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string"}},"required":["id"]}"#,
    },
    // Decay tools
    ToolDefinition {
        name: "time_decay_create",
        description: "Create a decay model tracking value over time",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string"},"initial_value":{"type":"number"},"decay_type":{"type":"string","enum":["linear","exponential","half_life","step"],"default":"exponential"},"rate":{"type":"number","description":"Decay rate (for linear/exponential)"},"half_life_hours":{"type":"number","description":"Half-life in hours (for half_life type)"},"floor":{"type":"number","default":0},"tags":{"type":"array","items":{"type":"string"}}},"required":["label","initial_value"]}"#,
    },
    ToolDefinition {
        name: "time_decay_value",
        description: "Get the current value of a decay model",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string"}},"required":["id"]}"#,
    },
    ToolDefinition {
        name: "time_decay_alert",
        description: "Get decay models approaching a threshold",
        input_schema: r#"{"type":"object","properties":{"threshold":{"type":"number"},"within_hours":{"type":"number"}},"required":["threshold","within_hours"]}"#,
    },
    // Duration tools
    ToolDefinition {
        name: "time_duration_estimate",
        description: "Create a duration estimate with uncertainty (PERT model)",
        input_schema: r#"{"type":"object","properties":{"label":{"type":"string"},"expected_minutes":{"type":"integer"},"optimistic_minutes":{"type":"integer"},"pessimistic_minutes":{"type":"integer"},"confidence":{"type":"number","minimum":0,"maximum":1,"default":0.7},"tags":{"type":"array","items":{"type":"string"}}},"required":["label","expected_minutes"]}"#,
    },
    ToolDefinition {
        name: "time_duration_aggregate",
        description: "Aggregate multiple duration estimates",
        input_schema: r#"{"type":"object","properties":{"ids":{"type":"array","items":{"type":"string"}}},"required":["ids"]}"#,
    },
    // General tools
    ToolDefinition {
        name: "time_stats",
        description: "Get temporal statistics (counts by type and status)",
        input_schema: r#"{"type":"object","properties":{}}"#,
    },
    ToolDefinition {
        name: "time_refresh",
        description: "Refresh all statuses (update overdue deadlines, recalculate decays)",
        input_schema: r#"{"type":"object","properties":{}}"#,
    },
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a required string param, returning Err(String) on missing/null.
fn require_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required parameter: {}", key))
}

/// Parse a required RFC 3339 / ISO 8601 datetime param.
fn require_datetime(args: &Value, key: &str) -> Result<DateTime<Utc>, String> {
    let s = require_str(args, key)?;
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid datetime for '{}': {}", key, e))
}

/// Parse an optional RFC 3339 datetime param.
fn optional_datetime(args: &Value, key: &str) -> Result<Option<DateTime<Utc>>, String> {
    match args.get(key).and_then(|v| v.as_str()) {
        Some(s) => DateTime::parse_from_rfc3339(s)
            .map(|dt| Some(dt.with_timezone(&Utc)))
            .map_err(|e| format!("Invalid datetime for '{}': {}", key, e)),
        None => Ok(None),
    }
}

/// Parse a required TemporalId param.
fn require_id(args: &Value, key: &str) -> Result<agentic_time::TemporalId, String> {
    let s = require_str(args, key)?;
    s.parse::<agentic_time::TemporalId>()
        .map_err(|e| format!("Invalid ID for '{}': {}", key, e))
}

/// Parse optional string tags array.
fn parse_tags(args: &Value) -> Vec<String> {
    args.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Handle tool call
// ---------------------------------------------------------------------------

/// Handle a tool call.
pub async fn handle_tool_call(
    name: &str,
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    match name {
        // ==================================================================
        // DEADLINE TOOLS
        // ==================================================================
        "time_deadline_add" => {
            let label = require_str(&args, "label")?;
            let due_at = require_datetime(&args, "due_at")?;
            let warn_at = optional_datetime(&args, "warn_at")?;
            let tags = parse_tags(&args);

            let deadline_type = match args.get("deadline_type").and_then(|v| v.as_str()) {
                Some("hard") => agentic_time::DeadlineType::Hard,
                Some("soft") | None => agentic_time::DeadlineType::Soft,
                Some("target") => agentic_time::DeadlineType::Target,
                Some("recurring") => agentic_time::DeadlineType::Recurring,
                Some(other) => return Err(format!("Unknown deadline_type: {}", other)),
            };

            let consequence = match args.get("consequence").and_then(|v| v.as_str()) {
                Some(desc) if !desc.is_empty() => agentic_time::Consequence::Moderate {
                    description: desc.to_string(),
                },
                _ => agentic_time::Consequence::None,
            };

            let mut deadline = agentic_time::Deadline::new(label, due_at);
            deadline.warn_at = warn_at;
            deadline.deadline_type = deadline_type;
            deadline.consequence = consequence;
            deadline.tags = tags;

            let id = engine.add_deadline(deadline).map_err(|e| e.to_string())?;
            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "due_at": due_at.to_rfc3339(),
                "status": "pending"
            }))
        }

        "time_deadline_complete" => {
            let id = require_id(&args, "id")?;
            engine.complete_deadline(&id).map_err(|e| e.to_string())?;
            Ok(json!({
                "id": id.to_string(),
                "status": "completed"
            }))
        }

        "time_deadline_list" => {
            let query = agentic_time::QueryEngine::new(engine.file());

            let deadlines: Vec<agentic_time::Deadline> =
                if let Some(status_str) = args.get("status").and_then(|v| v.as_str()) {
                    let status = match status_str {
                        "pending" => agentic_time::DeadlineStatus::Pending,
                        "warning" => agentic_time::DeadlineStatus::Warning,
                        "overdue" => agentic_time::DeadlineStatus::Overdue,
                        "completed" => agentic_time::DeadlineStatus::Completed,
                        "cancelled" => agentic_time::DeadlineStatus::Cancelled,
                        other => return Err(format!("Unknown deadline status: {}", other)),
                    };
                    query
                        .deadlines_by_status(status)
                        .map_err(|e| e.to_string())?
                } else if let Some(dur_str) = args.get("due_within").and_then(|v| v.as_str()) {
                    // Simple ISO 8601 duration parser for common cases: PnD, PnH, PnM
                    let duration = parse_iso_duration(dur_str)?;
                    query
                        .deadlines_due_within(duration)
                        .map_err(|e| e.to_string())?
                } else {
                    // Return all deadlines
                    let mut all = Vec::new();
                    for block in engine
                        .file()
                        .list_by_type(agentic_time::EntityType::Deadline)
                    {
                        let d: agentic_time::Deadline =
                            block.deserialize().map_err(|e| e.to_string())?;
                        all.push(d);
                    }
                    all.sort_by_key(|d| d.due_at);
                    all
                };

            let items: Vec<Value> = deadlines
                .iter()
                .map(|d| {
                    json!({
                        "id": d.id.to_string(),
                        "label": d.label,
                        "due_at": d.due_at.to_rfc3339(),
                        "status": format!("{:?}", d.status),
                        "deadline_type": format!("{:?}", d.deadline_type),
                        "tags": d.tags,
                    })
                })
                .collect();

            Ok(json!({ "deadlines": items, "count": items.len() }))
        }

        "time_deadline_overdue" => {
            let query = agentic_time::QueryEngine::new(engine.file());
            let overdue = query.overdue_deadlines().map_err(|e| e.to_string())?;

            let items: Vec<Value> = overdue
                .iter()
                .map(|d| {
                    json!({
                        "id": d.id.to_string(),
                        "label": d.label,
                        "due_at": d.due_at.to_rfc3339(),
                        "overdue_by_secs": d.overdue_by().map(|dur| dur.num_seconds()),
                        "tags": d.tags,
                    })
                })
                .collect();

            Ok(json!({ "overdue": items, "count": items.len() }))
        }

        // ==================================================================
        // SCHEDULE TOOLS
        // ==================================================================
        "time_schedule_create" => {
            let label = require_str(&args, "label")?;
            let start_at = require_datetime(&args, "start_at")?;
            let duration_minutes = args
                .get("duration_minutes")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| "Missing required parameter: duration_minutes".to_string())?;
            let duration_secs = duration_minutes * 60;
            let tags = parse_tags(&args);

            let priority = match args.get("priority").and_then(|v| v.as_str()) {
                Some("critical") => agentic_time::Priority::Critical,
                Some("high") => agentic_time::Priority::High,
                Some("medium") | None => agentic_time::Priority::Medium,
                Some("low") => agentic_time::Priority::Low,
                Some("optional") => agentic_time::Priority::Optional,
                Some(other) => return Err(format!("Unknown priority: {}", other)),
            };

            let flexible = args
                .get("flexible")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let mut schedule = agentic_time::Schedule::new(label, start_at, duration_secs);
            schedule.priority = priority;
            schedule.flexible = flexible;
            schedule.tags = tags;

            // Handle recurrence if provided
            if let Some(rec) = args.get("recurrence") {
                if let Some(pattern_str) = rec.get("pattern").and_then(|v| v.as_str()) {
                    let interval = rec.get("interval").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
                    let pattern = match pattern_str {
                        "daily" => agentic_time::RecurrencePattern::Daily { interval },
                        "weekly" => agentic_time::RecurrencePattern::Weekly {
                            interval,
                            days: vec![],
                        },
                        "monthly" => agentic_time::RecurrencePattern::Monthly {
                            interval,
                            day_of_month: start_at
                                .format("%d")
                                .to_string()
                                .parse::<u32>()
                                .unwrap_or(1),
                        },
                        other => return Err(format!("Unknown recurrence pattern: {}", other)),
                    };
                    schedule.recurrence = Some(agentic_time::Recurrence {
                        pattern,
                        ends: agentic_time::RecurrenceEnd::Never,
                        exceptions: vec![],
                    });
                }
            }

            let id = engine.add_schedule(schedule).map_err(|e| e.to_string())?;
            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "start_at": start_at.to_rfc3339(),
                "duration_minutes": duration_minutes,
                "status": "scheduled"
            }))
        }

        "time_schedule_reschedule" => {
            let id = require_id(&args, "id")?;
            let new_start_at = require_datetime(&args, "new_start_at")?;

            engine
                .reschedule(&id, new_start_at)
                .map_err(|e| e.to_string())?;

            Ok(json!({
                "id": id.to_string(),
                "new_start_at": new_start_at.to_rfc3339(),
                "status": "rescheduled"
            }))
        }

        "time_schedule_range" => {
            let start = require_datetime(&args, "start")?;
            let end = require_datetime(&args, "end")?;

            let query = agentic_time::QueryEngine::new(engine.file());
            let schedules = query
                .schedules_in_range(start, end)
                .map_err(|e| e.to_string())?;

            let items: Vec<Value> = schedules
                .iter()
                .map(|s| {
                    json!({
                        "id": s.id.to_string(),
                        "label": s.label,
                        "start_at": s.start_at.to_rfc3339(),
                        "end_at": s.end_at().to_rfc3339(),
                        "duration_minutes": s.duration_secs / 60,
                        "priority": format!("{:?}", s.priority),
                        "status": format!("{:?}", s.status),
                        "tags": s.tags,
                    })
                })
                .collect();

            Ok(json!({ "schedules": items, "count": items.len() }))
        }

        "time_schedule_available" => {
            let start = require_datetime(&args, "start")?;
            let end = require_datetime(&args, "end")?;
            let min_duration_minutes = args
                .get("min_duration_minutes")
                .and_then(|v| v.as_i64())
                .unwrap_or(30);
            let min_duration = ChronoDuration::minutes(min_duration_minutes);

            let query = agentic_time::QueryEngine::new(engine.file());
            let slots = query
                .available_slots(start, end, min_duration)
                .map_err(|e| e.to_string())?;

            let items: Vec<Value> = slots
                .iter()
                .map(|s| {
                    json!({
                        "start": s.start.to_rfc3339(),
                        "end": s.end.to_rfc3339(),
                        "duration_minutes": s.duration_secs / 60,
                    })
                })
                .collect();

            Ok(json!({ "available_slots": items, "count": items.len() }))
        }

        "time_schedule_conflicts" => {
            let start_at = require_datetime(&args, "start_at")?;
            let duration_minutes = args
                .get("duration_minutes")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| "Missing required parameter: duration_minutes".to_string())?;
            let duration_secs = duration_minutes * 60;

            // Create a temporary schedule to check conflicts against
            let probe = agentic_time::Schedule::new("__conflict_check__", start_at, duration_secs);

            let query = agentic_time::QueryEngine::new(engine.file());
            let conflicts = query
                .schedule_conflicts(&probe)
                .map_err(|e| e.to_string())?;

            let items: Vec<Value> = conflicts
                .iter()
                .map(|s| {
                    json!({
                        "id": s.id.to_string(),
                        "label": s.label,
                        "start_at": s.start_at.to_rfc3339(),
                        "end_at": s.end_at().to_rfc3339(),
                        "priority": format!("{:?}", s.priority),
                    })
                })
                .collect();

            Ok(json!({ "conflicts": items, "count": items.len() }))
        }

        // ==================================================================
        // SEQUENCE TOOLS
        // ==================================================================
        "time_sequence_create" => {
            let label = require_str(&args, "label")?;
            let steps_val = args
                .get("steps")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Missing required parameter: steps".to_string())?;

            let allow_parallel = args
                .get("allow_parallel")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let mut steps = Vec::new();
            for (i, step_val) in steps_val.iter().enumerate() {
                let step_label = step_val
                    .get("label")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| format!("Missing label for step {}", i))?;

                let mut step = agentic_time::SequenceStep::new(step_label, i as u32);

                if let Some(dur) = step_val.get("duration_minutes").and_then(|v| v.as_i64()) {
                    step.duration_secs = Some(dur * 60);
                }

                if let Some(deps) = step_val.get("depends_on").and_then(|v| v.as_array()) {
                    step.depends_on = deps
                        .iter()
                        .filter_map(|v| v.as_u64().map(|n| n as u32))
                        .collect();
                }

                steps.push(step);
            }

            let mut sequence = agentic_time::Sequence::new(label, steps);
            sequence.allow_parallel = allow_parallel;

            let step_labels: Vec<&str> = steps_val
                .iter()
                .filter_map(|s| s.get("label").and_then(|v| v.as_str()))
                .collect();

            let id = engine.add_sequence(sequence).map_err(|e| e.to_string())?;

            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "steps": step_labels,
                "step_count": step_labels.len(),
                "status": "NotStarted"
            }))
        }

        "time_sequence_advance" => {
            let id = require_id(&args, "id")?;
            engine.advance_sequence(&id).map_err(|e| e.to_string())?;

            // Read back the updated sequence for status
            let seq: agentic_time::Sequence = engine
                .file()
                .get(&id)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("Sequence not found: {}", id))?;

            let current_step_label = seq
                .steps
                .get(seq.current_step)
                .map(|s| s.label.as_str())
                .unwrap_or("(completed)");

            Ok(json!({
                "id": id.to_string(),
                "status": format!("{:?}", seq.status),
                "current_step": seq.current_step,
                "current_step_label": current_step_label,
                "total_steps": seq.steps.len(),
            }))
        }

        "time_sequence_status" => {
            let id = require_id(&args, "id")?;
            let seq: agentic_time::Sequence = engine
                .file()
                .get(&id)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("Sequence not found: {}", id))?;

            let steps: Vec<Value> = seq
                .steps
                .iter()
                .map(|s| {
                    json!({
                        "order": s.order,
                        "label": s.label,
                        "status": format!("{:?}", s.status),
                        "duration_minutes": s.duration_secs.map(|d| d / 60),
                        "depends_on": s.depends_on,
                    })
                })
                .collect();

            Ok(json!({
                "id": id.to_string(),
                "label": seq.label,
                "status": format!("{:?}", seq.status),
                "current_step": seq.current_step,
                "total_steps": seq.steps.len(),
                "steps": steps,
                "total_duration_minutes": seq.total_duration().num_minutes(),
            }))
        }

        // ==================================================================
        // DECAY TOOLS
        // ==================================================================
        "time_decay_create" => {
            let label = require_str(&args, "label")?;
            let initial_value = args
                .get("initial_value")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| "Missing required parameter: initial_value".to_string())?;
            let tags = parse_tags(&args);

            let decay_type_str = args
                .get("decay_type")
                .and_then(|v| v.as_str())
                .unwrap_or("exponential");

            let decay_type = match decay_type_str {
                "linear" => {
                    let rate = args.get("rate").and_then(|v| v.as_f64()).unwrap_or(0.001);
                    agentic_time::DecayType::Linear { rate }
                }
                "exponential" => {
                    let lambda = args.get("rate").and_then(|v| v.as_f64()).unwrap_or(0.001);
                    agentic_time::DecayType::Exponential { lambda }
                }
                "half_life" => {
                    let half_life_hours = args
                        .get("half_life_hours")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(24.0);
                    let half_life_secs = (half_life_hours * 3600.0) as i64;
                    agentic_time::DecayType::HalfLife { half_life_secs }
                }
                "step" => {
                    let drop_amount = args.get("rate").and_then(|v| v.as_f64()).unwrap_or(1.0);
                    let interval_hours = args
                        .get("half_life_hours")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(1.0);
                    agentic_time::DecayType::Step {
                        drop_amount,
                        interval_secs: (interval_hours * 3600.0) as i64,
                    }
                }
                other => return Err(format!("Unknown decay_type: {}", other)),
            };

            let floor = args.get("floor").and_then(|v| v.as_f64()).unwrap_or(0.0);

            let mut model = agentic_time::DecayModel::new(label, initial_value, decay_type);
            model.floor = floor;
            model.tags = tags;

            let id = engine.add_decay(model).map_err(|e| e.to_string())?;
            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "initial_value": initial_value,
                "decay_type": decay_type_str,
                "floor": floor,
            }))
        }

        "time_decay_value" => {
            let id = require_id(&args, "id")?;
            let current_value = engine.refresh_decay(&id).map_err(|e| e.to_string())?;

            // Read back full model for extra context
            let model: agentic_time::DecayModel = engine
                .file()
                .get(&id)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("Decay model not found: {}", id))?;

            Ok(json!({
                "id": id.to_string(),
                "label": model.label,
                "initial_value": model.initial_value,
                "current_value": current_value,
                "floor": model.floor,
                "last_calculated": model.last_calculated.to_rfc3339(),
            }))
        }

        "time_decay_alert" => {
            let threshold = args
                .get("threshold")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| "Missing required parameter: threshold".to_string())?;
            let within_hours = args
                .get("within_hours")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| "Missing required parameter: within_hours".to_string())?;
            let within = ChronoDuration::seconds((within_hours * 3600.0) as i64);

            let query = agentic_time::QueryEngine::new(engine.file());

            // Get decays currently below threshold
            let below = query
                .decays_below_threshold(threshold)
                .map_err(|e| e.to_string())?;

            // Get decays that will reach threshold within the timeframe
            let reaching = query
                .decays_reaching_threshold(threshold, within)
                .map_err(|e| e.to_string())?;

            let below_items: Vec<Value> = below
                .iter()
                .map(|d| {
                    json!({
                        "id": d.id.to_string(),
                        "label": d.label,
                        "current_value": d.current_value,
                        "already_below": true,
                    })
                })
                .collect();

            let reaching_items: Vec<Value> = reaching
                .iter()
                .map(|(d, time_until)| {
                    json!({
                        "id": d.id.to_string(),
                        "label": d.label,
                        "current_value": d.current_value,
                        "reaches_threshold_in_hours": time_until.num_minutes() as f64 / 60.0,
                    })
                })
                .collect();

            Ok(json!({
                "threshold": threshold,
                "within_hours": within_hours,
                "already_below": below_items,
                "approaching": reaching_items,
                "total_alerts": below_items.len() + reaching_items.len(),
            }))
        }

        // ==================================================================
        // DURATION TOOLS
        // ==================================================================
        "time_duration_estimate" => {
            let label = require_str(&args, "label")?;
            let expected_minutes = args
                .get("expected_minutes")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| "Missing required parameter: expected_minutes".to_string())?;

            // Optimistic defaults to 60% of expected, pessimistic to 200%
            let optimistic_minutes = args
                .get("optimistic_minutes")
                .and_then(|v| v.as_i64())
                .unwrap_or((expected_minutes as f64 * 0.6) as i64);
            let pessimistic_minutes = args
                .get("pessimistic_minutes")
                .and_then(|v| v.as_i64())
                .unwrap_or(expected_minutes * 2);

            let confidence = args
                .get("confidence")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.7);
            let tags = parse_tags(&args);

            let optimistic_secs = optimistic_minutes * 60;
            let expected_secs = expected_minutes * 60;
            let pessimistic_secs = pessimistic_minutes * 60;

            let mut estimate = agentic_time::DurationEstimate::new(
                label,
                optimistic_secs,
                expected_secs,
                pessimistic_secs,
            );
            estimate.confidence = confidence;
            estimate.tags = tags;

            let pert_secs = estimate.pert_estimate().num_seconds();
            let std_dev_secs = estimate.std_deviation().num_seconds();

            let id = engine.add_duration(estimate).map_err(|e| e.to_string())?;

            Ok(json!({
                "id": id.to_string(),
                "label": label,
                "optimistic_minutes": optimistic_minutes,
                "expected_minutes": expected_minutes,
                "pessimistic_minutes": pessimistic_minutes,
                "pert_estimate_minutes": pert_secs / 60,
                "std_deviation_minutes": std_dev_secs / 60,
                "confidence": confidence,
            }))
        }

        "time_duration_aggregate" => {
            let ids_val = args
                .get("ids")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Missing required parameter: ids".to_string())?;

            let ids: Vec<agentic_time::TemporalId> = ids_val
                .iter()
                .filter_map(|v| {
                    v.as_str()
                        .and_then(|s| s.parse::<agentic_time::TemporalId>().ok())
                })
                .collect();

            if ids.is_empty() {
                return Err("No valid IDs provided".to_string());
            }

            let query = agentic_time::QueryEngine::new(engine.file());
            let total = query.estimate_total(&ids).map_err(|e| e.to_string())?;

            let pert_secs = total.pert_estimate().num_seconds();
            let std_dev_secs = total.std_deviation().num_seconds();

            Ok(json!({
                "aggregated_count": ids.len(),
                "optimistic_minutes": total.optimistic_secs / 60,
                "expected_minutes": total.expected_secs / 60,
                "pessimistic_minutes": total.pessimistic_secs / 60,
                "pert_estimate_minutes": pert_secs / 60,
                "std_deviation_minutes": std_dev_secs / 60,
            }))
        }

        // ==================================================================
        // GENERAL TOOLS
        // ==================================================================
        "time_stats" => {
            let query = agentic_time::QueryEngine::new(engine.file());
            let stats = query.stats().map_err(|e| e.to_string())?;
            serde_json::to_value(&stats).map_err(|e| e.to_string())
        }

        "time_refresh" => {
            let report = engine.update_all_statuses().map_err(|e| e.to_string())?;

            let new_overdue_ids: Vec<String> =
                report.new_overdue.iter().map(|id| id.to_string()).collect();

            Ok(json!({
                "deadlines_updated": report.deadlines_updated,
                "decays_updated": report.decays_updated,
                "new_overdue": new_overdue_ids,
                "new_overdue_count": new_overdue_ids.len(),
            }))
        }

        _ => Err(format!("Unknown tool: {}", name)),
    }
}

// ---------------------------------------------------------------------------
// ISO 8601 duration parser (simple subset)
// ---------------------------------------------------------------------------

/// Parse a simple ISO 8601 duration string (e.g., "P7D", "PT2H", "PT30M", "P1DT6H").
fn parse_iso_duration(s: &str) -> Result<ChronoDuration, String> {
    let s = s.trim();
    if !s.starts_with('P') {
        return Err(format!("Invalid ISO 8601 duration: {}", s));
    }

    let rest = &s[1..];
    let mut total_secs: i64 = 0;
    let mut in_time = false;
    let mut num_buf = String::new();

    for ch in rest.chars() {
        match ch {
            'T' => {
                in_time = true;
            }
            '0'..='9' | '.' => {
                num_buf.push(ch);
            }
            'D' if !in_time => {
                let n: f64 = num_buf
                    .parse()
                    .map_err(|_| format!("Invalid number in duration: {}", num_buf))?;
                total_secs += (n * 86400.0) as i64;
                num_buf.clear();
            }
            'W' if !in_time => {
                let n: f64 = num_buf
                    .parse()
                    .map_err(|_| format!("Invalid number in duration: {}", num_buf))?;
                total_secs += (n * 604800.0) as i64;
                num_buf.clear();
            }
            'H' if in_time => {
                let n: f64 = num_buf
                    .parse()
                    .map_err(|_| format!("Invalid number in duration: {}", num_buf))?;
                total_secs += (n * 3600.0) as i64;
                num_buf.clear();
            }
            'M' if in_time => {
                let n: f64 = num_buf
                    .parse()
                    .map_err(|_| format!("Invalid number in duration: {}", num_buf))?;
                total_secs += (n * 60.0) as i64;
                num_buf.clear();
            }
            'S' if in_time => {
                let n: f64 = num_buf
                    .parse()
                    .map_err(|_| format!("Invalid number in duration: {}", num_buf))?;
                total_secs += n as i64;
                num_buf.clear();
            }
            _ => return Err(format!("Unexpected character '{}' in duration: {}", ch, s)),
        }
    }

    if total_secs == 0 && !s.is_empty() {
        // Handle bare "P" with no units — treat leftover digits as days
        if !num_buf.is_empty() {
            let n: f64 = num_buf
                .parse()
                .map_err(|_| format!("Invalid number in duration: {}", num_buf))?;
            total_secs = (n * 86400.0) as i64;
        }
    }

    Ok(ChronoDuration::seconds(total_secs))
}
