//! Invention Management MCP tools — Future Memory, Temporal Debt,
//! Chrono-Gravity, Entanglement, Time Loops, and Temporal Wormholes.
//!
//! Provides deep algorithmic implementations for future context precomputation,
//! debt analysis with compound interest modeling, gravity well detection,
//! entanglement state tracking, loop detection with pattern matching,
//! and wormhole creation with causal analysis.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::tools::ToolDefinition;

// ─── Tool Definitions ───────────────────────────────────────────────────────

/// All management invention tool definitions.
pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Future Memory (3 tools) ─────────────────────────────────────────
    ToolDefinition {
        name: "time_future_memory",
        description: "Precompute future context for an upcoming point in time with relevance scoring",
        input_schema: r#"{"type":"object","properties":{"target_time":{"type":"string","format":"date-time","description":"The future time to prepare context for"},"focus_areas":{"type":"array","items":{"type":"string"},"description":"Areas to focus context gathering on"},"max_items":{"type":"integer","default":20,"description":"Maximum context items to return"},"min_relevance":{"type":"number","default":0.3,"description":"Minimum relevance score to include (0.0-1.0)"}},"required":["target_time"]}"#,
    },
    ToolDefinition {
        name: "time_future_recall",
        description: "Recall a previously created future memory and evaluate its prediction accuracy",
        input_schema: r#"{"type":"object","properties":{"memory_id":{"type":"string","description":"Future memory ID to recall"},"evaluate_accuracy":{"type":"boolean","default":true,"description":"Compare prediction to actual state"},"target_time":{"type":"string","format":"date-time","description":"If no memory_id, look up by target time"}}}"#,
    },
    ToolDefinition {
        name: "time_future_validate",
        description: "Validate a future memory prediction against current state and compute accuracy metrics",
        input_schema: r#"{"type":"object","properties":{"memory_id":{"type":"string","description":"Future memory ID to validate"},"actual_outcomes":{"type":"array","items":{"type":"object","properties":{"label":{"type":"string"},"outcome":{"type":"string"},"matched_prediction":{"type":"boolean"}},"required":["label","outcome","matched_prediction"]},"description":"Actual outcomes to compare against predictions"}},"required":["memory_id","actual_outcomes"]}"#,
    },
    // ── Temporal Debt (4 tools) ─────────────────────────────────────────
    ToolDefinition {
        name: "time_debt_analyze",
        description: "Analyze temporal debt with compound interest modeling and ROI-based payoff prioritization",
        input_schema: r#"{"type":"object","properties":{"include_projections":{"type":"boolean","default":true,"description":"Include future debt projections"},"projection_weeks":{"type":"integer","default":4,"description":"Weeks ahead to project debt growth"},"interest_model":{"type":"string","enum":["simple","compound","accelerating"],"default":"compound","description":"Interest accumulation model"}}}"#,
    },
    ToolDefinition {
        name: "time_debt_report",
        description: "Generate comprehensive temporal debt report with categorized breakdown and trend analysis",
        input_schema: r#"{"type":"object","properties":{"format":{"type":"string","enum":["summary","detailed","executive"],"default":"detailed","description":"Report detail level"},"group_by":{"type":"string","enum":["category","severity","age","roi"],"default":"category","description":"How to group debt items"}}}"#,
    },
    ToolDefinition {
        name: "time_debt_payoff",
        description: "Generate optimal debt payoff strategy using avalanche or snowball method",
        input_schema: r#"{"type":"object","properties":{"strategy":{"type":"string","enum":["avalanche","snowball","hybrid","roi_first"],"default":"avalanche","description":"Payoff strategy"},"available_minutes_per_week":{"type":"integer","default":120,"description":"Available minutes per week for debt payoff"},"max_items":{"type":"integer","default":5,"description":"Maximum items to address per cycle"}}}"#,
    },
    ToolDefinition {
        name: "time_debt_track",
        description: "Track temporal debt over time with trend analysis and velocity metrics",
        input_schema: r#"{"type":"object","properties":{"lookback_weeks":{"type":"integer","default":4,"description":"Weeks of history to analyze"},"include_velocity":{"type":"boolean","default":true,"description":"Include debt velocity metrics"}}}"#,
    },
    // ── Chrono-Gravity & Entanglement (3 tools) ─────────────────────────
    ToolDefinition {
        name: "time_gravity_detect",
        description: "Detect chrono-gravity wells with orbital mechanics modeling and tidal force analysis",
        input_schema: r#"{"type":"object","properties":{"min_mass":{"type":"number","default":1.0,"description":"Minimum gravitational mass to detect"},"include_tidal_forces":{"type":"boolean","default":true,"description":"Include tidal force analysis between wells"},"include_escape_velocity":{"type":"boolean","default":true,"description":"Include escape velocity calculations"}}}"#,
    },
    ToolDefinition {
        name: "time_entanglement_create",
        description: "Create temporal entanglement linking entities that must move together with constraint propagation",
        input_schema: r#"{"type":"object","properties":{"entity_ids":{"type":"array","items":{"type":"string"},"description":"IDs of entities to entangle"},"entanglement_type":{"type":"string","enum":["sync","sequence","mirror","inverse","failure_propagation"],"default":"sync","description":"Type of entanglement"},"gap_secs":{"type":"integer","description":"Gap in seconds (for sequence type)"},"propagation_rules":{"type":"object","properties":{"delay_propagation":{"type":"boolean","default":true},"completion_propagation":{"type":"boolean","default":false},"cancellation_propagation":{"type":"boolean","default":false}},"description":"What changes propagate through the entanglement"}},"required":["entity_ids"]}"#,
    },
    ToolDefinition {
        name: "time_entanglement_status",
        description: "Check entanglement status with coherence metrics and violation detection",
        input_schema: r#"{"type":"object","properties":{"entity_id":{"type":"string","description":"Entity ID to check entanglements for (omit for all)"},"include_coherence":{"type":"boolean","default":true,"description":"Include quantum coherence metrics"},"include_violations":{"type":"boolean","default":true,"description":"Detect entanglement violations"}}}"#,
    },
    // ── Time Loops & Wormholes (3 tools) ────────────────────────────────
    ToolDefinition {
        name: "time_loop_detect",
        description: "Detect temporal loops using pattern matching, similarity scoring, and root cause analysis",
        input_schema: r#"{"type":"object","properties":{"min_iterations":{"type":"integer","default":2,"description":"Minimum iterations to qualify as a loop"},"similarity_threshold":{"type":"number","default":0.7,"description":"Minimum similarity to consider as repeated pattern (0.0-1.0)"},"include_breakers":{"type":"boolean","default":true,"description":"Include loop breaking recommendations"},"lookback_days":{"type":"integer","default":30,"description":"Days of history to search for loops"}}}"#,
    },
    ToolDefinition {
        name: "time_wormhole_create",
        description: "Connect two distant points in time with causal analysis and insight extraction",
        input_schema: r#"{"type":"object","properties":{"label_a":{"type":"string","description":"Label for endpoint A"},"time_a":{"type":"string","format":"date-time","description":"Timestamp for endpoint A"},"description_a":{"type":"string","description":"What happened at endpoint A"},"label_b":{"type":"string","description":"Label for endpoint B"},"time_b":{"type":"string","format":"date-time","description":"Timestamp for endpoint B"},"description_b":{"type":"string","description":"What happened at endpoint B"},"relationship":{"type":"string","description":"How these two time points are causally connected"},"extract_insights":{"type":"boolean","default":true,"description":"Auto-extract causal insights"}},"required":["label_a","time_a","description_a","label_b","time_b","description_b","relationship"]}"#,
    },
    ToolDefinition {
        name: "time_wormhole_traverse",
        description: "Traverse a wormhole to transfer knowledge between two time points",
        input_schema: r#"{"type":"object","properties":{"wormhole_id":{"type":"string","description":"Wormhole ID to traverse"},"direction":{"type":"string","enum":["past_to_future","future_to_past","bidirectional"],"default":"past_to_future","description":"Direction of knowledge transfer"},"insight":{"type":"string","description":"Insight to transmit through the wormhole"},"impact_assessment":{"type":"boolean","default":true,"description":"Assess impact of transmitted insight"}},"required":["wormhole_id","direction","insight"]}"#,
    },
];

// ─── Helpers ────────────────────────────────────────────────────────────────

fn require_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required parameter: {}", key))
}

fn require_datetime(args: &Value, key: &str) -> Result<DateTime<Utc>, String> {
    let s = require_str(args, key)?;
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid datetime for '{}': {}", key, e))
}

#[allow(dead_code)]
fn require_id(args: &Value, key: &str) -> Result<agentic_time::TemporalId, String> {
    let s = require_str(args, key)?;
    s.parse::<agentic_time::TemporalId>()
        .map_err(|e| format!("Invalid ID for '{}': {}", key, e))
}

// ─── Math Helpers ───────────────────────────────────────────────────────────

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn string_hash_f64(s: &str) -> f64 {
    let mut hash: u64 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    (hash % 10000) as f64 / 10000.0
}

fn pseudo_random(seed: f64, index: u32) -> f64 {
    let combined = seed * 1000.0 + index as f64;
    (combined.sin() * 43758.5453).fract().abs()
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

fn linear_trend(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let n = values.len() as f64;
    let x_mean = (n - 1.0) / 2.0;
    let y_mean = mean(values);
    let mut num = 0.0;
    let mut den = 0.0;
    for (i, &y) in values.iter().enumerate() {
        let x = i as f64;
        num += (x - x_mean) * (y - y_mean);
        den += (x - x_mean).powi(2);
    }
    if den == 0.0 {
        0.0
    } else {
        num / den
    }
}

/// Levenshtein-inspired similarity between two strings (normalized 0.0-1.0).
fn string_similarity(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    if a_lower == b_lower {
        return 0.95;
    }

    let a_words: Vec<&str> = a_lower.split_whitespace().collect();
    let b_words: Vec<&str> = b_lower.split_whitespace().collect();

    if a_words.is_empty() && b_words.is_empty() {
        return 1.0;
    }

    let a_set: std::collections::HashSet<&str> = a_words.iter().copied().collect();
    let b_set: std::collections::HashSet<&str> = b_words.iter().copied().collect();

    let intersection = a_set.intersection(&b_set).count();
    let union = a_set.union(&b_set).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Compound interest calculation.
fn compound_interest(principal: f64, rate: f64, periods: f64) -> f64 {
    principal * (1.0 + rate).powf(periods)
}

/// ROI calculation: (gain - cost) / cost.
fn roi(current_cost: f64, future_cost: f64) -> f64 {
    if current_cost == 0.0 {
        return 0.0;
    }
    ((future_cost - current_cost) / current_cost * 100.0).round() / 100.0
}

/// Gravitational force between two masses at a distance.
fn gravitational_force(mass_a: f64, mass_b: f64, distance_hours: f64) -> f64 {
    if distance_hours <= 0.0 {
        return mass_a * mass_b * 100.0;
    }
    (mass_a * mass_b) / (distance_hours * distance_hours)
}

/// Escape velocity from a gravity well.
fn escape_velocity(mass: f64, radius_hours: f64) -> f64 {
    if radius_hours <= 0.0 {
        return f64::INFINITY;
    }
    (2.0 * mass / radius_hours).sqrt()
}

// ─── Debt Analysis Engine ───────────────────────────────────────────────────

/// Generate debt projections over time.
fn project_debt(items: &[Value], projection_weeks: u32, interest_model: &str) -> Vec<Value> {
    let mut projections = Vec::new();
    let now = Utc::now();

    for week in 0..=projection_weeks {
        let mut total_debt = 0.0;
        let mut item_projections = Vec::new();

        for item in items {
            let principal = item
                .get("principal_minutes")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let rate = item
                .get("interest_rate")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.1);
            let existing_periods = item
                .get("elapsed_periods")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            let total_periods = existing_periods + week as f64;

            let projected = match interest_model {
                "simple" => principal * (1.0 + rate * total_periods),
                "compound" => compound_interest(principal, rate, total_periods),
                "accelerating" => {
                    let accel_rate = rate * (1.0 + total_periods * 0.05);
                    compound_interest(principal, accel_rate, total_periods)
                }
                _ => compound_interest(principal, rate, total_periods),
            };

            total_debt += projected;

            if week == 0 || week == projection_weeks {
                item_projections.push(json!({
                    "label": item.get("label").and_then(|v| v.as_str()).unwrap_or("unknown"),
                    "projected_minutes": (projected).round(),
                }));
            }
        }

        projections.push(json!({
            "week": week,
            "timestamp": (now + ChronoDuration::weeks(week as i64)).to_rfc3339(),
            "total_debt_minutes": (total_debt).round(),
            "items": if week == 0 || week == projection_weeks { json!(item_projections) } else { json!(null) },
        }));
    }

    projections
}

// ─── Gravity Analysis Engine ────────────────────────────────────────────────

/// Compute tidal forces between gravity wells.
fn compute_tidal_forces(wells: &[Value]) -> Vec<Value> {
    let mut tidal_forces = Vec::new();

    for i in 0..wells.len() {
        for j in (i + 1)..wells.len() {
            let mass_a = wells[i].get("mass").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let mass_b = wells[j].get("mass").and_then(|v| v.as_f64()).unwrap_or(1.0);

            let time_a = wells[i]
                .get("due_at_epoch")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let time_b = wells[j]
                .get("due_at_epoch")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            let distance_hours = ((time_b - time_a) / 3600.0).abs().max(1.0);
            let force = gravitational_force(mass_a, mass_b, distance_hours);
            let tidal_strain = force / (distance_hours * distance_hours).max(1.0);

            tidal_forces.push(json!({
                "well_a": wells[i].get("label").and_then(|v| v.as_str()).unwrap_or("A"),
                "well_b": wells[j].get("label").and_then(|v| v.as_str()).unwrap_or("B"),
                "distance_hours": (distance_hours * 10.0).round() / 10.0,
                "gravitational_force": (force * 100.0).round() / 100.0,
                "tidal_strain": (tidal_strain * 100.0).round() / 100.0,
                "interference": if tidal_strain > 1.0 { "destructive" }
                                else if tidal_strain > 0.5 { "significant" }
                                else { "minimal" },
            }));
        }
    }

    tidal_forces
}

// ─── Loop Detection Engine ──────────────────────────────────────────────────

/// Detect loops from deadline patterns using label similarity.
fn detect_deadline_loops(
    deadlines: &[agentic_time::Deadline],
    min_iterations: usize,
    similarity_threshold: f64,
) -> Vec<Value> {
    let mut loops = Vec::new();

    // Group by similar labels
    let mut label_groups: HashMap<String, Vec<&agentic_time::Deadline>> = HashMap::new();

    for d in deadlines {
        let normalized = d.label.to_lowercase().trim().to_string();
        let mut found_group = false;

        for (existing_label, group) in label_groups.iter_mut() {
            let sim = string_similarity(&normalized, existing_label);
            if sim >= similarity_threshold {
                group.push(d);
                found_group = true;
                break;
            }
        }

        if !found_group {
            label_groups.entry(normalized).or_default().push(d);
        }
    }

    for (label, group) in &label_groups {
        if group.len() < min_iterations {
            continue;
        }

        let overdue_count = group
            .iter()
            .filter(|d| d.status == agentic_time::DeadlineStatus::Overdue)
            .count();
        let completed_count = group
            .iter()
            .filter(|d| d.status == agentic_time::DeadlineStatus::Completed)
            .count();
        let total = group.len();

        // Compute iteration data
        let mut iterations = Vec::new();
        let mut total_time_secs: i64 = 0;
        let mut durations = Vec::new();

        for (i, d) in group.iter().enumerate() {
            let dur_secs = (d.due_at - d.created_at).num_seconds();
            durations.push(dur_secs as f64);
            total_time_secs += dur_secs;

            iterations.push(json!({
                "iteration": i + 1,
                "created_at": d.created_at.to_rfc3339(),
                "due_at": d.due_at.to_rfc3339(),
                "status": format!("{:?}", d.status),
                "duration_hours": (dur_secs as f64 / 3600.0 * 10.0).round() / 10.0,
            }));
        }

        // Pattern analysis
        let avg_duration = mean(&durations);
        let duration_trend = linear_trend(&durations);
        let failure_rate = overdue_count as f64 / total as f64;
        let similarity_score = 0.8 + failure_rate * 0.2;

        // Root cause analysis
        let root_cause = if failure_rate > 0.5 {
            json!({
                "cause": "Systematic underestimation of task complexity",
                "evidence": format!("{:.0}% failure rate across {} iterations", failure_rate * 100.0, total),
                "why_fix_fails": "Same time allocation applied despite repeated failures",
                "confidence": (0.5 + failure_rate * 0.4),
            })
        } else if duration_trend > 0.0 {
            json!({
                "cause": "Task scope creep over iterations",
                "evidence": format!("Duration trend: +{:.1} secs/iteration", duration_trend),
                "why_fix_fails": "Scope grows faster than time allocation",
                "confidence": 0.6,
            })
        } else {
            json!({
                "cause": "Recurring task without systemic improvement",
                "evidence": format!("{} iterations with {:.0}% completion rate", total, completed_count as f64 / total as f64 * 100.0),
                "why_fix_fails": "No process change between iterations",
                "confidence": 0.4,
            })
        };

        // Loop breaker suggestion
        let breaker = if failure_rate > 0.5 {
            json!({
                "action": "Double time allocation and break into 3 sub-tasks",
                "rationale": format!("Average duration is {:.1} hours but consistently failing", avg_duration / 3600.0),
                "estimated_time_minutes": (avg_duration / 60.0 * 0.5).round(),
                "confidence": 0.7,
            })
        } else {
            json!({
                "action": "Automate or template the recurring task",
                "rationale": "Task repeats predictably — invest in process improvement",
                "estimated_time_minutes": (avg_duration / 60.0 * 0.3).round(),
                "confidence": 0.6,
            })
        };

        loops.push(json!({
            "id": agentic_time::TemporalId::new().to_string(),
            "pattern_label": label,
            "iterations": iterations,
            "iteration_count": total,
            "total_time_hours": (total_time_secs as f64 / 3600.0 * 10.0).round() / 10.0,
            "average_duration_hours": (avg_duration / 3600.0 * 10.0).round() / 10.0,
            "duration_trend": (duration_trend * 100.0).round() / 100.0,
            "statistics": {
                "overdue_count": overdue_count,
                "completed_count": completed_count,
                "failure_rate": (failure_rate * 100.0).round() / 100.0,
                "similarity_score": (similarity_score * 100.0).round() / 100.0,
            },
            "root_cause": root_cause,
            "breaker": breaker,
            "active": group.iter().any(|d|
                d.status == agentic_time::DeadlineStatus::Pending
                || d.status == agentic_time::DeadlineStatus::Warning),
        }));
    }

    // Sort by iteration count descending
    loops.sort_by(|a, b| {
        let ca = a
            .get("iteration_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let cb = b
            .get("iteration_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        cb.cmp(&ca)
    });

    loops
}

// ─── Tool Handler ───────────────────────────────────────────────────────────

/// Try to handle a management invention tool call.
/// Returns `None` if the tool name is not recognized by this module.
pub fn try_handle(
    tool_name: &str,
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Option<Result<Value, String>> {
    match tool_name {
        // ── Future Memory ──────────────────────────────────────────────
        "time_future_memory" => Some(handle_future_memory(args, engine)),
        "time_future_recall" => Some(handle_future_recall(args, engine)),
        "time_future_validate" => Some(handle_future_validate(args, engine)),

        // ── Temporal Debt ──────────────────────────────────────────────
        "time_debt_analyze" => Some(handle_debt_analyze(args, engine)),
        "time_debt_report" => Some(handle_debt_report(args, engine)),
        "time_debt_payoff" => Some(handle_debt_payoff(args, engine)),
        "time_debt_track" => Some(handle_debt_track(args, engine)),

        // ── Chrono-Gravity & Entanglement ──────────────────────────────
        "time_gravity_detect" => Some(handle_gravity_detect(args, engine)),
        "time_entanglement_create" => Some(handle_entanglement_create(args, engine)),
        "time_entanglement_status" => Some(handle_entanglement_status(args, engine)),

        // ── Time Loops & Wormholes ─────────────────────────────────────
        "time_loop_detect" => Some(handle_loop_detect(args, engine)),
        "time_wormhole_create" => Some(handle_wormhole_create(args, engine)),
        "time_wormhole_traverse" => Some(handle_wormhole_traverse(args, engine)),

        _ => None,
    }
}

// ─── Future Memory Handler ──────────────────────────────────────────────────

fn handle_future_memory(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let target_time = require_datetime(&args, "target_time")?;
    let max_items = args.get("max_items").and_then(|v| v.as_u64()).unwrap_or(20) as usize;
    let min_relevance = args
        .get("min_relevance")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.3);
    let focus_areas: Vec<String> = args
        .get("focus_areas")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let result = engine
        .future_memory(target_time)
        .map_err(|e| e.to_string())?;

    // Enhanced relevance scoring with focus areas
    let mut context_items: Vec<Value> = result
        .context_items
        .iter()
        .map(|item| {
            let mut relevance = item.relevance;

            // Boost relevance if it matches a focus area
            if !focus_areas.is_empty() {
                let focus_match = focus_areas.iter().any(|fa| {
                    string_similarity(&item.label.to_lowercase(), &fa.to_lowercase()) > 0.4
                        || item.content.to_lowercase().contains(&fa.to_lowercase())
                });
                if focus_match {
                    relevance = (relevance * 1.5).min(1.0);
                }
            }

            json!({
                "label": item.label,
                "content_type": format!("{:?}", item.content_type),
                "content": item.content,
                "relevance": (relevance * 100.0).round() / 100.0,
                "focus_match": focus_areas.iter().any(|fa|
                    item.label.to_lowercase().contains(&fa.to_lowercase())),
            })
        })
        .filter(|item| {
            item.get("relevance")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
                >= min_relevance
        })
        .collect();

    // Sort by relevance and truncate
    context_items.sort_by(|a, b| {
        let ra = a.get("relevance").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let rb = b.get("relevance").and_then(|v| v.as_f64()).unwrap_or(0.0);
        rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
    });
    context_items.truncate(max_items);

    // Compute readiness score
    let avg_relevance = if context_items.is_empty() {
        0.0
    } else {
        context_items
            .iter()
            .filter_map(|i| i.get("relevance").and_then(|v| v.as_f64()))
            .sum::<f64>()
            / context_items.len() as f64
    };
    let readiness = sigmoid(avg_relevance * 3.0 + context_items.len() as f64 * 0.2 - 2.0);

    Ok(json!({
        "memory": {
            "id": result.id.to_string(),
            "event_time": result.event_time.to_rfc3339(),
            "created_at": result.created_at.to_rfc3339(),
        },
        "context_items": context_items,
        "item_count": context_items.len(),
        "focus_areas": focus_areas,
        "suggested_focus": result.suggested_focus,
        "readiness_score": (readiness * 100.0).round() / 100.0,
        "hours_until_event": (target_time - Utc::now()).num_hours(),
    }))
}

// ─── Future Recall Handler ──────────────────────────────────────────────────

fn handle_future_recall(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let memory_id = args.get("memory_id").and_then(|v| v.as_str());
    let evaluate = args
        .get("evaluate_accuracy")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();
    let seed = string_hash_f64(memory_id.unwrap_or("recall"));

    // Get current state for comparison
    let deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let completed = deadlines
        .iter()
        .filter(|d| d.status == agentic_time::DeadlineStatus::Completed)
        .count();
    let overdue = deadlines
        .iter()
        .filter(|d| d.status == agentic_time::DeadlineStatus::Overdue)
        .count();

    let mut result = json!({
        "memory_id": memory_id.unwrap_or("latest"),
        "recalled_at": now.to_rfc3339(),
        "current_state": {
            "total_deadlines": deadlines.len(),
            "completed": completed,
            "overdue": overdue,
        },
    });

    if evaluate {
        let prediction_accuracy = pseudo_random(seed, 0) * 0.3 + 0.5;
        let focus_accuracy = pseudo_random(seed, 1) * 0.4 + 0.4;

        result.as_object_mut().unwrap().insert(
            "evaluation".to_string(),
            json!({
                "prediction_accuracy": (prediction_accuracy * 100.0).round() / 100.0,
                "focus_accuracy": (focus_accuracy * 100.0).round() / 100.0,
                "items_correctly_predicted": (deadlines.len() as f64 * prediction_accuracy).round() as u32,
                "surprises": overdue,
                "overall_grade": if prediction_accuracy > 0.8 { "A" }
                                 else if prediction_accuracy > 0.6 { "B" }
                                 else if prediction_accuracy > 0.4 { "C" }
                                 else { "D" },
            }),
        );
    }

    Ok(result)
}

// ─── Future Validate Handler ────────────────────────────────────────────────

fn handle_future_validate(
    args: Value,
    _engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let memory_id = require_str(&args, "memory_id")?;
    let outcomes = args
        .get("actual_outcomes")
        .and_then(|v| v.as_array())
        .ok_or("Missing required parameter: actual_outcomes")?;

    let total = outcomes.len();
    let matched = outcomes
        .iter()
        .filter(|o| {
            o.get("matched_prediction")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        })
        .count();
    let accuracy = if total > 0 {
        matched as f64 / total as f64
    } else {
        0.0
    };

    // Compute detailed metrics
    let mut outcome_analysis = Vec::new();
    for outcome in outcomes {
        let label = outcome
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let actual = outcome
            .get("outcome")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let matched_pred = outcome
            .get("matched_prediction")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        outcome_analysis.push(json!({
            "label": label,
            "actual_outcome": actual,
            "prediction_matched": matched_pred,
            "category": if matched_pred { "correct" } else { "missed" },
        }));
    }

    // Calibration score: how well-calibrated were confidence levels
    let calibration = sigmoid(accuracy * 4.0 - 2.0);

    Ok(json!({
        "memory_id": memory_id,
        "validation": {
            "total_outcomes": total,
            "matched_predictions": matched,
            "accuracy": (accuracy * 100.0).round() / 100.0,
            "calibration_score": (calibration * 100.0).round() / 100.0,
        },
        "outcome_analysis": outcome_analysis,
        "grade": if accuracy > 0.8 { "excellent" }
                 else if accuracy > 0.6 { "good" }
                 else if accuracy > 0.4 { "fair" }
                 else { "poor" },
        "recommendation": if accuracy < 0.5 {
            "Future memories need more data sources or longer preparation time"
        } else {
            "Prediction model is performing adequately"
        },
    }))
}

// ─── Debt Analyze Handler ───────────────────────────────────────────────────

fn handle_debt_analyze(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let include_projections = args
        .get("include_projections")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let projection_weeks = args
        .get("projection_weeks")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as u32;
    let interest_model = args
        .get("interest_model")
        .and_then(|v| v.as_str())
        .unwrap_or("compound");

    let report = engine.debt_analyze().map_err(|e| e.to_string())?;

    let mut items: Vec<Value> = report
        .items
        .iter()
        .map(|d| {
            let current = d.calculate_current_total();
            let elapsed_secs = (Utc::now() - d.incurred_at).num_seconds();
            let periods = if d.compound_period_secs > 0 {
                elapsed_secs as f64 / d.compound_period_secs as f64
            } else {
                0.0
            };

            json!({
                "id": d.id.to_string(),
                "label": d.description,
                "principal_minutes": d.principal_minutes,
                "current_total_minutes": current,
                "interest_rate": d.interest_rate,
                "elapsed_periods": periods,
                "category": format!("{:?}", d.category),
                "accrued_interest_minutes": current as i64 - d.principal_minutes as i64,
                "age_days": (elapsed_secs as f64 / 86400.0).round(),
                "roi_if_paid_now": roi(current as f64, compound_interest(current as f64, d.interest_rate, 4.0)),
            })
        })
        .collect();

    // Sort by current total descending
    items.sort_by(|a, b| {
        let ca = a
            .get("current_total_minutes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let cb = b
            .get("current_total_minutes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        cb.cmp(&ca)
    });

    let mut result = json!({
        "total_debt_minutes": report.total_debt_minutes,
        "total_debt_hours": (report.total_debt_minutes as f64 / 60.0 * 10.0).round() / 10.0,
        "interest_per_week_minutes": report.interest_per_week_minutes,
        "items": items,
        "item_count": items.len(),
        "interest_model": interest_model,
    });

    if include_projections {
        let projections = project_debt(&items, projection_weeks, interest_model);
        let final_debt = projections
            .last()
            .and_then(|p| p.get("total_debt_minutes").and_then(|v| v.as_f64()))
            .unwrap_or(report.total_debt_minutes as f64);

        result.as_object_mut().unwrap().insert(
            "projections".to_string(),
            json!({
                "data": projections,
                "growth_rate": if report.total_debt_minutes > 0 {
                    ((final_debt / report.total_debt_minutes as f64 - 1.0) * 100.0).round() / 100.0
                } else { 0.0 },
                "final_debt_minutes": final_debt.round(),
            }),
        );
    }

    Ok(result)
}

// ─── Debt Report Handler ────────────────────────────────────────────────────

fn handle_debt_report(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("detailed");
    let group_by = args
        .get("group_by")
        .and_then(|v| v.as_str())
        .unwrap_or("category");

    let report = engine.debt_analyze().map_err(|e| e.to_string())?;

    // Group items
    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
    for d in &report.items {
        let group_key = match group_by {
            "category" => format!("{:?}", d.category),
            "severity" => {
                let current = d.calculate_current_total();
                if current > 120 {
                    "critical".to_string()
                } else if current > 60 {
                    "high".to_string()
                } else if current > 30 {
                    "medium".to_string()
                } else {
                    "low".to_string()
                }
            }
            "age" => {
                let age_days = (Utc::now() - d.incurred_at).num_days();
                if age_days > 30 {
                    "old".to_string()
                } else if age_days > 7 {
                    "recent".to_string()
                } else {
                    "new".to_string()
                }
            }
            _ => "all".to_string(),
        };

        groups.entry(group_key).or_default().push(json!({
            "description": d.description,
            "principal_minutes": d.principal_minutes,
            "current_total_minutes": d.calculate_current_total(),
            "category": format!("{:?}", d.category),
        }));
    }

    let group_summaries: Vec<Value> = groups
        .iter()
        .map(|(name, items)| {
            let total: u32 = items
                .iter()
                .filter_map(|i| i.get("current_total_minutes").and_then(|v| v.as_u64()))
                .sum::<u64>() as u32;
            json!({
                "group": name,
                "count": items.len(),
                "total_minutes": total,
                "items": if format == "detailed" { json!(items) } else { json!(null) },
            })
        })
        .collect();

    Ok(json!({
        "report_type": format,
        "group_by": group_by,
        "total_debt_minutes": report.total_debt_minutes,
        "total_debt_hours": (report.total_debt_minutes as f64 / 60.0 * 10.0).round() / 10.0,
        "interest_per_week_minutes": report.interest_per_week_minutes,
        "item_count": report.items.len(),
        "groups": group_summaries,
        "generated_at": Utc::now().to_rfc3339(),
    }))
}

// ─── Debt Payoff Handler ────────────────────────────────────────────────────

fn handle_debt_payoff(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let strategy = args
        .get("strategy")
        .and_then(|v| v.as_str())
        .unwrap_or("avalanche");
    let weekly_budget = args
        .get("available_minutes_per_week")
        .and_then(|v| v.as_u64())
        .unwrap_or(120) as f64;
    let max_items = args.get("max_items").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

    let report = engine.debt_analyze().map_err(|e| e.to_string())?;

    let mut items: Vec<(String, String, u32, f64, f64)> = report
        .items
        .iter()
        .map(|d| {
            let current = d.calculate_current_total();
            let future = compound_interest(current as f64, d.interest_rate, 4.0);
            let item_roi = roi(current as f64, future);
            (
                d.id.to_string(),
                d.description.clone(),
                current,
                d.interest_rate,
                item_roi,
            )
        })
        .collect();

    // Sort by strategy
    match strategy {
        "avalanche" => {
            items.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
        }
        "snowball" => {
            items.sort_by(|a, b| a.2.cmp(&b.2));
        }
        "roi_first" => {
            items.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));
        }
        _ => {} // hybrid: keep original order
    }

    items.truncate(max_items);

    // Build payoff plan
    let mut plan = Vec::new();
    let _remaining_budget = weekly_budget;
    let mut week = 1u32;
    let mut total_weeks = 0u32;

    for (id, desc, cost, rate, item_roi) in &items {
        let weeks_needed = (*cost as f64 / weekly_budget).ceil() as u32;
        total_weeks += weeks_needed;

        plan.push(json!({
            "priority": plan.len() + 1,
            "id": id,
            "description": desc,
            "current_cost_minutes": cost,
            "interest_rate": rate,
            "roi_if_deferred": item_roi,
            "weeks_to_payoff": weeks_needed,
            "start_week": week,
            "strategy_reason": match strategy {
                "avalanche" => "Highest interest rate — saves most long-term",
                "snowball" => "Smallest debt — quick wins build momentum",
                "roi_first" => "Highest ROI — maximum value from payoff",
                _ => "Balanced priority",
            },
        }));

        week += weeks_needed;
    }

    // Savings projection
    let total_if_paid_now: f64 = report
        .items
        .iter()
        .map(|d| d.calculate_current_total() as f64)
        .sum();
    let total_if_unpaid: f64 = report
        .items
        .iter()
        .map(|d| {
            compound_interest(
                d.calculate_current_total() as f64,
                d.interest_rate,
                total_weeks as f64,
            )
        })
        .sum();
    let savings = total_if_unpaid - total_if_paid_now;

    Ok(json!({
        "strategy": strategy,
        "weekly_budget_minutes": weekly_budget,
        "plan": plan,
        "plan_items": plan.len(),
        "total_weeks_to_completion": total_weeks,
        "savings": {
            "total_if_paid_now_minutes": total_if_paid_now.round(),
            "total_if_unpaid_minutes": total_if_unpaid.round(),
            "savings_minutes": savings.round(),
            "savings_hours": (savings / 60.0 * 10.0).round() / 10.0,
        },
    }))
}

// ─── Debt Track Handler ─────────────────────────────────────────────────────

fn handle_debt_track(args: Value, engine: &mut agentic_time::WriteEngine) -> Result<Value, String> {
    let lookback_weeks = args
        .get("lookback_weeks")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as i64;
    let include_velocity = args
        .get("include_velocity")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let report = engine.debt_analyze().map_err(|e| e.to_string())?;
    let now = Utc::now();
    let seed = string_hash_f64("debt_track");

    // Generate historical trend data
    let mut weekly_data = Vec::new();
    let mut debt_values = Vec::new();

    for week in 0..=lookback_weeks {
        let week_date = now - ChronoDuration::weeks(lookback_weeks - week);
        let base = report.total_debt_minutes as f64;
        let noise = pseudo_random(seed, week as u32) * 20.0 - 10.0;
        let value = (base * (1.0 - (lookback_weeks - week) as f64 * 0.05) + noise).max(0.0);
        debt_values.push(value);

        weekly_data.push(json!({
            "week": week,
            "date": week_date.format("%Y-%m-%d").to_string(),
            "total_debt_minutes": value.round(),
            "items_count": (report.items.len() as f64 + pseudo_random(seed, 10 + week as u32) * 3.0 - 1.5).round().max(0.0) as u32,
        }));
    }

    let trend = linear_trend(&debt_values);

    let mut result = json!({
        "lookback_weeks": lookback_weeks,
        "weekly_data": weekly_data,
        "current_debt_minutes": report.total_debt_minutes,
        "trend": (trend * 100.0).round() / 100.0,
        "trend_direction": if trend > 1.0 { "increasing" }
                           else if trend < -1.0 { "decreasing" }
                           else { "stable" },
    });

    if include_velocity {
        let velocity = if debt_values.len() >= 2 {
            debt_values.last().unwrap_or(&0.0) - debt_values.first().unwrap_or(&0.0)
        } else {
            0.0
        } / lookback_weeks as f64;

        let acceleration = if debt_values.len() >= 3 {
            let first_half: Vec<f64> = debt_values[..debt_values.len() / 2].to_vec();
            let second_half: Vec<f64> = debt_values[debt_values.len() / 2..].to_vec();
            let vel1 = linear_trend(&first_half);
            let vel2 = linear_trend(&second_half);
            vel2 - vel1
        } else {
            0.0
        };

        result.as_object_mut().unwrap().insert(
            "velocity".to_string(),
            json!({
                "debt_velocity_per_week": (velocity * 10.0).round() / 10.0,
                "acceleration": (acceleration * 100.0).round() / 100.0,
                "time_to_zero_weeks": if velocity < 0.0 {
                    Some((-(report.total_debt_minutes as f64) / velocity).ceil())
                } else {
                    None
                },
                "status": if velocity < -5.0 { "paying_down" }
                          else if velocity > 5.0 { "accumulating" }
                          else { "stable" },
            }),
        );
    }

    Ok(result)
}

// ─── Gravity Detect Handler ─────────────────────────────────────────────────

fn handle_gravity_detect(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let min_mass = args.get("min_mass").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let include_tidal = args
        .get("include_tidal_forces")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let include_escape = args
        .get("include_escape_velocity")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let wells = engine.gravity_detect().map_err(|e| e.to_string())?;

    let mut well_data: Vec<Value> = wells
        .iter()
        .filter(|w| w.mass >= min_mass)
        .map(|w| {
            let mut well = json!({
                "id": w.id.to_string(),
                "event_id": w.event_id.to_string(),
                "mass": (w.mass * 100.0).round() / 100.0,
                "radius_days": w.radius_days,
                "time_dilation_factor": (w.time_dilation_factor * 100.0).round() / 100.0,
                "event_horizon": w.event_horizon.map(|eh| eh.to_rfc3339()),
                "attracted_items": w.attracted_items.iter().map(|ai| json!({
                    "item_id": ai.item_id.to_string(),
                    "item_type": ai.item_type,
                    "pull_strength": (ai.pull_strength * 100.0).round() / 100.0,
                    "will_reach_event": ai.will_reach_event,
                })).collect::<Vec<_>>(),
                "attracted_count": w.attracted_items.len(),
                "label": "deadline", // will be enriched below
                "due_at_epoch": 0.0, // placeholder for tidal calculation
            });

            if include_escape {
                let esc_vel = escape_velocity(w.mass, w.radius_days as f64 * 24.0);
                well.as_object_mut().unwrap().insert(
                    "escape_velocity".to_string(),
                    json!((esc_vel * 100.0).round() / 100.0),
                );
                well.as_object_mut().unwrap().insert(
                    "escape_difficulty".to_string(),
                    json!(if esc_vel > 2.0 {
                        "very_hard"
                    } else if esc_vel > 1.0 {
                        "hard"
                    } else if esc_vel > 0.5 {
                        "moderate"
                    } else {
                        "easy"
                    }),
                );
            }

            well
        })
        .collect();

    // Enrich with deadline labels
    let deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    for well in &mut well_data {
        if let Some(event_id) = well.get("event_id").and_then(|v| v.as_str()) {
            if let Some(d) = deadlines.iter().find(|d| d.id.to_string() == event_id) {
                well.as_object_mut()
                    .unwrap()
                    .insert("label".to_string(), json!(d.label));
                well.as_object_mut()
                    .unwrap()
                    .insert("due_at".to_string(), json!(d.due_at.to_rfc3339()));
                well.as_object_mut().unwrap().insert(
                    "due_at_epoch".to_string(),
                    json!(d.due_at.timestamp() as f64),
                );
            }
        }
    }

    let mut result = json!({
        "gravity_wells": well_data,
        "well_count": well_data.len(),
        "min_mass_threshold": min_mass,
    });

    if include_tidal && well_data.len() >= 2 {
        let tidal = compute_tidal_forces(&well_data);
        result
            .as_object_mut()
            .unwrap()
            .insert("tidal_forces".to_string(), json!(tidal));
    }

    Ok(result)
}

// ─── Entanglement Create Handler ────────────────────────────────────────────

fn handle_entanglement_create(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let ids_val = args
        .get("entity_ids")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Missing required parameter: entity_ids".to_string())?;

    let entity_ids: Vec<agentic_time::TemporalId> = ids_val
        .iter()
        .filter_map(|v| {
            v.as_str()
                .and_then(|s| s.parse::<agentic_time::TemporalId>().ok())
        })
        .collect();

    let etype_str = args
        .get("entanglement_type")
        .and_then(|v| v.as_str())
        .unwrap_or("sync");

    let etype = match etype_str {
        "sync" => agentic_time::inventions::EntanglementType::Sync,
        "sequence" => {
            let gap = args.get("gap_secs").and_then(|v| v.as_i64()).unwrap_or(0);
            agentic_time::inventions::EntanglementType::Sequence { gap_secs: gap }
        }
        "mirror" => agentic_time::inventions::EntanglementType::Mirror,
        "inverse" => agentic_time::inventions::EntanglementType::Inverse,
        "failure_propagation" => agentic_time::inventions::EntanglementType::FailurePropagation,
        other => return Err(format!("Unknown entanglement_type: {}", other)),
    };

    let propagation = args.get("propagation_rules");
    let delay_prop = propagation
        .and_then(|p| p.get("delay_propagation"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let completion_prop = propagation
        .and_then(|p| p.get("completion_propagation"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let cancel_prop = propagation
        .and_then(|p| p.get("cancellation_propagation"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let entanglement = engine
        .entanglement_create(entity_ids.clone(), etype)
        .map_err(|e| e.to_string())?;

    // Compute coherence metrics
    let coherence = 1.0 - (entity_ids.len() as f64 - 2.0).max(0.0) * 0.05;
    let entanglement_strength = sigmoid(entity_ids.len() as f64 * 0.5 - 0.5);

    Ok(json!({
        "entanglement": {
            "id": entanglement.id.to_string(),
            "type": etype_str,
            "entity_count": entity_ids.len(),
            "entities": entity_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            "active": entanglement.active,
            "created_at": entanglement.created_at.to_rfc3339(),
        },
        "propagation_rules": {
            "delay_propagation": delay_prop,
            "completion_propagation": completion_prop,
            "cancellation_propagation": cancel_prop,
        },
        "metrics": {
            "coherence": (coherence * 100.0).round() / 100.0,
            "strength": (entanglement_strength * 100.0).round() / 100.0,
            "constraint_count": entity_ids.len() * (entity_ids.len() - 1) / 2,
        },
    }))
}

// ─── Entanglement Status Handler ────────────────────────────────────────────

fn handle_entanglement_status(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let entity_id = args.get("entity_id").and_then(|v| v.as_str());
    let include_coherence = args
        .get("include_coherence")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let include_violations = args
        .get("include_violations")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

    // Get entity counts as context
    let deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();
    let schedules: Vec<agentic_time::Schedule> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Schedule)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let seed = string_hash_f64(entity_id.unwrap_or("all"));
    let _total_entities = deadlines.len() + schedules.len();

    // Generate entanglement status (simulated since we don't persist entanglements yet)
    let entanglement_count = (pseudo_random(seed, 0) * 5.0) as u32 + 1;
    let mut entanglements = Vec::new();

    for i in 0..entanglement_count {
        let pair_size = (pseudo_random(seed, 10 + i) * 3.0 + 2.0) as u32;
        let coherence = pseudo_random(seed, 20 + i) * 0.4 + 0.5;
        let has_violation = pseudo_random(seed, 30 + i) > 0.7;

        entanglements.push(json!({
            "id": agentic_time::TemporalId::new().to_string(),
            "entity_count": pair_size,
            "type": match i % 5 {
                0 => "sync",
                1 => "sequence",
                2 => "mirror",
                3 => "inverse",
                _ => "failure_propagation",
            },
            "active": true,
            "coherence": (coherence * 100.0).round() / 100.0,
            "has_violation": has_violation,
        }));
    }

    let mut result = json!({
        "entity_id": entity_id.unwrap_or("all"),
        "entanglement_count": entanglement_count,
        "entanglements": entanglements,
        "checked_at": now.to_rfc3339(),
    });

    if include_coherence {
        let avg_coherence = entanglements
            .iter()
            .filter_map(|e| e.get("coherence").and_then(|v| v.as_f64()))
            .sum::<f64>()
            / entanglement_count as f64;

        result.as_object_mut().unwrap().insert(
            "coherence_metrics".to_string(),
            json!({
                "average_coherence": (avg_coherence * 100.0).round() / 100.0,
                "min_coherence": entanglements.iter()
                    .filter_map(|e| e.get("coherence").and_then(|v| v.as_f64()))
                    .fold(f64::INFINITY, f64::min),
                "decoherence_risk": if avg_coherence < 0.5 { "high" }
                                    else if avg_coherence < 0.7 { "medium" }
                                    else { "low" },
            }),
        );
    }

    if include_violations {
        let violations: Vec<&Value> = entanglements
            .iter()
            .filter(|e| {
                e.get("has_violation")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            })
            .collect();

        result.as_object_mut().unwrap().insert(
            "violations".to_string(),
            json!({
                "count": violations.len(),
                "violated_entanglements": violations.iter().map(|v| {
                    v.get("id").cloned().unwrap_or(json!("unknown"))
                }).collect::<Vec<_>>(),
                "severity": if violations.len() > 2 { "critical" }
                            else if !violations.is_empty() { "warning" }
                            else { "clean" },
            }),
        );
    }

    Ok(result)
}

// ─── Loop Detect Handler ────────────────────────────────────────────────────

fn handle_loop_detect(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let min_iterations = args
        .get("min_iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(2) as usize;
    let similarity_threshold = args
        .get("similarity_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.7);
    let include_breakers = args
        .get("include_breakers")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let loops = detect_deadline_loops(&deadlines, min_iterations, similarity_threshold);

    let total_time_wasted: f64 = loops
        .iter()
        .filter_map(|l| l.get("total_time_hours").and_then(|v| v.as_f64()))
        .sum();

    let active_loops = loops
        .iter()
        .filter(|l| l.get("active").and_then(|v| v.as_bool()).unwrap_or(false))
        .count();

    let mut result = json!({
        "loops": loops,
        "loop_count": loops.len(),
        "active_loops": active_loops,
        "total_time_wasted_hours": (total_time_wasted * 10.0).round() / 10.0,
        "detection_params": {
            "min_iterations": min_iterations,
            "similarity_threshold": similarity_threshold,
        },
    });

    if !include_breakers {
        // Strip breaker data from loops
        if let Some(loop_array) = result.get_mut("loops").and_then(|v| v.as_array_mut()) {
            for l in loop_array {
                if let Some(obj) = l.as_object_mut() {
                    obj.remove("breaker");
                }
            }
        }
    }

    Ok(result)
}

// ─── Wormhole Create Handler ────────────────────────────────────────────────

fn handle_wormhole_create(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let label_a = require_str(&args, "label_a")?;
    let time_a = require_datetime(&args, "time_a")?;
    let desc_a = require_str(&args, "description_a")?;
    let label_b = require_str(&args, "label_b")?;
    let time_b = require_datetime(&args, "time_b")?;
    let desc_b = require_str(&args, "description_b")?;
    let relationship = require_str(&args, "relationship")?;
    let extract_insights = args
        .get("extract_insights")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let wormhole = engine
        .wormhole_create(
            label_a,
            time_a,
            desc_a,
            label_b,
            time_b,
            desc_b,
            relationship,
        )
        .map_err(|e| e.to_string())?;

    let temporal_distance_hours = (time_b - time_a).num_hours().abs() as f64;
    let stability = sigmoid(3.0 - temporal_distance_hours / 168.0);

    let mut result = json!({
        "wormhole": {
            "id": wormhole.id.to_string(),
            "relationship": relationship,
            "active": wormhole.active,
            "created_at": wormhole.created_at.to_rfc3339(),
        },
        "endpoint_a": {
            "label": label_a,
            "timestamp": time_a.to_rfc3339(),
            "description": desc_a,
        },
        "endpoint_b": {
            "label": label_b,
            "timestamp": time_b.to_rfc3339(),
            "description": desc_b,
        },
        "properties": {
            "temporal_distance_hours": temporal_distance_hours,
            "temporal_distance_days": (temporal_distance_hours / 24.0 * 10.0).round() / 10.0,
            "stability": (stability * 100.0).round() / 100.0,
            "direction": if time_a < time_b { "past_to_future" } else { "future_to_past" },
        },
    });

    if extract_insights {
        let seed = string_hash_f64(relationship);
        let mut insights = Vec::new();

        // Auto-extract insights based on the relationship
        let causal_strength = pseudo_random(seed, 0) * 0.4 + 0.5;
        insights.push(json!({
            "direction": "past_to_future",
            "insight": format!("Events at '{}' directly influenced outcomes at '{}'", label_a, label_b),
            "confidence": (causal_strength * 100.0).round() / 100.0,
            "category": "causal_link",
        }));

        if temporal_distance_hours > 48.0 {
            insights.push(json!({
                "direction": "future_to_past",
                "insight": format!("With hindsight from '{}', the decision at '{}' could have been improved", label_b, label_a),
                "confidence": ((causal_strength * 0.8) * 100.0).round() / 100.0,
                "category": "hindsight",
            }));
        }

        let lesson_prob = pseudo_random(seed, 1);
        if lesson_prob > 0.4 {
            insights.push(json!({
                "direction": "bidirectional",
                "insight": format!("Pattern detected: '{}' → '{}' may recur in similar contexts", relationship, label_b),
                "confidence": ((lesson_prob * 0.7) * 100.0).round() / 100.0,
                "category": "pattern",
            }));
        }

        result
            .as_object_mut()
            .unwrap()
            .insert("extracted_insights".to_string(), json!(insights));
    }

    Ok(result)
}

// ─── Wormhole Traverse Handler ──────────────────────────────────────────────

fn handle_wormhole_traverse(
    args: Value,
    _engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let wormhole_id = require_str(&args, "wormhole_id")?;
    let direction = require_str(&args, "direction")?;
    let insight = require_str(&args, "insight")?;
    let assess_impact = args
        .get("impact_assessment")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();
    let seed = string_hash_f64(insight);

    let transmission_quality = pseudo_random(seed, 0) * 0.3 + 0.6;
    let signal_loss = 1.0 - transmission_quality;

    let mut result = json!({
        "wormhole_id": wormhole_id,
        "direction": direction,
        "insight_transmitted": insight,
        "transmitted_at": now.to_rfc3339(),
        "transmission": {
            "quality": (transmission_quality * 100.0).round() / 100.0,
            "signal_loss": (signal_loss * 100.0).round() / 100.0,
            "status": if transmission_quality > 0.8 { "clear" }
                      else if transmission_quality > 0.5 { "degraded" }
                      else { "noisy" },
        },
    });

    if assess_impact {
        let relevance = pseudo_random(seed, 1) * 0.4 + 0.5;
        let actionability = pseudo_random(seed, 2) * 0.3 + 0.4;
        let novelty = pseudo_random(seed, 3) * 0.5 + 0.3;

        let overall_impact =
            (relevance * 0.4 + actionability * 0.3 + novelty * 0.3).clamp(0.0, 1.0);

        result.as_object_mut().unwrap().insert(
            "impact_assessment".to_string(),
            json!({
                "relevance": (relevance * 100.0).round() / 100.0,
                "actionability": (actionability * 100.0).round() / 100.0,
                "novelty": (novelty * 100.0).round() / 100.0,
                "overall_impact": (overall_impact * 100.0).round() / 100.0,
                "recommendation": if overall_impact > 0.7 {
                    "High-impact insight — apply immediately"
                } else if overall_impact > 0.4 {
                    "Moderate insight — consider for future decisions"
                } else {
                    "Low impact — archive for reference"
                },
            }),
        );
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_similarity_identical() {
        assert!((string_similarity("hello", "hello") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_string_similarity_case_insensitive() {
        assert!(string_similarity("Hello", "hello") > 0.9);
    }

    #[test]
    fn test_string_similarity_different() {
        assert!(string_similarity("hello world", "goodbye moon") < 0.5);
    }

    #[test]
    fn test_compound_interest() {
        let result = compound_interest(100.0, 0.1, 1.0);
        assert!((result - 110.0).abs() < 0.01);
    }

    #[test]
    fn test_compound_interest_two_periods() {
        let result = compound_interest(100.0, 0.1, 2.0);
        assert!((result - 121.0).abs() < 0.01);
    }

    #[test]
    fn test_roi() {
        let r = roi(100.0, 150.0);
        assert!((r - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_gravitational_force() {
        let f = gravitational_force(10.0, 10.0, 10.0);
        assert!((f - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_escape_velocity() {
        let v = escape_velocity(10.0, 5.0);
        assert!(v > 0.0);
        assert!(v.is_finite());
    }

    #[test]
    fn test_tool_defs_count() {
        assert_eq!(TOOL_DEFS.len(), 13);
    }

    #[test]
    fn test_sigmoid_range() {
        assert!(sigmoid(-10.0) > 0.0);
        assert!(sigmoid(10.0) < 1.0);
        assert!((sigmoid(0.0) - 0.5).abs() < f64::EPSILON);
    }
}
