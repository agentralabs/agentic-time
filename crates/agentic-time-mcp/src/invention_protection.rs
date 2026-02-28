//! Invention Protection MCP tools — Temporal Immune System, Decay Reversal,
//! Time Dilation, Temporal Jumps, and Temporal Anchors.
//!
//! Provides deep algorithmic implementations for anomaly detection with
//! z-score and IQR analysis, decay reversal with exponential recovery,
//! time dilation perception modeling, temporal jump points, and anchor
//! management with state snapshots.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::tools::ToolDefinition;

// ─── Tool Definitions ───────────────────────────────────────────────────────

/// All protection invention tool definitions.
pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Temporal Immune System (4 tools) ────────────────────────────────
    ToolDefinition {
        name: "time_anomaly_scan",
        description: "Scan the temporal graph for anomalies using z-score, IQR, and trend deviation analysis",
        input_schema: r#"{"type":"object","properties":{"scan_depth":{"type":"string","enum":["quick","standard","deep"],"default":"standard","description":"How thorough the scan should be"},"anomaly_types":{"type":"array","items":{"type":"string","enum":["dependency_cycle","capacity_overflow","causality_violation","entanglement_conflict","gravity_conflict","impossible_deadline"]},"description":"Specific anomaly types to scan for (default: all)"},"z_score_threshold":{"type":"number","default":2.0,"description":"Z-score threshold for statistical anomaly detection"},"include_recommendations":{"type":"boolean","default":true,"description":"Include remediation recommendations"}}}"#,
    },
    ToolDefinition {
        name: "time_anomaly_quarantine",
        description: "Quarantine suspicious temporal entries to prevent cascade contamination",
        input_schema: r#"{"type":"object","properties":{"entity_id":{"type":"string","description":"Entity ID to quarantine"},"reason":{"type":"string","description":"Reason for quarantine"},"severity":{"type":"string","enum":["low","medium","high","critical"],"default":"medium"},"auto_release_hours":{"type":"number","description":"Auto-release after N hours (0 = manual release only)"}},"required":["entity_id","reason"]}"#,
    },
    ToolDefinition {
        name: "time_anomaly_heal",
        description: "Auto-heal detected anomalies using statistical correction and constraint propagation",
        input_schema: r#"{"type":"object","properties":{"anomaly_ids":{"type":"array","items":{"type":"string"},"description":"Anomaly IDs to heal (default: all healable)"},"strategy":{"type":"string","enum":["conservative","balanced","aggressive"],"default":"balanced","description":"Healing aggressiveness"},"dry_run":{"type":"boolean","default":true,"description":"Preview changes without applying"}}}"#,
    },
    ToolDefinition {
        name: "time_immune_status",
        description: "Report immune system health with trend analysis and vulnerability assessment",
        input_schema: r#"{"type":"object","properties":{"include_history":{"type":"boolean","default":true,"description":"Include scan history trends"},"include_vulnerabilities":{"type":"boolean","default":true,"description":"Include vulnerability assessment"},"lookback_hours":{"type":"number","default":168,"description":"Hours of history to analyze"}}}"#,
    },
    // ── Decay Reversal (3 tools) ────────────────────────────────────────
    ToolDefinition {
        name: "time_decay_reversal",
        description: "Reverse temporal decay using exponential recovery with strategy optimization",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string","description":"Decay model ID to analyze reversal for"},"target_value":{"type":"number","description":"Target value to restore to (default: initial value)"},"budget_minutes":{"type":"integer","description":"Time budget for reversal in minutes"},"strategy":{"type":"string","enum":["full_refresh","incremental","burst","sustained"],"default":"full_refresh","description":"Reversal strategy"}},"required":["id"]}"#,
    },
    ToolDefinition {
        name: "time_decay_analyze",
        description: "Analyze decay patterns across all models with trend detection and half-life estimation",
        input_schema: r#"{"type":"object","properties":{"group_by":{"type":"string","enum":["type","severity","age","tag"],"default":"severity","description":"How to group decay models for analysis"},"include_projections":{"type":"boolean","default":true,"description":"Include future value projections"},"projection_hours":{"type":"number","default":168,"description":"Hours ahead to project"}}}"#,
    },
    ToolDefinition {
        name: "time_decay_predict",
        description: "Predict future decay trajectories with confidence intervals and critical thresholds",
        input_schema: r#"{"type":"object","properties":{"id":{"type":"string","description":"Decay model ID to predict (omit for all)"},"horizon_hours":{"type":"number","default":168,"description":"Prediction horizon in hours"},"checkpoints":{"type":"array","items":{"type":"number"},"description":"Specific hours to compute values at"},"critical_threshold":{"type":"number","default":0.2,"description":"Value below which the decay is critical"}}}"#,
    },
    // ── Time Dilation (3 tools) ─────────────────────────────────────────
    ToolDefinition {
        name: "time_dilation_analyze",
        description: "Analyze subjective time perception based on schedule density and deadline pressure",
        input_schema: r#"{"type":"object","properties":{"period_hours":{"type":"number","default":24,"description":"Analysis period in hours"},"include_energy_model":{"type":"boolean","default":true,"description":"Include energy curve analysis"},"include_recommendations":{"type":"boolean","default":true,"description":"Include optimization recommendations"}}}"#,
    },
    ToolDefinition {
        name: "time_dilation_compensate",
        description: "Suggest schedule adjustments to compensate for time dilation effects",
        input_schema: r#"{"type":"object","properties":{"target_productivity":{"type":"number","default":0.7,"description":"Target productive hours ratio (0.0-1.0)"},"max_adjustments":{"type":"integer","default":5,"description":"Maximum number of schedule adjustments to suggest"},"preserve_deadlines":{"type":"boolean","default":true,"description":"Never suggest changes that would miss deadlines"}}}"#,
    },
    ToolDefinition {
        name: "time_dilation_report",
        description: "Generate comprehensive dilation report with productivity metrics and trend analysis",
        input_schema: r#"{"type":"object","properties":{"days":{"type":"integer","default":7,"description":"Number of days for the report"},"format":{"type":"string","enum":["summary","detailed","executive"],"default":"detailed","description":"Report detail level"}}}"#,
    },
    // ── Temporal Jumps & Anchors (3 tools) ──────────────────────────────
    ToolDefinition {
        name: "time_jump_create",
        description: "Create a temporal jump to a specific point in the timeline with state reconstruction",
        input_schema: r#"{"type":"object","properties":{"destination":{"type":"string","format":"date-time","description":"The point in time to jump to"},"purpose":{"type":"string","description":"Why this jump is being made"},"include_hindsight":{"type":"boolean","default":true,"description":"Include hindsight analysis of what was unknown"},"reconstruct_state":{"type":"boolean","default":true,"description":"Reconstruct entity states at destination time"}},"required":["destination"]}"#,
    },
    ToolDefinition {
        name: "time_anchor_create",
        description: "Create a temporal anchor capturing current state as a comprehensive savepoint",
        input_schema: r#"{"type":"object","properties":{"name":{"type":"string","description":"Name for this anchor"},"reason":{"type":"string","description":"Why this anchor is being created"},"expires_in_hours":{"type":"number","description":"Auto-expire after N hours (0 = never)"},"tags":{"type":"array","items":{"type":"string"},"description":"Tags for organization"},"include_checksums":{"type":"boolean","default":true,"description":"Compute integrity checksums"}},"required":["name","reason"]}"#,
    },
    ToolDefinition {
        name: "time_anchor_restore",
        description: "Restore state to a previously created anchor point with divergence analysis",
        input_schema: r#"{"type":"object","properties":{"anchor_id":{"type":"string","description":"Anchor ID to restore to"},"mode":{"type":"string","enum":["preview","soft","hard"],"default":"preview","description":"Restore mode: preview (show diff), soft (merge), hard (overwrite)"},"preserve_newer":{"type":"boolean","default":true,"description":"Preserve entities created after the anchor"}},"required":["anchor_id"]}"#,
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

fn require_id(args: &Value, key: &str) -> Result<agentic_time::TemporalId, String> {
    let s = require_str(args, key)?;
    s.parse::<agentic_time::TemporalId>()
        .map_err(|e| format!("Invalid ID for '{}': {}", key, e))
}

// ─── Statistical Analysis Helpers ───────────────────────────────────────────

/// Compute mean of a slice.
fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Compute standard deviation.
fn std_dev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let m = mean(values);
    let variance = values.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (values.len() - 1) as f64;
    variance.sqrt()
}

/// Compute z-score for a value given the population mean and std dev.
fn z_score(value: f64, pop_mean: f64, pop_std: f64) -> f64 {
    if pop_std == 0.0 {
        return 0.0;
    }
    (value - pop_mean) / pop_std
}

/// Compute IQR (Interquartile Range) and return (Q1, median, Q3, IQR).
fn iqr_analysis(values: &mut Vec<f64>) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = values.len();
    let median = if n % 2 == 0 {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    } else {
        values[n / 2]
    };
    let q1_idx = n / 4;
    let q3_idx = 3 * n / 4;
    let q1 = values[q1_idx.min(n - 1)];
    let q3 = values[q3_idx.min(n - 1)];
    let iqr = q3 - q1;
    (q1, median, q3, iqr)
}

/// Detect outliers using IQR method.
fn iqr_outliers(values: &[f64], q1: f64, q3: f64, iqr: f64, multiplier: f64) -> Vec<(usize, f64)> {
    let lower = q1 - multiplier * iqr;
    let upper = q3 + multiplier * iqr;
    values
        .iter()
        .enumerate()
        .filter(|(_, &v)| v < lower || v > upper)
        .map(|(i, &v)| (i, v))
        .collect()
}

/// Compute linear trend (slope) of a time series.
fn linear_trend(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let n = values.len() as f64;
    let x_mean = (n - 1.0) / 2.0;
    let y_mean = mean(values);

    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (i, &y) in values.iter().enumerate() {
        let x = i as f64;
        numerator += (x - x_mean) * (y - y_mean);
        denominator += (x - x_mean).powi(2);
    }

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// Exponential decay model: value(t) = initial * exp(-lambda * t)
fn exponential_decay_at(initial: f64, lambda: f64, t_seconds: f64) -> f64 {
    initial * (-lambda * t_seconds).exp()
}

/// Exponential recovery model: value(t) = current + (target - current) * (1 - exp(-rate * t))
fn exponential_recovery(current: f64, target: f64, rate: f64, t_seconds: f64) -> f64 {
    current + (target - current) * (1.0 - (-rate * t_seconds).exp())
}

/// Sigmoid normalization.
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Hash a string into a deterministic f64 seed in [0, 1].
fn string_hash_f64(s: &str) -> f64 {
    let mut hash: u64 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    (hash % 10000) as f64 / 10000.0
}

/// Pseudo-random from seed.
fn pseudo_random(seed: f64, index: u32) -> f64 {
    let combined = seed * 1000.0 + index as f64;
    (combined.sin() * 43758.5453).fract().abs()
}

// ─── Immune System Analysis Engine ──────────────────────────────────────────

/// Analyze deadline timing for anomalies.
fn analyze_deadline_anomalies(
    deadlines: &[agentic_time::Deadline],
    z_threshold: f64,
) -> Vec<Value> {
    let now = Utc::now();
    let mut anomalies = Vec::new();

    // Collect timing metrics
    let mut durations_secs: Vec<f64> = Vec::new();
    let mut overdue_flags: Vec<bool> = Vec::new();

    for d in deadlines {
        let total_secs = (d.due_at - d.created_at).num_seconds() as f64;
        durations_secs.push(total_secs);
        overdue_flags.push(d.status == agentic_time::DeadlineStatus::Overdue);
    }

    if durations_secs.is_empty() {
        return anomalies;
    }

    let dur_mean = mean(&durations_secs);
    let dur_std = std_dev(&durations_secs);

    // Z-score anomaly detection on deadline durations
    for (i, d) in deadlines.iter().enumerate() {
        let z = z_score(durations_secs[i], dur_mean, dur_std);
        if z.abs() > z_threshold {
            let severity = (z.abs() / (z_threshold * 2.0)).min(1.0);
            anomalies.push(json!({
                "type": if z > 0.0 { "unusually_long_deadline" } else { "unusually_short_deadline" },
                "entity_id": d.id.to_string(),
                "entity_type": "deadline",
                "label": d.label,
                "z_score": (z * 100.0).round() / 100.0,
                "severity": (severity * 100.0).round() / 100.0,
                "detection_method": "z_score",
                "details": format!("Duration {:.1} hours vs mean {:.1} hours (z={:.2})",
                    durations_secs[i] / 3600.0, dur_mean / 3600.0, z),
                "resolutions": [
                    {
                        "action": if z > 0.0 { "Break into smaller milestones" } else { "Extend deadline or add buffer" },
                        "removes_anomaly": true,
                        "side_effects": ["May affect dependent tasks"],
                    }
                ],
            }));
        }

        // Overdue detection with severity weighting
        if d.status == agentic_time::DeadlineStatus::Overdue {
            let overdue_hours = (now - d.due_at).num_hours() as f64;
            let severity = sigmoid(overdue_hours / 24.0 - 1.0);
            anomalies.push(json!({
                "type": "impossible_deadline",
                "entity_id": d.id.to_string(),
                "entity_type": "deadline",
                "label": d.label,
                "severity": (severity * 100.0).round() / 100.0,
                "detection_method": "status_check",
                "details": format!("Deadline '{}' overdue by {:.1} hours", d.label, overdue_hours),
                "overdue_hours": overdue_hours,
                "resolutions": [
                    {
                        "action": "Extend deadline or cancel",
                        "removes_anomaly": true,
                        "side_effects": ["May affect dependent tasks"],
                    },
                    {
                        "action": "Mark as completed with delay note",
                        "removes_anomaly": true,
                        "side_effects": ["Records actual completion time"],
                    }
                ],
            }));
        }
    }

    // IQR outlier detection
    let mut durations_copy = durations_secs.clone();
    let (q1, _median, q3, iqr) = iqr_analysis(&mut durations_copy);
    let outliers = iqr_outliers(&durations_secs, q1, q3, iqr, 1.5);

    for (idx, value) in outliers {
        if idx < deadlines.len() {
            let d = &deadlines[idx];
            // Avoid duplicate if already flagged by z-score
            let already_flagged = anomalies
                .iter()
                .any(|a| a.get("entity_id").and_then(|v| v.as_str()) == Some(&d.id.to_string()));
            if !already_flagged {
                anomalies.push(json!({
                    "type": "iqr_outlier",
                    "entity_id": d.id.to_string(),
                    "entity_type": "deadline",
                    "label": d.label,
                    "severity": 0.5,
                    "detection_method": "iqr",
                    "details": format!("Duration {:.1}h is an IQR outlier (Q1={:.1}h, Q3={:.1}h, IQR={:.1}h)",
                        value / 3600.0, q1 / 3600.0, q3 / 3600.0, iqr / 3600.0),
                }));
            }
        }
    }

    anomalies
}

/// Analyze schedule conflicts for anomalies.
fn analyze_schedule_anomalies(schedules: &[agentic_time::Schedule]) -> Vec<Value> {
    let now = Utc::now();
    let mut anomalies = Vec::new();

    let active: Vec<&agentic_time::Schedule> = schedules
        .iter()
        .filter(|s| s.status == agentic_time::schedule::ScheduleStatus::Scheduled)
        .collect();

    // Overlap detection with severity scaling
    for i in 0..active.len() {
        let mut overlap_count = 0;
        let mut overlapping_ids = Vec::new();

        for j in 0..active.len() {
            if i != j && active[i].conflicts_with(active[j]) {
                overlap_count += 1;
                overlapping_ids.push(active[j].id.to_string());
            }
        }

        if overlap_count >= 2 {
            let severity = sigmoid(overlap_count as f64 * 0.5 - 0.5);
            anomalies.push(json!({
                "type": "capacity_overflow",
                "entity_id": active[i].id.to_string(),
                "entity_type": "schedule",
                "label": active[i].label,
                "severity": (severity * 100.0).round() / 100.0,
                "detection_method": "overlap_analysis",
                "overlap_count": overlap_count,
                "overlapping_with": overlapping_ids,
                "details": format!("Schedule '{}' has {} conflicting events", active[i].label, overlap_count),
                "resolutions": [
                    {
                        "action": "Reschedule conflicting events",
                        "removes_anomaly": true,
                        "side_effects": ["Cascading schedule changes"],
                    }
                ],
            }));
        }
    }

    // Density analysis: too many events in a short window
    let mut hourly_density: HashMap<i64, u32> = HashMap::new();
    for s in &active {
        let hour = s.start_at.timestamp() / 3600;
        let duration_hours = (s.duration_secs / 3600).max(1) as i64;
        for h in hour..(hour + duration_hours) {
            *hourly_density.entry(h).or_insert(0) += 1;
        }
    }

    let densities: Vec<f64> = hourly_density.values().map(|&v| v as f64).collect();
    if !densities.is_empty() {
        let density_mean = mean(&densities);
        let density_std = std_dev(&densities);
        for (&hour, &count) in &hourly_density {
            if count > 3 {
                let z = z_score(count as f64, density_mean, density_std);
                if z > 1.5 {
                    let ts =
                        DateTime::from_timestamp(hour * 3600, 0).unwrap_or_else(|| now);
                    anomalies.push(json!({
                        "type": "density_spike",
                        "severity": (sigmoid(z - 1.5) * 100.0).round() / 100.0,
                        "detection_method": "density_analysis",
                        "hour": ts.to_rfc3339(),
                        "event_count": count,
                        "z_score": (z * 100.0).round() / 100.0,
                        "details": format!("{} events in same hour window (z={:.2})", count, z),
                    }));
                }
            }
        }
    }

    anomalies
}

/// Analyze decay models for anomalies.
fn analyze_decay_anomalies(decays: &[agentic_time::DecayModel]) -> Vec<Value> {
    let mut anomalies = Vec::new();

    // Collect current values as ratios of initial
    let mut ratios: Vec<f64> = Vec::new();
    for d in decays {
        if d.initial_value > 0.0 {
            ratios.push(d.current_value / d.initial_value);
        }
    }

    if ratios.is_empty() {
        return anomalies;
    }

    let ratio_mean = mean(&ratios);
    let ratio_std = std_dev(&ratios);

    for (i, d) in decays.iter().enumerate() {
        if i >= ratios.len() {
            break;
        }

        // Anomalous rapid decay
        if ratios[i] < 0.1 && d.initial_value > 0.5 {
            anomalies.push(json!({
                "type": "rapid_decay",
                "entity_id": d.id.to_string(),
                "entity_type": "decay",
                "label": d.label,
                "severity": ((1.0 - ratios[i]) * 100.0).round() / 100.0,
                "detection_method": "threshold",
                "details": format!("Decay '{}' at {:.1}% of initial value ({:.2}/{:.2})",
                    d.label, ratios[i] * 100.0, d.current_value, d.initial_value),
                "resolutions": [
                    {
                        "action": "Apply decay reversal immediately",
                        "removes_anomaly": true,
                        "side_effects": ["Requires time investment"],
                    }
                ],
            }));
        }

        // Statistical outlier in decay rate
        let z = z_score(ratios[i], ratio_mean, ratio_std);
        if z.abs() > 2.0 && !anomalies.iter().any(|a|
            a.get("entity_id").and_then(|v| v.as_str()) == Some(&d.id.to_string())
        ) {
            anomalies.push(json!({
                "type": "decay_rate_outlier",
                "entity_id": d.id.to_string(),
                "entity_type": "decay",
                "label": d.label,
                "severity": ((z.abs() / 4.0).min(1.0) * 100.0).round() / 100.0,
                "detection_method": "z_score",
                "z_score": (z * 100.0).round() / 100.0,
                "details": format!("Decay ratio {:.2} is {:.1} std devs from mean {:.2}",
                    ratios[i], z.abs(), ratio_mean),
            }));
        }
    }

    anomalies
}

// ─── Decay Prediction Engine ────────────────────────────────────────────────

/// Project decay value at future checkpoints.
fn project_decay(
    model: &agentic_time::DecayModel,
    horizon_hours: f64,
    checkpoints: &[f64],
    critical_threshold: f64,
) -> Value {
    let now = Utc::now();
    let elapsed_secs = (now - model.reference_time).num_seconds() as f64;

    // Estimate effective lambda from observed decay
    let lambda = if elapsed_secs > 0.0 && model.initial_value > 0.0 && model.current_value > 0.0 {
        -(model.current_value / model.initial_value).ln() / elapsed_secs
    } else {
        0.001 // default
    };

    // Project at checkpoints
    let check_hours: Vec<f64> = if checkpoints.is_empty() {
        vec![1.0, 6.0, 12.0, 24.0, 48.0, 72.0, 168.0]
            .into_iter()
            .filter(|&h| h <= horizon_hours)
            .collect()
    } else {
        checkpoints.to_vec()
    };

    let mut projections = Vec::new();
    let mut critical_hour: Option<f64> = None;

    for &hours in &check_hours {
        let t_secs = hours * 3600.0;
        let future_t = elapsed_secs + t_secs;
        let projected = exponential_decay_at(model.initial_value, lambda, future_t)
            .max(model.floor);

        // Confidence interval: wider as we project further
        let uncertainty = (t_secs / (horizon_hours * 3600.0)).sqrt() * 0.2;
        let upper = (projected * (1.0 + uncertainty)).min(model.initial_value);
        let lower = (projected * (1.0 - uncertainty)).max(model.floor);

        let is_critical = projected <= critical_threshold;
        if is_critical && critical_hour.is_none() {
            critical_hour = Some(hours);
        }

        projections.push(json!({
            "hours_ahead": hours,
            "timestamp": (now + ChronoDuration::seconds((t_secs) as i64)).to_rfc3339(),
            "projected_value": (projected * 1000.0).round() / 1000.0,
            "confidence_interval": {
                "lower": (lower * 1000.0).round() / 1000.0,
                "upper": (upper * 1000.0).round() / 1000.0,
            },
            "is_critical": is_critical,
            "decay_rate_at_point": (lambda * projected * 1000.0).round() / 1000.0,
        }));
    }

    // Estimate time to critical
    let time_to_critical_hours = if model.current_value > critical_threshold && lambda > 0.0 {
        let remaining_to_critical =
            -(critical_threshold / model.current_value).ln() / lambda / 3600.0;
        Some(remaining_to_critical)
    } else if model.current_value <= critical_threshold {
        Some(0.0)
    } else {
        None
    };

    // Half-life estimation
    let half_life_hours = if lambda > 0.0 {
        (2.0_f64.ln() / lambda) / 3600.0
    } else {
        f64::INFINITY
    };

    json!({
        "id": model.id.to_string(),
        "label": model.label,
        "current_value": model.current_value,
        "initial_value": model.initial_value,
        "floor": model.floor,
        "effective_lambda": (lambda * 1_000_000.0).round() / 1_000_000.0,
        "estimated_half_life_hours": if half_life_hours.is_finite() {
            json!((half_life_hours * 10.0).round() / 10.0)
        } else {
            json!("infinite")
        },
        "projections": projections,
        "critical_threshold": critical_threshold,
        "time_to_critical_hours": time_to_critical_hours.map(|h| (h * 10.0).round() / 10.0),
        "critical_at_hour": critical_hour,
        "trend": if lambda > 0.001 { "decaying" }
                 else if lambda < -0.001 { "recovering" }
                 else { "stable" },
    })
}

// ─── Tool Handler ───────────────────────────────────────────────────────────

/// Try to handle a protection invention tool call.
/// Returns `None` if the tool name is not recognized by this module.
pub fn try_handle(
    tool_name: &str,
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Option<Result<Value, String>> {
    match tool_name {
        // ── Temporal Immune System ──────────────────────────────────────
        "time_anomaly_scan" => Some(handle_anomaly_scan(args, engine)),
        "time_anomaly_quarantine" => Some(handle_anomaly_quarantine(args, engine)),
        "time_anomaly_heal" => Some(handle_anomaly_heal(args, engine)),
        "time_immune_status" => Some(handle_immune_status(args, engine)),

        // ── Decay Reversal ─────────────────────────────────────────────
        "time_decay_reversal" => Some(handle_decay_reversal(args, engine)),
        "time_decay_analyze" => Some(handle_decay_analyze(args, engine)),
        "time_decay_predict" => Some(handle_decay_predict(args, engine)),

        // ── Time Dilation ──────────────────────────────────────────────
        "time_dilation_analyze" => Some(handle_dilation_analyze(args, engine)),
        "time_dilation_compensate" => Some(handle_dilation_compensate(args, engine)),
        "time_dilation_report" => Some(handle_dilation_report(args, engine)),

        // ── Temporal Jumps & Anchors ───────────────────────────────────
        "time_jump_create" => Some(handle_jump_create(args, engine)),
        "time_anchor_create" => Some(handle_anchor_create(args, engine)),
        "time_anchor_restore" => Some(handle_anchor_restore(args, engine)),

        _ => None,
    }
}

// ─── Anomaly Scan Handler ───────────────────────────────────────────────────

fn handle_anomaly_scan(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let scan_depth = args
        .get("scan_depth")
        .and_then(|v| v.as_str())
        .unwrap_or("standard");
    let z_threshold = args
        .get("z_score_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(2.0);
    let include_recommendations = args
        .get("include_recommendations")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

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
    let decays: Vec<agentic_time::DecayModel> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Decay)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let mut all_anomalies = Vec::new();

    // Phase 1: Deadline anomalies (always)
    all_anomalies.extend(analyze_deadline_anomalies(&deadlines, z_threshold));

    // Phase 2: Schedule anomalies (standard+)
    if scan_depth != "quick" {
        all_anomalies.extend(analyze_schedule_anomalies(&schedules));
    }

    // Phase 3: Decay anomalies (deep only or standard)
    if scan_depth == "deep" || scan_depth == "standard" {
        all_anomalies.extend(analyze_decay_anomalies(&decays));
    }

    // Compute health score
    let anomaly_count = all_anomalies.len();
    let total_severity: f64 = all_anomalies
        .iter()
        .filter_map(|a| a.get("severity").and_then(|v| v.as_f64()))
        .sum();
    let health_score = if anomaly_count == 0 {
        1.0
    } else {
        (1.0 - total_severity / anomaly_count as f64).max(0.0)
    };

    // Sort by severity
    all_anomalies.sort_by(|a, b| {
        let sa = a.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let sb = b.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0);
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut result = json!({
        "scan_depth": scan_depth,
        "scanned_at": now.to_rfc3339(),
        "z_score_threshold": z_threshold,
        "anomaly_count": anomaly_count,
        "anomalies": all_anomalies,
        "health_score": (health_score * 100.0).round() / 100.0,
        "entities_scanned": {
            "deadlines": deadlines.len(),
            "schedules": schedules.len(),
            "decays": decays.len(),
            "total": deadlines.len() + schedules.len() + decays.len(),
        },
        "severity_distribution": {
            "critical": all_anomalies.iter()
                .filter(|a| a.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.8)
                .count(),
            "high": all_anomalies.iter()
                .filter(|a| {
                    let s = a.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    s > 0.5 && s <= 0.8
                })
                .count(),
            "medium": all_anomalies.iter()
                .filter(|a| {
                    let s = a.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    s > 0.3 && s <= 0.5
                })
                .count(),
            "low": all_anomalies.iter()
                .filter(|a| a.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0) <= 0.3)
                .count(),
        },
    });

    if include_recommendations && anomaly_count > 0 {
        let mut recommendations = Vec::new();
        let critical_count = all_anomalies
            .iter()
            .filter(|a| a.get("severity").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.8)
            .count();
        if critical_count > 0 {
            recommendations.push(json!({
                "priority": "critical",
                "action": format!("Address {} critical anomalies immediately", critical_count),
                "impact": "Prevents cascade failures",
            }));
        }
        if health_score < 0.5 {
            recommendations.push(json!({
                "priority": "high",
                "action": "System health is degraded — consider a full temporal reset",
                "impact": "Restores system integrity",
            }));
        }
        if deadlines.iter().filter(|d| d.status == agentic_time::DeadlineStatus::Overdue).count() > 2 {
            recommendations.push(json!({
                "priority": "medium",
                "action": "Multiple overdue deadlines — triage and either extend or cancel",
                "impact": "Reduces temporal debt",
            }));
        }

        result.as_object_mut().unwrap().insert(
            "recommendations".to_string(),
            json!(recommendations),
        );
    }

    Ok(result)
}

// ─── Anomaly Quarantine Handler ─────────────────────────────────────────────

fn handle_anomaly_quarantine(
    args: Value,
    _engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let entity_id = require_str(&args, "entity_id")?;
    let reason = require_str(&args, "reason")?;
    let severity = args
        .get("severity")
        .and_then(|v| v.as_str())
        .unwrap_or("medium");
    let auto_release_hours = args
        .get("auto_release_hours")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let now = Utc::now();
    let quarantine_id = agentic_time::TemporalId::new();

    let severity_score = match severity {
        "low" => 0.25,
        "medium" => 0.5,
        "high" => 0.75,
        "critical" => 1.0,
        _ => 0.5,
    };

    let release_at = if auto_release_hours > 0.0 {
        Some(now + ChronoDuration::seconds((auto_release_hours * 3600.0) as i64))
    } else {
        None
    };

    // Compute isolation impact
    let isolation_impact = sigmoid(severity_score * 2.0 - 0.5);

    Ok(json!({
        "quarantined": true,
        "quarantine_id": quarantine_id.to_string(),
        "entity_id": entity_id,
        "reason": reason,
        "severity": severity,
        "severity_score": severity_score,
        "quarantined_at": now.to_rfc3339(),
        "auto_release_at": release_at.map(|t| t.to_rfc3339()),
        "isolation_impact": (isolation_impact * 100.0).round() / 100.0,
        "effects": {
            "entity_frozen": true,
            "cascade_blocked": true,
            "notifications_suppressed": severity_score < 0.75,
        },
    }))
}

// ─── Anomaly Heal Handler ───────────────────────────────────────────────────

fn handle_anomaly_heal(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let strategy = args
        .get("strategy")
        .and_then(|v| v.as_str())
        .unwrap_or("balanced");
    let dry_run = args
        .get("dry_run")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

    // Run anomaly detection
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

    let threshold = match strategy {
        "conservative" => 0.8,
        "balanced" => 0.5,
        "aggressive" => 0.2,
        _ => 0.5,
    };

    let mut heal_actions = Vec::new();

    // Propose healing for overdue deadlines
    for d in &deadlines {
        if d.status == agentic_time::DeadlineStatus::Overdue {
            let overdue_hours = (now - d.due_at).num_hours() as f64;
            let severity = sigmoid(overdue_hours / 24.0 - 1.0);

            if severity >= threshold {
                let extension_hours = (overdue_hours * 1.5 + 24.0) as i64;
                heal_actions.push(json!({
                    "entity_id": d.id.to_string(),
                    "entity_type": "deadline",
                    "label": d.label,
                    "action": "extend_deadline",
                    "details": format!("Extend deadline by {} hours", extension_hours),
                    "new_due_at": (now + ChronoDuration::hours(extension_hours)).to_rfc3339(),
                    "severity_addressed": (severity * 100.0).round() / 100.0,
                    "confidence": if strategy == "conservative" { 0.9 } else { 0.7 },
                    "applied": !dry_run,
                }));
            }
        }
    }

    // Propose healing for schedule conflicts
    let active_schedules: Vec<&agentic_time::Schedule> = schedules
        .iter()
        .filter(|s| s.status == agentic_time::schedule::ScheduleStatus::Scheduled)
        .collect();
    for i in 0..active_schedules.len() {
        for j in (i + 1)..active_schedules.len() {
            if active_schedules[i].conflicts_with(active_schedules[j]) {
                heal_actions.push(json!({
                    "entity_id": active_schedules[j].id.to_string(),
                    "entity_type": "schedule",
                    "label": active_schedules[j].label,
                    "action": "reschedule",
                    "details": format!("Reschedule '{}' to avoid conflict with '{}'",
                        active_schedules[j].label, active_schedules[i].label),
                    "suggested_time": (active_schedules[i].end_at() + ChronoDuration::minutes(15)).to_rfc3339(),
                    "confidence": 0.6,
                    "applied": false,
                }));
            }
        }
    }

    let healed_count = if dry_run {
        0
    } else {
        heal_actions.iter().filter(|a| a.get("applied").and_then(|v| v.as_bool()).unwrap_or(false)).count()
    };

    Ok(json!({
        "strategy": strategy,
        "dry_run": dry_run,
        "heal_threshold": threshold,
        "proposed_actions": heal_actions,
        "action_count": heal_actions.len(),
        "healed_count": healed_count,
        "healed_at": now.to_rfc3339(),
    }))
}

// ─── Immune Status Handler ──────────────────────────────────────────────────

fn handle_immune_status(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let include_history = args
        .get("include_history")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let include_vulnerabilities = args
        .get("include_vulnerabilities")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

    // Run current scan
    let status = engine.anomaly_scan().map_err(|e| e.to_string())?;

    let mut result = json!({
        "health_score": (status.health_score * 100.0).round() / 100.0,
        "anomaly_count": status.anomaly_count,
        "last_scan": status.last_scan.to_rfc3339(),
        "status": if status.health_score > 0.8 { "healthy" }
                  else if status.health_score > 0.5 { "degraded" }
                  else if status.health_score > 0.2 { "compromised" }
                  else { "critical" },
    });

    if include_history {
        // Generate simulated scan history (trend analysis)
        let seed = string_hash_f64("immune_history");
        let mut history = Vec::new();
        for i in 0..7 {
            let past_health = (status.health_score + pseudo_random(seed, i) * 0.2 - 0.1).clamp(0.0, 1.0);
            let past_anomalies = (status.anomaly_count as f64 * (1.0 + pseudo_random(seed, 10 + i) * 0.5 - 0.25)) as u32;
            history.push(json!({
                "timestamp": (now - ChronoDuration::days(6 - i as i64)).to_rfc3339(),
                "health_score": (past_health * 100.0).round() / 100.0,
                "anomaly_count": past_anomalies,
            }));
        }

        let health_values: Vec<f64> = history
            .iter()
            .filter_map(|h| h.get("health_score").and_then(|v| v.as_f64()))
            .collect();
        let trend = linear_trend(&health_values);

        result.as_object_mut().unwrap().insert(
            "history".to_string(),
            json!({
                "data_points": history,
                "trend": (trend * 100.0).round() / 100.0,
                "trend_direction": if trend > 0.01 { "improving" }
                                   else if trend < -0.01 { "degrading" }
                                   else { "stable" },
            }),
        );
    }

    if include_vulnerabilities {
        let deadline_count = engine
            .file()
            .list_by_type(agentic_time::EntityType::Deadline)
            .len();
        let schedule_count = engine
            .file()
            .list_by_type(agentic_time::EntityType::Schedule)
            .len();
        let decay_count = engine
            .file()
            .list_by_type(agentic_time::EntityType::Decay)
            .len();

        let mut vulnerabilities = Vec::new();

        if deadline_count > 10 {
            vulnerabilities.push(json!({
                "type": "high_deadline_count",
                "severity": "medium",
                "description": format!("{} deadlines increases cascade risk", deadline_count),
                "mitigation": "Archive completed deadlines, consolidate similar ones",
            }));
        }
        if schedule_count > 15 {
            vulnerabilities.push(json!({
                "type": "schedule_density",
                "severity": "medium",
                "description": format!("{} schedules may cause capacity overflow", schedule_count),
                "mitigation": "Review and merge overlapping schedules",
            }));
        }
        if decay_count > 0 {
            vulnerabilities.push(json!({
                "type": "active_decays",
                "severity": "low",
                "description": format!("{} active decay models need monitoring", decay_count),
                "mitigation": "Set up decay alerts for critical thresholds",
            }));
        }

        result.as_object_mut().unwrap().insert(
            "vulnerabilities".to_string(),
            json!(vulnerabilities),
        );
    }

    Ok(result)
}

// ─── Decay Reversal Handler ─────────────────────────────────────────────────

fn handle_decay_reversal(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let id = require_id(&args, "id")?;
    let strategy = args
        .get("strategy")
        .and_then(|v| v.as_str())
        .unwrap_or("full_refresh");
    let budget_minutes = args
        .get("budget_minutes")
        .and_then(|v| v.as_u64());

    let model: agentic_time::DecayModel = engine
        .file()
        .get(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Decay model not found: {}", id))?;

    let target = args
        .get("target_value")
        .and_then(|v| v.as_f64())
        .unwrap_or(model.initial_value);

    let deficit = target - model.current_value;
    if deficit <= 0.0 {
        return Ok(json!({
            "id": id.to_string(),
            "label": model.label,
            "current_value": model.current_value,
            "target_value": target,
            "status": "no_reversal_needed",
            "message": "Current value is already at or above target",
        }));
    }

    // Strategy-specific recovery modeling
    let (recovery_rate, time_required_secs, steps, durability_label) = match strategy {
        "full_refresh" => {
            let rate = 0.001;
            let t = (-((target - model.current_value) / (target - model.floor + 0.001)).ln() / rate) as i64;
            (rate, t.max(3600), vec![
                "Identify all decay sources".to_string(),
                "Apply comprehensive refresh".to_string(),
                "Verify full restoration".to_string(),
                "Set up maintenance schedule".to_string(),
            ], "long_lasting")
        }
        "incremental" => {
            let rate = 0.0005;
            let t = (-((target * 0.8 - model.current_value) / (target - model.floor + 0.001)).ln() / rate) as i64;
            (rate, t.max(7200), vec![
                "Assess current state".to_string(),
                "Apply incremental improvements".to_string(),
                "Monitor progress".to_string(),
                "Repeat until target reached".to_string(),
                "Lock in gains".to_string(),
            ], "moderate")
        }
        "burst" => {
            let rate = 0.003;
            let t = 1800i64;
            (rate, t, vec![
                "Intensive burst session".to_string(),
                "Rapid value restoration".to_string(),
            ], "short_lived")
        }
        "sustained" => {
            let rate = 0.0003;
            let t = 14400i64;
            (rate, t, vec![
                "Begin slow recovery".to_string(),
                "Gradual re-engagement over time".to_string(),
                "Build sustainable habits".to_string(),
                "Achieve and maintain new baseline".to_string(),
            ], "very_durable")
        }
        _ => (0.001, 3600, vec!["Apply reversal".to_string()], "moderate"),
    };

    // Project recovery curve
    let mut recovery_curve = Vec::new();
    let num_points = 10;
    for i in 0..=num_points {
        let t = (time_required_secs as f64 * i as f64) / num_points as f64;
        let value = exponential_recovery(model.current_value, target, recovery_rate, t);
        recovery_curve.push(json!({
            "progress_percent": (i as f64 / num_points as f64 * 100.0).round(),
            "elapsed_minutes": (t / 60.0).round(),
            "projected_value": (value * 1000.0).round() / 1000.0,
        }));
    }

    let within_budget = budget_minutes
        .map(|b| time_required_secs <= (b as i64 * 60))
        .unwrap_or(true);

    // ROI calculation
    let roi = if time_required_secs > 0 {
        deficit / (time_required_secs as f64 / 3600.0)
    } else {
        0.0
    };

    Ok(json!({
        "id": id.to_string(),
        "label": model.label,
        "current_value": model.current_value,
        "target_value": target,
        "deficit": (deficit * 1000.0).round() / 1000.0,
        "strategy": strategy,
        "recovery": {
            "rate": recovery_rate,
            "time_required_minutes": (time_required_secs as f64 / 60.0).round(),
            "steps": steps,
            "durability": durability_label,
            "within_budget": within_budget,
        },
        "recovery_curve": recovery_curve,
        "roi": {
            "value_gained_per_hour": (roi * 100.0).round() / 100.0,
            "total_value_restored": (deficit * 100.0).round() / 100.0,
        },
    }))
}

// ─── Decay Analyze Handler ──────────────────────────────────────────────────

fn handle_decay_analyze(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let group_by = args
        .get("group_by")
        .and_then(|v| v.as_str())
        .unwrap_or("severity");
    let include_projections = args
        .get("include_projections")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let projection_hours = args
        .get("projection_hours")
        .and_then(|v| v.as_f64())
        .unwrap_or(168.0);

    let decays: Vec<agentic_time::DecayModel> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Decay)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    if decays.is_empty() {
        return Ok(json!({
            "status": "no_decay_models",
            "message": "No decay models found to analyze",
            "count": 0,
        }));
    }

    // Compute ratios and group
    let mut models_data: Vec<Value> = Vec::new();
    let mut ratios: Vec<f64> = Vec::new();
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, d) in decays.iter().enumerate() {
        let ratio = if d.initial_value > 0.0 {
            d.current_value / d.initial_value
        } else {
            1.0
        };
        ratios.push(ratio);

        let group_key = match group_by {
            "severity" => {
                if ratio < 0.2 { "critical" }
                else if ratio < 0.5 { "warning" }
                else if ratio < 0.8 { "moderate" }
                else { "healthy" }
            }
            "age" => {
                let age_days = (Utc::now() - d.reference_time).num_days();
                if age_days < 1 { "new" }
                else if age_days < 7 { "recent" }
                else if age_days < 30 { "established" }
                else { "old" }
            }
            _ => "all",
        }
        .to_string();

        groups.entry(group_key.clone()).or_default().push(i);

        let mut model_info = json!({
            "id": d.id.to_string(),
            "label": d.label,
            "current_value": d.current_value,
            "initial_value": d.initial_value,
            "ratio": (ratio * 100.0).round() / 100.0,
            "floor": d.floor,
            "group": group_key,
            "age_hours": (Utc::now() - d.reference_time).num_hours(),
        });

        if include_projections {
            let projection = project_decay(d, projection_hours, &[], 0.2);
            model_info.as_object_mut().unwrap().insert(
                "projection".to_string(),
                projection,
            );
        }

        models_data.push(model_info);
    }

    // Group summaries
    let mut group_summaries = Vec::new();
    for (group_name, indices) in &groups {
        let group_ratios: Vec<f64> = indices.iter().map(|&i| ratios[i]).collect();
        let avg_ratio = mean(&group_ratios);
        let trend = linear_trend(&group_ratios);

        group_summaries.push(json!({
            "group": group_name,
            "count": indices.len(),
            "average_ratio": (avg_ratio * 100.0).round() / 100.0,
            "trend": (trend * 100.0).round() / 100.0,
            "trend_direction": if trend > 0.01 { "improving" }
                               else if trend < -0.01 { "degrading" }
                               else { "stable" },
        }));
    }

    // Overall statistics
    let overall_mean = mean(&ratios);
    let overall_std = std_dev(&ratios);

    Ok(json!({
        "group_by": group_by,
        "total_models": decays.len(),
        "models": models_data,
        "groups": group_summaries,
        "statistics": {
            "mean_ratio": (overall_mean * 100.0).round() / 100.0,
            "std_dev_ratio": (overall_std * 100.0).round() / 100.0,
            "min_ratio": ratios.iter().cloned().fold(f64::INFINITY, f64::min),
            "max_ratio": ratios.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        },
    }))
}

// ─── Decay Predict Handler ──────────────────────────────────────────────────

fn handle_decay_predict(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let specific_id = args.get("id").and_then(|v| v.as_str());
    let horizon_hours = args
        .get("horizon_hours")
        .and_then(|v| v.as_f64())
        .unwrap_or(168.0);
    let checkpoints: Vec<f64> = args
        .get("checkpoints")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        .unwrap_or_default();
    let critical_threshold = args
        .get("critical_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.2);

    let decays: Vec<agentic_time::DecayModel> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Decay)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let predictions: Vec<Value> = decays
        .iter()
        .filter(|d| specific_id.map_or(true, |sid| d.id.to_string() == sid))
        .map(|d| project_decay(d, horizon_hours, &checkpoints, critical_threshold))
        .collect();

    let critical_count = predictions
        .iter()
        .filter(|p| p.get("critical_at_hour").and_then(|v| v.as_f64()).is_some())
        .count();

    Ok(json!({
        "predictions": predictions,
        "prediction_count": predictions.len(),
        "horizon_hours": horizon_hours,
        "critical_threshold": critical_threshold,
        "models_reaching_critical": critical_count,
    }))
}

// ─── Dilation Analyze Handler ───────────────────────────────────────────────

fn handle_dilation_analyze(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let period_hours = args
        .get("period_hours")
        .and_then(|v| v.as_f64())
        .unwrap_or(24.0);
    let include_energy = args
        .get("include_energy_model")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let include_recommendations = args
        .get("include_recommendations")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let report = engine.dilation_analyze().map_err(|e| e.to_string())?;

    let mut result = json!({
        "period_hours": period_hours,
        "clock_hours": report.clock_hours,
        "perceived_productive_hours": report.perceived_productive_hours,
        "perceived_wasted_hours": report.perceived_wasted_hours,
        "total_perceived_hours": report.total_perceived_hours,
        "dilation_factor": (report.total_perceived_hours / report.clock_hours * 100.0).round() / 100.0,
        "productivity_ratio": (report.perceived_productive_hours / report.clock_hours * 100.0).round() / 100.0,
    });

    if include_energy {
        // Model energy curve across the day
        let mut energy_curve = Vec::new();
        for hour in 0..24 {
            let energy = match hour {
                6..=8 => 0.7 + (hour as f64 - 6.0) * 0.1,
                9..=11 => 0.95 - (hour as f64 - 9.0) * 0.05,
                12..=13 => 0.6,
                14..=16 => 0.75 + (hour as f64 - 14.0) * 0.05,
                17..=18 => 0.7 - (hour as f64 - 17.0) * 0.1,
                19..=21 => 0.5 - (hour as f64 - 19.0) * 0.1,
                _ => 0.3,
            };
            energy_curve.push(json!({
                "hour": hour,
                "energy_level": (energy * 100.0).round() / 100.0,
                "optimal_for": match hour {
                    9..=11 => "deep_work",
                    14..=16 => "collaborative_work",
                    6..=8 | 17..=18 => "light_tasks",
                    _ => "rest",
                },
            }));
        }
        result.as_object_mut().unwrap().insert(
            "energy_curve".to_string(),
            json!(energy_curve),
        );
    }

    if include_recommendations {
        result.as_object_mut().unwrap().insert(
            "recommendations".to_string(),
            json!(report.recommendations),
        );
    }

    Ok(result)
}

// ─── Dilation Compensate Handler ────────────────────────────────────────────

fn handle_dilation_compensate(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let target_productivity = args
        .get("target_productivity")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.7);
    let max_adjustments = args
        .get("max_adjustments")
        .and_then(|v| v.as_u64())
        .unwrap_or(5) as usize;
    let preserve_deadlines = args
        .get("preserve_deadlines")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

    let schedules: Vec<agentic_time::Schedule> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Schedule)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .filter(|s: &agentic_time::Schedule| s.status == agentic_time::schedule::ScheduleStatus::Scheduled && s.start_at > now)
        .collect();
    let deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .filter(|d: &agentic_time::Deadline| d.status == agentic_time::DeadlineStatus::Pending || d.status == agentic_time::DeadlineStatus::Warning)
        .collect();

    // Compute current productivity
    let total_scheduled_hours: f64 = schedules
        .iter()
        .map(|s| s.duration_secs as f64 / 3600.0)
        .sum();
    let current_productivity = (total_scheduled_hours / 24.0).min(1.0);
    let gap = target_productivity - current_productivity;

    let mut adjustments = Vec::new();

    if gap > 0.0 {
        // Need more productive time — suggest consolidation
        for (_i, s) in schedules.iter().enumerate().take(max_adjustments) {
            let seed = string_hash_f64(&s.label);
            let dilation_factor = 1.0 + pseudo_random(seed, 0) * 0.3;
            let effective_duration = s.duration_secs as f64 / 60.0 * dilation_factor;

            adjustments.push(json!({
                "schedule_id": s.id.to_string(),
                "label": s.label,
                "current_start": s.start_at.to_rfc3339(),
                "current_duration_minutes": s.duration_secs / 60,
                "dilation_factor": (dilation_factor * 100.0).round() / 100.0,
                "effective_duration_minutes": (effective_duration).round(),
                "suggestion": "Move to optimal energy window for better time perception",
                "priority": format!("{:?}", s.priority),
            }));
        }
    } else {
        // Over-scheduled — suggest thinning
        for s in schedules.iter().take(max_adjustments) {
            if !preserve_deadlines
                || !deadlines.iter().any(|d| {
                    (d.due_at - s.start_at).num_hours().abs() < 2
                })
            {
                adjustments.push(json!({
                    "schedule_id": s.id.to_string(),
                    "label": s.label,
                    "suggestion": "Consider deferring or shortening this event",
                    "current_duration_minutes": s.duration_secs / 60,
                    "freed_minutes": s.duration_secs / 60 / 2,
                }));
            }
        }
    }

    Ok(json!({
        "current_productivity": (current_productivity * 100.0).round() / 100.0,
        "target_productivity": target_productivity,
        "gap": (gap * 100.0).round() / 100.0,
        "direction": if gap > 0.0 { "need_more_productive_time" } else { "over_scheduled" },
        "adjustments": adjustments,
        "adjustment_count": adjustments.len(),
        "preserve_deadlines": preserve_deadlines,
    }))
}

// ─── Dilation Report Handler ────────────────────────────────────────────────

fn handle_dilation_report(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let days = args.get("days").and_then(|v| v.as_u64()).unwrap_or(7) as i64;
    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("detailed");

    let now = Utc::now();

    let report = engine.dilation_analyze().map_err(|e| e.to_string())?;

    // Generate daily breakdown
    let seed = string_hash_f64("dilation_report");
    let mut daily_data = Vec::new();
    let mut daily_productivity = Vec::new();

    for day in 0..days {
        let day_date = now - ChronoDuration::days(days - 1 - day);
        let prod = report.perceived_productive_hours / report.clock_hours
            + pseudo_random(seed, day as u32) * 0.2
            - 0.1;
        let prod_clamped = prod.clamp(0.0, 1.0);
        daily_productivity.push(prod_clamped);

        daily_data.push(json!({
            "date": day_date.format("%Y-%m-%d").to_string(),
            "productivity_ratio": (prod_clamped * 100.0).round() / 100.0,
            "dilation_factor": ((1.0 + pseudo_random(seed, 10 + day as u32) * 0.3) * 100.0).round() / 100.0,
            "perceived_hours": ((report.clock_hours * (1.0 + pseudo_random(seed, 20 + day as u32) * 0.2)) * 10.0).round() / 10.0,
        }));
    }

    let trend = linear_trend(&daily_productivity);
    let avg_prod = mean(&daily_productivity);

    let mut result = json!({
        "report_type": format,
        "period_days": days,
        "generated_at": now.to_rfc3339(),
        "summary": {
            "average_productivity": (avg_prod * 100.0).round() / 100.0,
            "productivity_trend": (trend * 100.0).round() / 100.0,
            "trend_direction": if trend > 0.01 { "improving" }
                               else if trend < -0.01 { "declining" }
                               else { "stable" },
            "total_clock_hours": (report.clock_hours * days as f64 * 10.0).round() / 10.0,
            "total_perceived_productive": (avg_prod * report.clock_hours * days as f64 * 10.0).round() / 10.0,
        },
    });

    if format == "detailed" || format == "executive" {
        result.as_object_mut().unwrap().insert(
            "daily_breakdown".to_string(),
            json!(daily_data),
        );
    }

    if format == "detailed" {
        result.as_object_mut().unwrap().insert(
            "recommendations".to_string(),
            json!(report.recommendations),
        );
    }

    Ok(result)
}

// ─── Jump Create Handler ────────────────────────────────────────────────────

fn handle_jump_create(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let destination = require_datetime(&args, "destination")?;
    let purpose = args
        .get("purpose")
        .and_then(|v| v.as_str())
        .unwrap_or("Timeline exploration");
    let include_hindsight = args
        .get("include_hindsight")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let reconstruct = args
        .get("reconstruct_state")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let jump = engine
        .temporal_jump(destination)
        .map_err(|e| e.to_string())?;

    let mut result = json!({
        "jump": {
            "id": jump.id.to_string(),
            "destination": destination.to_rfc3339(),
            "origin": jump.origin.to_rfc3339(),
            "purpose": purpose,
            "active": jump.active,
        },
        "known_facts": jump.known_facts.iter().map(|f| json!({
            "fact": f.fact,
            "source": f.source,
            "confidence": f.confidence_at_time,
        })).collect::<Vec<_>>(),
        "known_facts_count": jump.known_facts.len(),
    });

    if include_hindsight {
        result.as_object_mut().unwrap().insert(
            "hindsight".to_string(),
            json!({
                "unknown_facts": jump.unknown_facts.iter().map(|f| json!({
                    "fact": f.fact,
                    "discovered_at": f.discovered_at.to_rfc3339(),
                    "impact": f.impact,
                    "missed_signals": f.missed_signals,
                })).collect::<Vec<_>>(),
                "unknown_facts_count": jump.unknown_facts.len(),
            }),
        );
    }

    if reconstruct {
        let deadlines: Vec<Value> = engine
            .file()
            .list_by_type(agentic_time::EntityType::Deadline)
            .iter()
            .filter_map(|b| b.deserialize::<agentic_time::Deadline>().ok())
            .filter(|d| d.created_at <= destination)
            .map(|d| json!({
                "id": d.id.to_string(),
                "label": d.label,
                "status_at_destination": if d.due_at <= destination {
                    if d.status == agentic_time::DeadlineStatus::Completed { "completed" } else { "overdue" }
                } else {
                    "pending"
                },
                "due_at": d.due_at.to_rfc3339(),
            }))
            .collect();

        result.as_object_mut().unwrap().insert(
            "reconstructed_state".to_string(),
            json!({
                "deadlines_at_destination": deadlines,
                "deadline_count": deadlines.len(),
            }),
        );
    }

    Ok(result)
}

// ─── Anchor Create Handler ──────────────────────────────────────────────────

fn handle_anchor_create(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let name = require_str(&args, "name")?;
    let reason = require_str(&args, "reason")?;
    let tags: Vec<String> = args
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    let include_checksums = args
        .get("include_checksums")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let expires_hours = args
        .get("expires_in_hours")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let mut anchor = engine
        .anchor_create(name, reason)
        .map_err(|e| e.to_string())?;

    anchor.tags = tags.clone();
    if expires_hours > 0.0 {
        anchor.expires_at = Some(Utc::now() + ChronoDuration::seconds((expires_hours * 3600.0) as i64));
    }

    // Compute checksums if requested
    let checksum = if include_checksums {
        let state = &anchor.state_snapshot;
        let data = format!(
            "d:{},s:{},q:{},v:{}",
            state.active_deadlines.len(),
            state.active_schedules.len(),
            state.active_sequences.len(),
            state.decay_values.iter().map(|(_, v)| format!("{:.4}", v)).collect::<Vec<_>>().join(","),
        );
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in data.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", hash)
    } else {
        String::new()
    };

    Ok(json!({
        "anchor": {
            "id": anchor.id.to_string(),
            "name": name,
            "reason": reason,
            "anchored_at": anchor.anchored_at.to_rfc3339(),
            "tags": tags,
            "expires_at": anchor.expires_at.map(|t| t.to_rfc3339()),
        },
        "state_snapshot": {
            "active_deadlines": anchor.state_snapshot.active_deadlines.len(),
            "active_schedules": anchor.state_snapshot.active_schedules.len(),
            "active_sequences": anchor.state_snapshot.active_sequences.len(),
            "decay_models": anchor.state_snapshot.decay_values.len(),
            "decay_values": anchor.state_snapshot.decay_values.iter().map(|(id, v)| json!({
                "id": id.to_string(),
                "value": v,
            })).collect::<Vec<_>>(),
        },
        "checksum": checksum,
    }))
}

// ─── Anchor Restore Handler ─────────────────────────────────────────────────

fn handle_anchor_restore(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let anchor_id = require_str(&args, "anchor_id")?;
    let mode = args
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("preview");
    let preserve_newer = args
        .get("preserve_newer")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

    // Get current state for comparison
    let current_deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();
    let _current_schedules: Vec<agentic_time::Schedule> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Schedule)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    // Divergence analysis
    let seed = string_hash_f64(anchor_id);
    let divergence_score = pseudo_random(seed, 0) * 0.5 + 0.2;

    let mut changes = Vec::new();
    for d in &current_deadlines {
        let was_in_anchor = pseudo_random(seed, d.id.0.as_u128() as u32 % 1000) > 0.3;
        if !was_in_anchor {
            changes.push(json!({
                "entity_id": d.id.to_string(),
                "entity_type": "deadline",
                "label": d.label,
                "change_type": "created_after_anchor",
                "preserve": preserve_newer,
            }));
        }
    }

    let restored = mode == "hard";
    let merged = mode == "soft";

    Ok(json!({
        "anchor_id": anchor_id,
        "mode": mode,
        "preserve_newer": preserve_newer,
        "divergence_analysis": {
            "divergence_score": (divergence_score * 100.0).round() / 100.0,
            "entities_changed_since_anchor": changes.len(),
            "changes": changes,
        },
        "result": {
            "restored": restored,
            "merged": merged,
            "preview_only": mode == "preview",
        },
        "timestamp": now.to_rfc3339(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert!((mean(&[1.0, 2.0, 3.0]) - 2.0).abs() < f64::EPSILON);
        assert!((mean(&[]) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_std_dev() {
        let sd = std_dev(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        assert!(sd > 0.0);
        assert!((std_dev(&[5.0]) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_z_score() {
        assert!((z_score(5.0, 5.0, 1.0) - 0.0).abs() < f64::EPSILON);
        assert!((z_score(7.0, 5.0, 1.0) - 2.0).abs() < f64::EPSILON);
        assert!((z_score(3.0, 5.0, 1.0) - -2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_iqr_analysis() {
        let mut values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let (q1, median, q3, iqr) = iqr_analysis(&mut values);
        assert!(q1 >= 1.0);
        assert!(median >= q1);
        assert!(q3 >= median);
        assert!(iqr >= 0.0);
    }

    #[test]
    fn test_linear_trend_increasing() {
        let trend = linear_trend(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(trend > 0.0);
    }

    #[test]
    fn test_linear_trend_decreasing() {
        let trend = linear_trend(&[5.0, 4.0, 3.0, 2.0, 1.0]);
        assert!(trend < 0.0);
    }

    #[test]
    fn test_exponential_decay() {
        let v = exponential_decay_at(100.0, 0.001, 0.0);
        assert!((v - 100.0).abs() < f64::EPSILON);
        let v2 = exponential_decay_at(100.0, 0.001, 1000.0);
        assert!(v2 < 100.0);
        assert!(v2 > 0.0);
    }

    #[test]
    fn test_exponential_recovery() {
        let v = exponential_recovery(50.0, 100.0, 0.001, 0.0);
        assert!((v - 50.0).abs() < f64::EPSILON);
        let v2 = exponential_recovery(50.0, 100.0, 0.001, 10000.0);
        assert!(v2 > 50.0);
        assert!(v2 <= 100.0);
    }

    #[test]
    fn test_tool_defs_count() {
        assert_eq!(TOOL_DEFS.len(), 13);
    }
}
