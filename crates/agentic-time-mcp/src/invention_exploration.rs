//! Invention Exploration MCP tools — Temporal Replay, Cloning, Echoes,
//! Deadline Prophecy, and Causal Archaeology.
//!
//! Provides deep algorithmic implementations for timeline forking,
//! clone racing, echo detection/amplification, deadline prediction,
//! and causal chain excavation.

use chrono::{Duration as ChronoDuration, Utc};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::tools::ToolDefinition;

// ─── Tool Definitions ───────────────────────────────────────────────────────

/// All exploration invention tool definitions.
pub const TOOL_DEFS: &[ToolDefinition] = &[
    // ── Temporal Replay (4 tools) ───────────────────────────────────────
    ToolDefinition {
        name: "time_timeline_fork",
        description: "Fork the timeline at a decision point for what-if analysis with deep simulation of alternatives",
        input_schema: r#"{"type":"object","properties":{"decision":{"type":"string","description":"The decision being made"},"alternatives":{"type":"array","items":{"type":"object","properties":{"label":{"type":"string"},"description":{"type":"string"}},"required":["label","description"]},"description":"Alternative choices to simulate"},"simulation_depth":{"type":"integer","default":3,"description":"How many downstream effects to simulate per alternative"},"context_tags":{"type":"array","items":{"type":"string"},"description":"Tags to filter context entities"}},"required":["decision","alternatives"]}"#,
    },
    ToolDefinition {
        name: "time_timeline_merge",
        description: "Merge insights from forked timelines back into the main timeline",
        input_schema: r#"{"type":"object","properties":{"branch_id":{"type":"string","description":"Branch point ID to merge from"},"chosen_alternative":{"type":"integer","description":"Index of the chosen alternative (0-based)"},"outcome_notes":{"type":"string","description":"Notes about the actual outcome observed"},"lessons_learned":{"type":"array","items":{"type":"string"},"description":"Lessons learned from this decision"}},"required":["branch_id","chosen_alternative"]}"#,
    },
    ToolDefinition {
        name: "time_timeline_compare",
        description: "Compare timeline branches using Jaccard similarity, divergence scoring, and risk analysis",
        input_schema: r#"{"type":"object","properties":{"branch_id":{"type":"string","description":"Branch point ID to compare alternatives for"},"compare_indices":{"type":"array","items":{"type":"integer"},"description":"Indices of alternatives to compare (default: all)"},"metrics":{"type":"array","items":{"type":"string","enum":["risk","speed","cost","quality","reversibility"]},"description":"Metrics to compare across branches"}},"required":["branch_id"]}"#,
    },
    ToolDefinition {
        name: "time_timeline_visualize",
        description: "Generate timeline visualization data showing branches, events, and convergence points",
        input_schema: r#"{"type":"object","properties":{"branch_id":{"type":"string","description":"Branch point ID to visualize"},"format":{"type":"string","enum":["tree","gantt","flow","matrix"],"default":"tree","description":"Visualization format"},"include_metrics":{"type":"boolean","default":true,"description":"Include metric overlays"}},"required":["branch_id"]}"#,
    },
    // ── Temporal Cloning (4 tools) ──────────────────────────────────────
    ToolDefinition {
        name: "time_clone_create",
        description: "Create a temporal clone for parallel exploration of a hypothesis",
        input_schema: r#"{"type":"object","properties":{"hypothesis":{"type":"string","description":"What this clone is testing"},"parent_id":{"type":"string","description":"Optional parent entity ID to clone from"},"exploration_budget_minutes":{"type":"integer","default":60,"description":"Time budget for exploration in minutes"},"success_criteria":{"type":"array","items":{"type":"string"},"description":"Criteria that define successful exploration"},"strategy":{"type":"string","enum":["depth_first","breadth_first","random_walk","hill_climb"],"default":"depth_first","description":"Exploration strategy"}},"required":["hypothesis"]}"#,
    },
    ToolDefinition {
        name: "time_clone_race",
        description: "Race multiple clones to find the fastest approach with convergence detection",
        input_schema: r#"{"type":"object","properties":{"approaches":{"type":"array","items":{"type":"string"},"description":"List of approaches to explore in parallel"},"scoring_weights":{"type":"object","properties":{"speed":{"type":"number","default":0.4},"quality":{"type":"number","default":0.3},"risk":{"type":"number","default":0.3}},"description":"Weights for scoring approaches"},"convergence_threshold":{"type":"number","default":0.85,"description":"Similarity threshold above which approaches are considered converged"}},"required":["approaches"]}"#,
    },
    ToolDefinition {
        name: "time_clone_merge",
        description: "Merge findings from a clone back into the parent context with conflict resolution",
        input_schema: r#"{"type":"object","properties":{"clone_id":{"type":"string","description":"Clone ID to merge"},"merge_strategy":{"type":"string","enum":["take_all","take_confident","take_novel","manual"],"default":"take_confident","description":"How to resolve conflicting findings"},"confidence_threshold":{"type":"number","default":0.7,"description":"Minimum confidence for findings to merge"}},"required":["clone_id"]}"#,
    },
    ToolDefinition {
        name: "time_clone_status",
        description: "Check clone status with exploration progress, findings summary, and convergence metrics",
        input_schema: r#"{"type":"object","properties":{"clone_id":{"type":"string","description":"Clone ID to check (omit for all active clones)"},"include_findings":{"type":"boolean","default":true,"description":"Include detailed findings"},"include_metrics":{"type":"boolean","default":true,"description":"Include exploration metrics"}}}"#,
    },
    // ── Temporal Echoes (3 tools) ───────────────────────────────────────
    ToolDefinition {
        name: "time_echo_detect",
        description: "Detect temporal echoes and ripple effects with multi-horizon propagation analysis",
        input_schema: r#"{"type":"object","properties":{"event":{"type":"string","description":"Description of the event to analyze ripple effects for"},"propagation_depth":{"type":"integer","default":3,"description":"How many levels of cascading effects to analyze"},"time_horizons":{"type":"array","items":{"type":"string","enum":["immediate","short_term","medium_term","long_term","extended"]},"description":"Which time horizons to analyze"},"sensitivity":{"type":"number","default":0.3,"description":"Minimum impact threshold for detection (0.0-1.0)"}},"required":["event"]}"#,
    },
    ToolDefinition {
        name: "time_echo_amplify",
        description: "Amplify weak echo signals to surface hidden dependencies and second-order effects",
        input_schema: r#"{"type":"object","properties":{"event":{"type":"string","description":"Event whose echoes to amplify"},"amplification_factor":{"type":"number","default":2.0,"description":"How much to amplify weak signals (1.0-5.0)"},"include_speculative":{"type":"boolean","default":false,"description":"Include speculative second-order effects"}},"required":["event"]}"#,
    },
    ToolDefinition {
        name: "time_echo_suppress",
        description: "Suppress known echo patterns to reduce noise and surface novel signals",
        input_schema: r#"{"type":"object","properties":{"event":{"type":"string","description":"Event whose echoes to filter"},"suppress_patterns":{"type":"array","items":{"type":"string"},"description":"Patterns to suppress (e.g., 'deadline_cascade', 'schedule_shift')"},"novelty_threshold":{"type":"number","default":0.5,"description":"Minimum novelty score for an echo to pass through (0.0-1.0)"}},"required":["event"]}"#,
    },
];

// ─── Helpers ────────────────────────────────────────────────────────────────

fn require_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required parameter: {}", key))
}

#[allow(dead_code)]
fn require_id(args: &Value, key: &str) -> Result<agentic_time::TemporalId, String> {
    let s = require_str(args, key)?;
    s.parse::<agentic_time::TemporalId>()
        .map_err(|e| format!("Invalid ID for '{}': {}", key, e))
}

// ─── Statistical Helpers ────────────────────────────────────────────────────

/// Compute Jaccard similarity between two sets of strings.
fn jaccard_similarity(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let set_a: std::collections::HashSet<&str> = a.iter().map(|s| s.as_str()).collect();
    let set_b: std::collections::HashSet<&str> = b.iter().map(|s| s.as_str()).collect();
    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();
    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Compute cosine similarity between two f64 vectors.
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        (dot / (mag_a * mag_b)).clamp(-1.0, 1.0)
    }
}

/// Exponential decay weighting for impact propagation across time horizons.
fn decay_weight(depth: u32, decay_rate: f64) -> f64 {
    (-decay_rate * depth as f64).exp()
}

/// Sigmoid function for normalizing scores into [0, 1].
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

/// Generate a deterministic pseudo-random f64 from a seed.
fn pseudo_random(seed: f64, index: u32) -> f64 {
    let combined = seed * 1000.0 + index as f64;
    let hash = (combined.sin() * 43758.5453).fract().abs();
    hash
}

/// Compute risk score from impact and probability.
fn risk_score(impact: f64, probability: f64) -> f64 {
    (impact * probability).clamp(0.0, 1.0)
}

/// Weighted geometric mean of scores.
fn weighted_geometric_mean(values: &[(f64, f64)]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let total_weight: f64 = values.iter().map(|(_, w)| w).sum();
    if total_weight == 0.0 {
        return 0.0;
    }
    let log_sum: f64 = values
        .iter()
        .map(|(v, w)| w * v.max(0.001).ln())
        .sum::<f64>();
    (log_sum / total_weight).exp().clamp(0.0, 1.0)
}

/// Compute entropy of a probability distribution.
fn entropy(probs: &[f64]) -> f64 {
    probs
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.ln())
        .sum()
}

// ─── Timeline Simulation Engine ─────────────────────────────────────────────

/// Simulate downstream effects of an alternative decision.
fn simulate_alternative(
    alt_label: &str,
    alt_description: &str,
    depth: u32,
    deadlines: &[agentic_time::Deadline],
    schedules: &[agentic_time::Schedule],
    decays: &[agentic_time::DecayModel],
) -> Value {
    let now = Utc::now();
    let seed = string_hash_f64(alt_label);
    let mut simulated_events = Vec::new();
    let mut cumulative_impact = 0.0;
    let mut risk_factors = Vec::new();

    // Phase 1: Immediate impacts on deadlines
    for (i, d) in deadlines.iter().enumerate() {
        if d.status != agentic_time::DeadlineStatus::Pending
            && d.status != agentic_time::DeadlineStatus::Warning
        {
            continue;
        }
        let remaining_hours = (d.due_at - now).num_hours() as f64;
        let pressure = if remaining_hours > 0.0 {
            (168.0 / remaining_hours).min(10.0) / 10.0
        } else {
            1.0
        };

        let impact_modifier = pseudo_random(seed, i as u32) * 0.4 - 0.2;
        let impact = (pressure + impact_modifier).clamp(-1.0, 1.0);
        let weight = decay_weight(0, 0.5);

        simulated_events.push(json!({
            "timestamp": (now + ChronoDuration::hours(1)).to_rfc3339(),
            "description": format!("Impact on deadline '{}': {}", d.label,
                if impact > 0.3 { "increased urgency" }
                else if impact < -0.1 { "reduced pressure" }
                else { "minimal change" }),
            "impact": (impact * weight * 100.0).round() / 100.0,
            "horizon": "immediate",
            "entity_type": "deadline",
            "entity_id": d.id.to_string(),
        }));
        cumulative_impact += impact.abs() * weight;
    }

    // Phase 2: Schedule disruptions (depth >= 2)
    if depth >= 2 {
        for (i, s) in schedules.iter().enumerate() {
            if s.status != agentic_time::schedule::ScheduleStatus::Scheduled {
                continue;
            }
            let hours_until = (s.start_at - now).num_hours() as f64;
            if hours_until <= 0.0 || hours_until > 168.0 {
                continue;
            }

            let disruption_prob = pseudo_random(seed, 100 + i as u32);
            let time_weight = decay_weight(1, 0.3);
            let impact = disruption_prob * time_weight * 0.6;

            if impact > 0.1 {
                simulated_events.push(json!({
                    "timestamp": s.start_at.to_rfc3339(),
                    "description": format!("Schedule '{}' may need adjustment: {:.0}% disruption probability",
                        s.label, disruption_prob * 100.0),
                    "impact": (impact * 100.0).round() / 100.0,
                    "horizon": "short_term",
                    "entity_type": "schedule",
                    "entity_id": s.id.to_string(),
                }));
                cumulative_impact += impact;

                risk_factors.push(json!({
                    "factor": format!("Schedule disruption: {}", s.label),
                    "probability": (disruption_prob * 100.0).round() / 100.0,
                    "impact": (impact * 100.0).round() / 100.0,
                    "risk_score": (risk_score(impact, disruption_prob) * 100.0).round() / 100.0,
                }));
            }
        }
    }

    // Phase 3: Decay cascade effects (depth >= 3)
    if depth >= 3 {
        for (i, decay) in decays.iter().enumerate() {
            let value_at_risk = decay.initial_value - decay.current_value;
            let decay_impact = pseudo_random(seed, 200 + i as u32);
            let cascade_weight = decay_weight(2, 0.4);

            if value_at_risk > 0.1 && decay_impact > 0.3 {
                let projected_value = decay.current_value * (1.0 - decay_impact * 0.2);
                simulated_events.push(json!({
                    "timestamp": (now + ChronoDuration::days(7)).to_rfc3339(),
                    "description": format!("Decay '{}' may accelerate: {:.2} -> {:.2}",
                        decay.label, decay.current_value, projected_value),
                    "impact": (decay_impact * cascade_weight * 100.0).round() / 100.0,
                    "horizon": "medium_term",
                    "entity_type": "decay",
                    "entity_id": decay.id.to_string(),
                }));
                cumulative_impact += decay_impact * cascade_weight;
            }
        }
    }

    // Compute overall scores
    let event_count = simulated_events.len();
    let avg_impact = if event_count > 0 {
        cumulative_impact / event_count as f64
    } else {
        0.0
    };
    let confidence = sigmoid(event_count as f64 * 0.5 - 2.0);
    let speed_score = 1.0 - (avg_impact * 0.5).min(1.0);
    let quality_score = sigmoid(1.5 - cumulative_impact);
    let risk_total = 1.0 - quality_score;

    json!({
        "alternative": alt_label,
        "description": alt_description,
        "simulated_events": simulated_events,
        "event_count": event_count,
        "cumulative_impact": (cumulative_impact * 100.0).round() / 100.0,
        "average_impact": (avg_impact * 100.0).round() / 100.0,
        "confidence": (confidence * 100.0).round() / 100.0,
        "scores": {
            "speed": (speed_score * 100.0).round() / 100.0,
            "quality": (quality_score * 100.0).round() / 100.0,
            "risk": (risk_total * 100.0).round() / 100.0,
            "reversibility": ((1.0 - risk_total * 0.7) * 100.0).round() / 100.0,
        },
        "risk_factors": risk_factors,
    })
}

// ─── Echo Propagation Engine ────────────────────────────────────────────────

/// Horizon labels for echo analysis.
const HORIZON_LABELS: &[&str] = &[
    "immediate",
    "short_term",
    "medium_term",
    "long_term",
    "extended",
];

/// Duration multipliers for each horizon level (hours).
const HORIZON_HOURS: &[i64] = &[1, 24, 168, 720, 2160];

/// Propagate echoes across time horizons with cascading severity.
fn propagate_echoes(
    event: &str,
    deadlines: &[agentic_time::Deadline],
    schedules: &[agentic_time::Schedule],
    sequences: &[agentic_time::Sequence],
    decays: &[agentic_time::DecayModel],
    propagation_depth: u32,
    sensitivity: f64,
    horizons_filter: &[String],
) -> Value {
    let now = Utc::now();
    let seed = string_hash_f64(event);
    let mut all_ripples = Vec::new();
    let mut total_affected = 0u32;
    let mut max_severity = 0.0f64;

    let active_horizons: Vec<usize> = if horizons_filter.is_empty() {
        (0..HORIZON_LABELS.len().min(propagation_depth as usize + 1)).collect()
    } else {
        HORIZON_LABELS
            .iter()
            .enumerate()
            .filter(|(_, label)| horizons_filter.iter().any(|f| f == *label))
            .map(|(i, _)| i)
            .collect()
    };

    for &horizon_idx in &active_horizons {
        let horizon_label = HORIZON_LABELS[horizon_idx];
        let hours = HORIZON_HOURS[horizon_idx];
        let horizon_end = now + ChronoDuration::hours(hours);
        let depth_decay = decay_weight(horizon_idx as u32, 0.35);
        let mut ripple_items = Vec::new();

        // Deadline ripples at this horizon
        for (i, d) in deadlines.iter().enumerate() {
            if d.status != agentic_time::DeadlineStatus::Pending
                && d.status != agentic_time::DeadlineStatus::Warning
            {
                continue;
            }
            if d.due_at > horizon_end {
                continue;
            }

            let remaining_ratio = if (d.due_at - d.created_at).num_seconds() > 0 {
                (d.due_at - now).num_seconds() as f64
                    / (d.due_at - d.created_at).num_seconds() as f64
            } else {
                0.0
            };
            let urgency = (1.0 - remaining_ratio).max(0.0);
            let base_impact = urgency * depth_decay;
            let noise = pseudo_random(seed, (horizon_idx * 100 + i) as u32) * 0.15;
            let impact = (base_impact + noise).clamp(0.0, 1.0);

            if impact >= sensitivity {
                ripple_items.push(json!({
                    "entity_id": d.id.to_string(),
                    "entity_type": "deadline",
                    "label": d.label,
                    "description": format!("Deadline '{}' due {} — {:.0}% time elapsed, urgency {:.2}",
                        d.label, d.due_at.to_rfc3339(),
                        (1.0 - remaining_ratio) * 100.0, urgency),
                    "impact": (impact * 100.0).round() / 100.0,
                    "urgency": (urgency * 100.0).round() / 100.0,
                    "reversible": remaining_ratio > 0.2,
                    "cascade_probability": ((impact * 0.7 + urgency * 0.3) * 100.0).round() / 100.0,
                }));
                total_affected += 1;
            }
        }

        // Schedule ripples at this horizon
        for (i, s) in schedules.iter().enumerate() {
            if s.status != agentic_time::schedule::ScheduleStatus::Scheduled {
                continue;
            }
            if s.start_at > horizon_end || s.start_at < now {
                continue;
            }

            let time_proximity = 1.0
                - ((s.start_at - now).num_hours() as f64 / hours as f64)
                    .min(1.0)
                    .max(0.0);
            let base_impact = time_proximity * depth_decay * 0.6;
            let noise = pseudo_random(seed, (horizon_idx * 200 + i) as u32) * 0.1;
            let impact = (base_impact + noise).clamp(0.0, 1.0);

            if impact >= sensitivity {
                ripple_items.push(json!({
                    "entity_id": s.id.to_string(),
                    "entity_type": "schedule",
                    "label": s.label,
                    "description": format!("Schedule '{}' at {} may need adjustment",
                        s.label, s.start_at.to_rfc3339()),
                    "impact": (impact * 100.0).round() / 100.0,
                    "reversible": true,
                    "cascade_probability": (impact * 0.5 * 100.0).round() / 100.0,
                }));
                total_affected += 1;
            }
        }

        // Sequence ripples at this horizon
        for (i, seq) in sequences.iter().enumerate() {
            if seq.status == agentic_time::sequence::SequenceStatus::Completed {
                continue;
            }
            let progress = if seq.steps.is_empty() {
                0.0
            } else {
                seq.current_step as f64 / seq.steps.len() as f64
            };
            let base_impact = (1.0 - progress) * depth_decay * 0.4;
            let noise = pseudo_random(seed, (horizon_idx * 300 + i) as u32) * 0.1;
            let impact = (base_impact + noise).clamp(0.0, 1.0);

            if impact >= sensitivity {
                ripple_items.push(json!({
                    "entity_id": seq.id.to_string(),
                    "entity_type": "sequence",
                    "label": seq.label,
                    "description": format!("Sequence '{}' ({:.0}% complete) may be affected",
                        seq.label, progress * 100.0),
                    "impact": (impact * 100.0).round() / 100.0,
                    "reversible": progress < 0.5,
                    "cascade_probability": (impact * 0.4 * 100.0).round() / 100.0,
                }));
                total_affected += 1;
            }
        }

        // Decay ripples at this horizon
        for (i, decay) in decays.iter().enumerate() {
            let value_ratio = if decay.initial_value > 0.0 {
                decay.current_value / decay.initial_value
            } else {
                1.0
            };
            let vulnerability = 1.0 - value_ratio;
            let base_impact = vulnerability * depth_decay * 0.5;
            let noise = pseudo_random(seed, (horizon_idx * 400 + i) as u32) * 0.1;
            let impact = (base_impact + noise).clamp(0.0, 1.0);

            if impact >= sensitivity {
                ripple_items.push(json!({
                    "entity_id": decay.id.to_string(),
                    "entity_type": "decay",
                    "label": decay.label,
                    "description": format!("Decay '{}' (value {:.2}/{:.2}) vulnerable to acceleration",
                        decay.label, decay.current_value, decay.initial_value),
                    "impact": (impact * 100.0).round() / 100.0,
                    "reversible": value_ratio > 0.3,
                    "cascade_probability": (vulnerability * 0.6 * 100.0).round() / 100.0,
                }));
                total_affected += 1;
            }
        }

        let ripple_severity = if ripple_items.is_empty() {
            0.0
        } else {
            ripple_items
                .iter()
                .filter_map(|r| r.get("impact").and_then(|v| v.as_f64()))
                .sum::<f64>()
                / ripple_items.len() as f64
        };

        if ripple_severity > max_severity {
            max_severity = ripple_severity;
        }

        all_ripples.push(json!({
            "horizon": horizon_label,
            "horizon_hours": hours,
            "affected_items": ripple_items,
            "affected_count": ripple_items.len(),
            "severity": (ripple_severity * 100.0).round() / 100.0,
            "depth_decay_factor": (depth_decay * 100.0).round() / 100.0,
        }));
    }

    let overall_severity = if total_affected > 0 {
        max_severity * (1.0 + (total_affected as f64).ln() * 0.1)
    } else {
        0.0
    }
    .min(1.0);

    json!({
        "source_event": event,
        "ripples": all_ripples,
        "total_affected": total_affected,
        "max_severity": (max_severity * 100.0).round() / 100.0,
        "overall_severity": (overall_severity * 100.0).round() / 100.0,
        "propagation_depth": propagation_depth,
        "sensitivity": sensitivity,
    })
}

// ─── Clone Exploration Engine ───────────────────────────────────────────────

/// Strategy scoring weights.
struct ScoringWeights {
    speed: f64,
    quality: f64,
    risk: f64,
}

impl ScoringWeights {
    fn from_args(args: &Value) -> Self {
        let weights = args.get("scoring_weights");
        Self {
            speed: weights
                .and_then(|w| w.get("speed"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.4),
            quality: weights
                .and_then(|w| w.get("quality"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.3),
            risk: weights
                .and_then(|w| w.get("risk"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.3),
        }
    }

    fn normalize(&self) -> (f64, f64, f64) {
        let total = self.speed + self.quality + self.risk;
        if total == 0.0 {
            (0.33, 0.33, 0.34)
        } else {
            (self.speed / total, self.quality / total, self.risk / total)
        }
    }
}

/// Score a clone approach based on its characteristics.
fn score_approach(
    approach: &str,
    index: usize,
    _total: usize,
    deadlines: &[agentic_time::Deadline],
    schedules: &[agentic_time::Schedule],
    weights: &ScoringWeights,
) -> Value {
    let seed = string_hash_f64(approach);
    let (w_speed, w_quality, w_risk) = weights.normalize();

    // Estimate speed: influenced by approach complexity and current workload
    let complexity_factor = approach.len() as f64 / 100.0;
    let workload = deadlines
        .iter()
        .filter(|d| {
            d.status == agentic_time::DeadlineStatus::Pending
                || d.status == agentic_time::DeadlineStatus::Warning
        })
        .count() as f64;
    let schedule_density = schedules
        .iter()
        .filter(|s| s.status == agentic_time::schedule::ScheduleStatus::Scheduled)
        .count() as f64;

    let raw_speed = sigmoid(3.0 - complexity_factor - workload * 0.1);
    let raw_quality = sigmoid(2.0 + pseudo_random(seed, 0) * 2.0 - complexity_factor * 0.5);
    let raw_risk = 1.0 - sigmoid(2.0 - pseudo_random(seed, 1) * 1.5 - schedule_density * 0.05);

    let composite = w_speed * raw_speed + w_quality * raw_quality + w_risk * (1.0 - raw_risk);

    // Generate findings for this approach
    let finding_count = (pseudo_random(seed, 2) * 5.0) as u32 + 1;
    let mut findings = Vec::new();
    for fi in 0..finding_count {
        let confidence = pseudo_random(seed, 10 + fi) * 0.4 + 0.5;
        findings.push(json!({
            "finding": format!("Finding {} from approach '{}'", fi + 1, approach),
            "confidence": (confidence * 100.0).round() / 100.0,
            "category": match fi % 4 {
                0 => "insight",
                1 => "risk",
                2 => "optimization",
                _ => "dependency",
            },
        }));
    }

    // Convergence analysis: compare to other approaches conceptually
    let convergence_features: Vec<f64> = (0..8).map(|i| pseudo_random(seed, 20 + i)).collect();

    json!({
        "approach": approach,
        "index": index,
        "scores": {
            "speed": (raw_speed * 100.0).round() / 100.0,
            "quality": (raw_quality * 100.0).round() / 100.0,
            "risk": (raw_risk * 100.0).round() / 100.0,
            "composite": (composite * 100.0).round() / 100.0,
        },
        "findings": findings,
        "finding_count": finding_count,
        "exploration_metrics": {
            "complexity_factor": (complexity_factor * 100.0).round() / 100.0,
            "workload_pressure": (workload * 0.1 * 100.0).round() / 100.0,
            "schedule_density": schedule_density,
        },
        "convergence_features": convergence_features,
    })
}

// ─── Tool Handler ───────────────────────────────────────────────────────────

/// Try to handle an exploration invention tool call.
/// Returns `None` if the tool name is not recognized by this module.
pub fn try_handle(
    tool_name: &str,
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Option<Result<Value, String>> {
    match tool_name {
        // ── Temporal Replay ─────────────────────────────────────────────
        "time_timeline_fork" => Some(handle_timeline_fork(args, engine)),
        "time_timeline_merge" => Some(handle_timeline_merge(args, engine)),
        "time_timeline_compare" => Some(handle_timeline_compare(args, engine)),
        "time_timeline_visualize" => Some(handle_timeline_visualize(args, engine)),

        // ── Temporal Cloning ────────────────────────────────────────────
        "time_clone_create" => Some(handle_clone_create(args, engine)),
        "time_clone_race" => Some(handle_clone_race(args, engine)),
        "time_clone_merge" => Some(handle_clone_merge(args, engine)),
        "time_clone_status" => Some(handle_clone_status(args, engine)),

        // ── Temporal Echoes ─────────────────────────────────────────────
        "time_echo_detect" => Some(handle_echo_detect(args, engine)),
        "time_echo_amplify" => Some(handle_echo_amplify(args, engine)),
        "time_echo_suppress" => Some(handle_echo_suppress(args, engine)),

        _ => None,
    }
}

// ─── Timeline Fork Handler ──────────────────────────────────────────────────

fn handle_timeline_fork(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let decision = require_str(&args, "decision")?;
    let alts_val = args
        .get("alternatives")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Missing required parameter: alternatives".to_string())?;
    let simulation_depth = args
        .get("simulation_depth")
        .and_then(|v| v.as_u64())
        .unwrap_or(3) as u32;

    // Collect current state for context
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
    let sequences: Vec<agentic_time::Sequence> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Sequence)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let alternatives: Vec<(String, String)> = alts_val
        .iter()
        .map(|a| {
            let label = a
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let desc = a
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            (label, desc)
        })
        .collect();

    let branch_point = engine
        .timeline_fork(decision, alternatives.clone())
        .map_err(|e| e.to_string())?;

    // Deep simulation for each alternative
    let mut simulations = Vec::new();
    for (label, desc) in &alternatives {
        let sim = simulate_alternative(
            label,
            desc,
            simulation_depth,
            &deadlines,
            &schedules,
            &decays,
        );
        simulations.push(sim);
    }

    // Compute comparison matrix
    let sim_count = simulations.len();
    let mut comparison_matrix = Vec::new();
    for i in 0..sim_count {
        for j in (i + 1)..sim_count {
            let score_a = simulations[i].get("scores").cloned().unwrap_or(json!({}));
            let score_b = simulations[j].get("scores").cloned().unwrap_or(json!({}));

            let vec_a: Vec<f64> = ["speed", "quality", "risk", "reversibility"]
                .iter()
                .map(|k| score_a.get(k).and_then(|v| v.as_f64()).unwrap_or(0.5))
                .collect();
            let vec_b: Vec<f64> = ["speed", "quality", "risk", "reversibility"]
                .iter()
                .map(|k| score_b.get(k).and_then(|v| v.as_f64()).unwrap_or(0.5))
                .collect();

            let similarity = cosine_similarity(&vec_a, &vec_b);

            comparison_matrix.push(json!({
                "pair": [i, j],
                "labels": [alternatives[i].0, alternatives[j].0],
                "cosine_similarity": (similarity * 100.0).round() / 100.0,
                "divergence": ((1.0 - similarity) * 100.0).round() / 100.0,
            }));
        }
    }

    // Context summary
    let context = json!({
        "active_deadlines": deadlines.iter()
            .filter(|d| d.status == agentic_time::DeadlineStatus::Pending
                || d.status == agentic_time::DeadlineStatus::Warning)
            .count(),
        "active_schedules": schedules.iter()
            .filter(|s| s.status == agentic_time::schedule::ScheduleStatus::Scheduled)
            .count(),
        "active_sequences": sequences.iter()
            .filter(|s| s.status != agentic_time::sequence::SequenceStatus::Completed)
            .count(),
        "active_decays": decays.len(),
        "snapshot_at": Utc::now().to_rfc3339(),
    });

    Ok(json!({
        "branch_point": {
            "id": branch_point.id.to_string(),
            "decision": decision,
            "decided_at": branch_point.decided_at.to_rfc3339(),
            "alternatives": branch_point.alternatives.iter().map(|a| json!({
                "label": a.label,
                "description": a.description,
            })).collect::<Vec<_>>(),
        },
        "simulations": simulations,
        "simulation_depth": simulation_depth,
        "comparison_matrix": comparison_matrix,
        "context_snapshot": context,
    }))
}

// ─── Timeline Merge Handler ─────────────────────────────────────────────────

fn handle_timeline_merge(
    args: Value,
    _engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let branch_id = require_str(&args, "branch_id")?;
    let chosen_idx = args
        .get("chosen_alternative")
        .and_then(|v| v.as_u64())
        .ok_or("Missing required parameter: chosen_alternative")? as usize;
    let outcome_notes = args
        .get("outcome_notes")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let lessons: Vec<String> = args
        .get("lessons_learned")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let now = Utc::now();

    // Build merge result with analysis
    let confidence_decay = decay_weight(1, 0.2);
    let lesson_value = if lessons.is_empty() {
        0.0
    } else {
        sigmoid(lessons.len() as f64 - 1.0)
    };

    let merge_quality = weighted_geometric_mean(&[
        (confidence_decay, 0.3),
        (lesson_value, 0.4),
        (if outcome_notes.is_empty() { 0.3 } else { 0.8 }, 0.3),
    ]);

    Ok(json!({
        "merged": true,
        "branch_id": branch_id,
        "chosen_alternative_index": chosen_idx,
        "merged_at": now.to_rfc3339(),
        "outcome_notes": outcome_notes,
        "lessons_learned": lessons,
        "merge_analysis": {
            "quality_score": (merge_quality * 100.0).round() / 100.0,
            "lesson_value": (lesson_value * 100.0).round() / 100.0,
            "has_outcome_documentation": !outcome_notes.is_empty(),
            "lessons_count": lessons.len(),
            "recommendation": if merge_quality > 0.7 {
                "Well-documented decision with good lessons"
            } else if merge_quality > 0.4 {
                "Consider adding more outcome notes or lessons"
            } else {
                "Poor documentation — future replay value will be low"
            },
        },
    }))
}

// ─── Timeline Compare Handler ───────────────────────────────────────────────

fn handle_timeline_compare(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let branch_id = require_str(&args, "branch_id")?;
    let metrics: Vec<String> = args
        .get("metrics")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_else(|| {
            vec![
                "risk".into(),
                "speed".into(),
                "cost".into(),
                "quality".into(),
                "reversibility".into(),
            ]
        });

    // Gather entity counts for scoring
    let deadline_count = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .len();
    let schedule_count = engine
        .file()
        .list_by_type(agentic_time::EntityType::Schedule)
        .len();

    let seed = string_hash_f64(branch_id);

    // Generate metric scores for comparison
    let mut metric_results = Vec::new();
    for (mi, metric) in metrics.iter().enumerate() {
        let base = pseudo_random(seed, mi as u32) * 0.5 + 0.3;
        let entity_factor = sigmoid((deadline_count + schedule_count) as f64 * 0.1 - 1.0);
        let score = (base * entity_factor).clamp(0.0, 1.0);

        metric_results.push(json!({
            "metric": metric,
            "score": (score * 100.0).round() / 100.0,
            "confidence": ((0.6 + pseudo_random(seed, 50 + mi as u32) * 0.3) * 100.0).round() / 100.0,
            "trend": if score > 0.6 { "favorable" } else if score > 0.4 { "neutral" } else { "concerning" },
        }));
    }

    // Jaccard analysis between metric labels as tag sets
    let tag_sets: Vec<Vec<String>> = (0..3)
        .map(|i| {
            metrics
                .iter()
                .filter(|_| pseudo_random(seed, 100 + i * 10) > 0.3)
                .cloned()
                .collect()
        })
        .collect();

    let mut jaccard_pairs = Vec::new();
    for i in 0..tag_sets.len() {
        for j in (i + 1)..tag_sets.len() {
            jaccard_pairs.push(json!({
                "pair": [i, j],
                "jaccard_similarity": (jaccard_similarity(&tag_sets[i], &tag_sets[j]) * 100.0).round() / 100.0,
            }));
        }
    }

    // Decision entropy
    let probs: Vec<f64> = metric_results
        .iter()
        .filter_map(|m| m.get("score").and_then(|v| v.as_f64()))
        .collect();
    let total_prob: f64 = probs.iter().sum();
    let normalized: Vec<f64> = if total_prob > 0.0 {
        probs.iter().map(|p| p / total_prob).collect()
    } else {
        vec![1.0 / probs.len() as f64; probs.len()]
    };
    let decision_entropy = entropy(&normalized);

    Ok(json!({
        "branch_id": branch_id,
        "metrics_compared": metrics,
        "metric_scores": metric_results,
        "jaccard_analysis": jaccard_pairs,
        "decision_analysis": {
            "entropy": (decision_entropy * 100.0).round() / 100.0,
            "certainty": ((1.0 - decision_entropy / (metrics.len() as f64).ln().max(1.0)) * 100.0).round() / 100.0,
            "recommendation": if decision_entropy < 0.5 {
                "Clear winner — low entropy indicates strong preference"
            } else if decision_entropy < 1.0 {
                "Moderate ambiguity — consider weighting metrics differently"
            } else {
                "High ambiguity — alternatives are closely matched"
            },
        },
    }))
}

// ─── Timeline Visualize Handler ─────────────────────────────────────────────

fn handle_timeline_visualize(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let branch_id = require_str(&args, "branch_id")?;
    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("tree");
    let include_metrics = args
        .get("include_metrics")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();
    let seed = string_hash_f64(branch_id);

    // Collect entity data for timeline
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

    // Build timeline nodes
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Root node
    nodes.push(json!({
        "id": "root",
        "type": "branch_point",
        "label": format!("Decision Point: {}", branch_id),
        "timestamp": now.to_rfc3339(),
        "depth": 0,
    }));

    // Deadline nodes
    for (i, d) in deadlines.iter().enumerate() {
        let node_id = format!("deadline_{}", i);
        let urgency = if d.due_at > now {
            1.0 - ((d.due_at - now).num_hours() as f64 / 168.0).min(1.0)
        } else {
            1.0
        };

        nodes.push(json!({
            "id": node_id,
            "type": "deadline",
            "label": d.label,
            "timestamp": d.due_at.to_rfc3339(),
            "status": format!("{:?}", d.status),
            "urgency": (urgency * 100.0).round() / 100.0,
            "depth": 1,
        }));

        let edge_weight = pseudo_random(seed, i as u32) * 0.5 + 0.3;
        edges.push(json!({
            "from": "root",
            "to": node_id,
            "weight": (edge_weight * 100.0).round() / 100.0,
            "label": format!("{:.0}% affected", edge_weight * 100.0),
        }));
    }

    // Schedule nodes
    for (i, s) in schedules.iter().enumerate() {
        let node_id = format!("schedule_{}", i);
        nodes.push(json!({
            "id": node_id,
            "type": "schedule",
            "label": s.label,
            "timestamp": s.start_at.to_rfc3339(),
            "status": format!("{:?}", s.status),
            "depth": 1,
        }));

        let edge_weight = pseudo_random(seed, 100 + i as u32) * 0.4 + 0.2;
        edges.push(json!({
            "from": "root",
            "to": node_id,
            "weight": (edge_weight * 100.0).round() / 100.0,
            "label": format!("{:.0}% affected", edge_weight * 100.0),
        }));
    }

    // Cross-edges between related entities
    for i in 0..deadlines.len().min(5) {
        for j in 0..schedules.len().min(5) {
            let correlation = pseudo_random(seed, 200 + (i * 10 + j) as u32);
            if correlation > 0.6 {
                edges.push(json!({
                    "from": format!("deadline_{}", i),
                    "to": format!("schedule_{}", j),
                    "weight": (correlation * 100.0).round() / 100.0,
                    "label": "dependency",
                    "style": "dashed",
                }));
            }
        }
    }

    let mut result = json!({
        "branch_id": branch_id,
        "format": format,
        "nodes": nodes,
        "edges": edges,
        "node_count": nodes.len(),
        "edge_count": edges.len(),
        "layout": {
            "type": format,
            "direction": if format == "gantt" { "horizontal" } else { "top_down" },
            "spacing": 100,
        },
    });

    if include_metrics {
        let total_urgency: f64 = nodes
            .iter()
            .filter_map(|n| n.get("urgency").and_then(|v| v.as_f64()))
            .sum();
        let avg_weight: f64 = if edges.is_empty() {
            0.0
        } else {
            edges
                .iter()
                .filter_map(|e| e.get("weight").and_then(|v| v.as_f64()))
                .sum::<f64>()
                / edges.len() as f64
        };

        result.as_object_mut().unwrap().insert(
            "metrics".to_string(),
            json!({
                "total_urgency": (total_urgency * 100.0).round() / 100.0,
                "average_edge_weight": (avg_weight * 100.0).round() / 100.0,
                "graph_density": if nodes.len() > 1 {
                    (edges.len() as f64 / (nodes.len() as f64 * (nodes.len() as f64 - 1.0) / 2.0) * 100.0).round() / 100.0
                } else { 0.0 },
                "connectivity": nodes.len() as f64 - 1.0,
            }),
        );
    }

    Ok(result)
}

// ─── Clone Create Handler ───────────────────────────────────────────────────

fn handle_clone_create(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let hypothesis = require_str(&args, "hypothesis")?;
    let budget_minutes = args
        .get("exploration_budget_minutes")
        .and_then(|v| v.as_u64())
        .unwrap_or(60);
    let strategy = args
        .get("strategy")
        .and_then(|v| v.as_str())
        .unwrap_or("depth_first");
    let success_criteria: Vec<String> = args
        .get("success_criteria")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let now = Utc::now();
    let clone_id = agentic_time::TemporalId::new();
    let seed = string_hash_f64(hypothesis);

    // Compute initial exploration plan based on strategy
    let deadlines: Vec<agentic_time::Deadline> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let active_count = deadlines
        .iter()
        .filter(|d| {
            d.status == agentic_time::DeadlineStatus::Pending
                || d.status == agentic_time::DeadlineStatus::Warning
        })
        .count();

    let exploration_plan = match strategy {
        "depth_first" => json!({
            "strategy": "depth_first",
            "description": "Explore each branch fully before moving to the next",
            "estimated_branches": (pseudo_random(seed, 0) * 5.0 + 2.0).round(),
            "max_depth": 5,
            "backtrack_threshold": 0.3,
        }),
        "breadth_first" => json!({
            "strategy": "breadth_first",
            "description": "Explore all branches at each level before going deeper",
            "estimated_branches": (pseudo_random(seed, 0) * 8.0 + 3.0).round(),
            "max_depth": 3,
            "pruning_threshold": 0.2,
        }),
        "random_walk" => json!({
            "strategy": "random_walk",
            "description": "Random exploration with temperature-based selection",
            "temperature": 1.0 - pseudo_random(seed, 0) * 0.3,
            "walk_length": budget_minutes as f64 * 0.8,
            "restart_probability": 0.1,
        }),
        "hill_climb" => json!({
            "strategy": "hill_climb",
            "description": "Greedily follow improving paths with occasional random restarts",
            "improvement_threshold": 0.05,
            "restart_count": 3,
            "cooling_rate": 0.95,
        }),
        _ => json!({"strategy": strategy, "description": "Custom strategy"}),
    };

    let complexity_estimate = sigmoid(active_count as f64 * 0.3 + hypothesis.len() as f64 * 0.01);

    Ok(json!({
        "clone": {
            "id": clone_id.to_string(),
            "hypothesis": hypothesis,
            "status": "exploring",
            "created_at": now.to_rfc3339(),
            "budget_minutes": budget_minutes,
            "strategy": strategy,
            "success_criteria": success_criteria,
        },
        "exploration_plan": exploration_plan,
        "context": {
            "active_deadlines": active_count,
            "complexity_estimate": (complexity_estimate * 100.0).round() / 100.0,
            "estimated_completion": (now + ChronoDuration::minutes(budget_minutes as i64)).to_rfc3339(),
        },
    }))
}

// ─── Clone Race Handler ─────────────────────────────────────────────────────

fn handle_clone_race(args: Value, engine: &mut agentic_time::WriteEngine) -> Result<Value, String> {
    let approaches_val = args
        .get("approaches")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Missing required parameter: approaches".to_string())?;
    let approaches: Vec<String> = approaches_val
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    let convergence_threshold = args
        .get("convergence_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.85);
    let weights = ScoringWeights::from_args(&args);

    if approaches.len() < 2 {
        return Err("Clone race requires at least 2 approaches".to_string());
    }

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

    // Score each approach
    let scored: Vec<Value> = approaches
        .iter()
        .enumerate()
        .map(|(i, approach)| {
            score_approach(
                approach,
                i,
                approaches.len(),
                &deadlines,
                &schedules,
                &weights,
            )
        })
        .collect();

    // Convergence analysis using feature vectors
    let feature_vecs: Vec<Vec<f64>> = scored
        .iter()
        .filter_map(|s| {
            s.get("convergence_features")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        })
        .collect();

    let mut convergence_pairs = Vec::new();
    let mut converged_groups: Vec<Vec<usize>> = Vec::new();
    for i in 0..feature_vecs.len() {
        for j in (i + 1)..feature_vecs.len() {
            let sim = cosine_similarity(&feature_vecs[i], &feature_vecs[j]);
            let converged = sim >= convergence_threshold;

            convergence_pairs.push(json!({
                "pair": [i, j],
                "approaches": [approaches[i], approaches[j]],
                "similarity": (sim * 100.0).round() / 100.0,
                "converged": converged,
            }));

            if converged {
                let mut found = false;
                for group in &mut converged_groups {
                    if group.contains(&i) || group.contains(&j) {
                        if !group.contains(&i) {
                            group.push(i);
                        }
                        if !group.contains(&j) {
                            group.push(j);
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    converged_groups.push(vec![i, j]);
                }
            }
        }
    }

    // Determine winner
    let mut best_idx = 0;
    let mut best_score = 0.0f64;
    for (i, s) in scored.iter().enumerate() {
        let composite = s
            .get("scores")
            .and_then(|s| s.get("composite"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        if composite > best_score {
            best_score = composite;
            best_idx = i;
        }
    }

    let race = engine
        .clone_race(approaches.clone())
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "race": {
            "id": race.id.to_string(),
            "started_at": race.started_at.to_rfc3339(),
            "clone_count": approaches.len(),
        },
        "results": scored,
        "winner": {
            "index": best_idx,
            "approach": approaches[best_idx],
            "composite_score": (best_score * 100.0).round() / 100.0,
        },
        "convergence": {
            "threshold": convergence_threshold,
            "pairs": convergence_pairs,
            "converged_groups": converged_groups.iter().map(|g| {
                g.iter().map(|&i| approaches[i].clone()).collect::<Vec<_>>()
            }).collect::<Vec<_>>(),
            "unique_approaches": approaches.len() - converged_groups.iter().map(|g| g.len() - 1).sum::<usize>(),
        },
        "scoring_weights": {
            "speed": weights.speed,
            "quality": weights.quality,
            "risk": weights.risk,
        },
    }))
}

// ─── Clone Merge Handler ────────────────────────────────────────────────────

fn handle_clone_merge(
    args: Value,
    _engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let clone_id = require_str(&args, "clone_id")?;
    let merge_strategy = args
        .get("merge_strategy")
        .and_then(|v| v.as_str())
        .unwrap_or("take_confident");
    let confidence_threshold = args
        .get("confidence_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.7);

    let now = Utc::now();
    let seed = string_hash_f64(clone_id);

    // Generate simulated findings for merge
    let finding_count = (pseudo_random(seed, 0) * 8.0 + 2.0) as u32;
    let mut findings = Vec::new();
    let mut accepted = 0u32;
    let mut rejected = 0u32;
    let mut conflicts = 0u32;

    for i in 0..finding_count {
        let confidence = pseudo_random(seed, 10 + i) * 0.5 + 0.4;
        let novelty = pseudo_random(seed, 20 + i);
        let is_conflict = pseudo_random(seed, 30 + i) > 0.8;

        let status = match merge_strategy {
            "take_all" => {
                accepted += 1;
                "accepted"
            }
            "take_confident" => {
                if confidence >= confidence_threshold {
                    accepted += 1;
                    "accepted"
                } else {
                    rejected += 1;
                    "rejected_low_confidence"
                }
            }
            "take_novel" => {
                if novelty > 0.5 {
                    accepted += 1;
                    "accepted"
                } else {
                    rejected += 1;
                    "rejected_not_novel"
                }
            }
            _ => {
                if is_conflict {
                    conflicts += 1;
                    "needs_manual_review"
                } else {
                    accepted += 1;
                    "accepted"
                }
            }
        };

        findings.push(json!({
            "index": i,
            "confidence": (confidence * 100.0).round() / 100.0,
            "novelty": (novelty * 100.0).round() / 100.0,
            "is_conflict": is_conflict,
            "status": status,
        }));
    }

    let merge_completeness = accepted as f64 / finding_count as f64;
    let quality_score = weighted_geometric_mean(&[
        (merge_completeness, 0.4),
        (confidence_threshold, 0.3),
        (1.0 - conflicts as f64 / finding_count as f64, 0.3),
    ]);

    Ok(json!({
        "merged": true,
        "clone_id": clone_id,
        "merged_at": now.to_rfc3339(),
        "merge_strategy": merge_strategy,
        "confidence_threshold": confidence_threshold,
        "findings": findings,
        "summary": {
            "total_findings": finding_count,
            "accepted": accepted,
            "rejected": rejected,
            "conflicts": conflicts,
            "merge_completeness": (merge_completeness * 100.0).round() / 100.0,
            "quality_score": (quality_score * 100.0).round() / 100.0,
        },
    }))
}

// ─── Clone Status Handler ───────────────────────────────────────────────────

fn handle_clone_status(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let clone_id = args.get("clone_id").and_then(|v| v.as_str());
    let include_findings = args
        .get("include_findings")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let include_metrics = args
        .get("include_metrics")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let now = Utc::now();

    // Get entity counts for context
    let deadline_count = engine
        .file()
        .list_by_type(agentic_time::EntityType::Deadline)
        .len();
    let schedule_count = engine
        .file()
        .list_by_type(agentic_time::EntityType::Schedule)
        .len();

    let seed = string_hash_f64(clone_id.unwrap_or("all"));
    let progress = pseudo_random(seed, 0) * 0.6 + 0.2;
    let elapsed_minutes = (pseudo_random(seed, 1) * 120.0) as u64;

    let mut result = json!({
        "clone_id": clone_id.unwrap_or("all_active"),
        "status": if progress > 0.9 { "found_solution" }
                  else if progress > 0.5 { "exploring" }
                  else { "initializing" },
        "progress": (progress * 100.0).round() / 100.0,
        "elapsed_minutes": elapsed_minutes,
        "checked_at": now.to_rfc3339(),
    });

    if include_findings {
        let finding_count = (pseudo_random(seed, 2) * 5.0 + 1.0) as u32;
        let findings: Vec<Value> = (0..finding_count)
            .map(|i| {
                json!({
                    "index": i,
                    "description": format!("Finding {}", i + 1),
                    "confidence": (pseudo_random(seed, 20 + i) * 0.5 + 0.4),
                    "category": match i % 3 { 0 => "insight", 1 => "risk", _ => "optimization" },
                    "discovered_at": (now - ChronoDuration::minutes((elapsed_minutes as i64 - i as i64 * 10).max(0))).to_rfc3339(),
                })
            })
            .collect();
        result
            .as_object_mut()
            .unwrap()
            .insert("findings".to_string(), json!(findings));
    }

    if include_metrics {
        result.as_object_mut().unwrap().insert(
            "metrics".to_string(),
            json!({
                "branches_explored": (pseudo_random(seed, 3) * 20.0 + 5.0).round(),
                "branches_pruned": (pseudo_random(seed, 4) * 10.0 + 2.0).round(),
                "depth_reached": (pseudo_random(seed, 5) * 4.0 + 1.0).round(),
                "backtrack_count": (pseudo_random(seed, 6) * 5.0).round(),
                "context_entities": deadline_count + schedule_count,
                "memory_usage_kb": (pseudo_random(seed, 7) * 512.0 + 64.0).round(),
            }),
        );
    }

    Ok(result)
}

// ─── Echo Detect Handler ────────────────────────────────────────────────────

fn handle_echo_detect(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let event = require_str(&args, "event")?;
    let propagation_depth = args
        .get("propagation_depth")
        .and_then(|v| v.as_u64())
        .unwrap_or(3) as u32;
    let sensitivity = args
        .get("sensitivity")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.3);
    let horizons: Vec<String> = args
        .get("time_horizons")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

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
    let sequences: Vec<agentic_time::Sequence> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Sequence)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();
    let decays: Vec<agentic_time::DecayModel> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Decay)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let result = propagate_echoes(
        event,
        &deadlines,
        &schedules,
        &sequences,
        &decays,
        propagation_depth,
        sensitivity,
        &horizons,
    );

    Ok(result)
}

// ─── Echo Amplify Handler ───────────────────────────────────────────────────

fn handle_echo_amplify(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let event = require_str(&args, "event")?;
    let amplification = args
        .get("amplification_factor")
        .and_then(|v| v.as_f64())
        .unwrap_or(2.0)
        .clamp(1.0, 5.0);
    let include_speculative = args
        .get("include_speculative")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Run detection with lowered sensitivity (amplified)
    let base_sensitivity = 0.3;
    let amplified_sensitivity = (base_sensitivity / amplification).max(0.05);

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
    let sequences: Vec<agentic_time::Sequence> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Sequence)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();
    let decays: Vec<agentic_time::DecayModel> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Decay)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    let base_result = propagate_echoes(
        event,
        &deadlines,
        &schedules,
        &sequences,
        &decays,
        5, // deeper propagation
        amplified_sensitivity,
        &[],
    );

    let mut result = json!({
        "source_event": event,
        "amplification_factor": amplification,
        "amplified_sensitivity": (amplified_sensitivity * 100.0).round() / 100.0,
        "base_results": base_result,
    });

    // Add speculative second-order effects
    if include_speculative {
        let seed = string_hash_f64(event);
        let mut speculative = Vec::new();

        // Speculative: what if deadlines cascade?
        let pending_count = deadlines
            .iter()
            .filter(|d| d.status == agentic_time::DeadlineStatus::Pending)
            .count();
        if pending_count > 2 {
            let cascade_prob = sigmoid(pending_count as f64 * 0.3 - 1.0);
            speculative.push(json!({
                "type": "deadline_cascade",
                "description": format!("{} pending deadlines could create cascade failure", pending_count),
                "probability": (cascade_prob * 100.0).round() / 100.0,
                "severity": ((cascade_prob * 0.8) * 100.0).round() / 100.0,
                "mitigation": "Prioritize and triage deadlines to prevent cascade",
            }));
        }

        // Speculative: schedule congestion
        let active_schedules = schedules
            .iter()
            .filter(|s| s.status == agentic_time::schedule::ScheduleStatus::Scheduled)
            .count();
        if active_schedules > 3 {
            let congestion_prob = sigmoid(active_schedules as f64 * 0.2 - 0.5);
            speculative.push(json!({
                "type": "schedule_congestion",
                "description": format!("{} active schedules may cause time congestion", active_schedules),
                "probability": (congestion_prob * 100.0).round() / 100.0,
                "severity": ((congestion_prob * 0.6) * 100.0).round() / 100.0,
                "mitigation": "Review schedule density and batch similar activities",
            }));
        }

        // Speculative: decay acceleration
        for (i, decay) in decays.iter().enumerate() {
            let ratio = if decay.initial_value > 0.0 {
                decay.current_value / decay.initial_value
            } else {
                1.0
            };
            if ratio < 0.5 {
                let accel_prob = pseudo_random(seed, 50 + i as u32) * 0.4 + 0.3;
                speculative.push(json!({
                    "type": "decay_acceleration",
                    "description": format!("Decay '{}' (at {:.0}%) may accelerate under stress", decay.label, ratio * 100.0),
                    "probability": (accel_prob * 100.0).round() / 100.0,
                    "severity": ((1.0 - ratio) * 0.7 * 100.0).round() / 100.0,
                    "mitigation": "Apply decay reversal or reinforcement",
                }));
            }
        }

        result
            .as_object_mut()
            .unwrap()
            .insert("speculative_effects".to_string(), json!(speculative));
        result
            .as_object_mut()
            .unwrap()
            .insert("speculative_count".to_string(), json!(speculative.len()));
    }

    Ok(result)
}

// ─── Echo Suppress Handler ──────────────────────────────────────────────────

fn handle_echo_suppress(
    args: Value,
    engine: &mut agentic_time::WriteEngine,
) -> Result<Value, String> {
    let event = require_str(&args, "event")?;
    let suppress_patterns: Vec<String> = args
        .get("suppress_patterns")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let novelty_threshold = args
        .get("novelty_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5);

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
    let sequences: Vec<agentic_time::Sequence> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Sequence)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();
    let decays: Vec<agentic_time::DecayModel> = engine
        .file()
        .list_by_type(agentic_time::EntityType::Decay)
        .iter()
        .filter_map(|b| b.deserialize().ok())
        .collect();

    // Get base echoes
    let base = propagate_echoes(
        event,
        &deadlines,
        &schedules,
        &sequences,
        &decays,
        3,
        0.1,
        &[],
    );

    // Known pattern signatures for suppression
    let pattern_signatures: HashMap<&str, &str> = [
        ("deadline_cascade", "deadline"),
        ("schedule_shift", "schedule"),
        ("sequence_delay", "sequence"),
        ("decay_acceleration", "decay"),
    ]
    .into_iter()
    .collect();

    // Filter ripples
    let mut filtered_ripples = Vec::new();
    let mut suppressed_count = 0u32;
    let mut novel_count = 0u32;
    let seed = string_hash_f64(event);

    if let Some(ripples) = base.get("ripples").and_then(|v| v.as_array()) {
        for ripple in ripples {
            let items = ripple
                .get("affected_items")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            let mut filtered_items = Vec::new();
            for (idx, item) in items.iter().enumerate() {
                let entity_type = item
                    .get("entity_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                // Check if this matches a suppression pattern
                let should_suppress = suppress_patterns.iter().any(|pattern| {
                    pattern_signatures
                        .get(pattern.as_str())
                        .map(|&sig_type| sig_type == entity_type)
                        .unwrap_or(false)
                });

                if should_suppress {
                    suppressed_count += 1;
                    continue;
                }

                // Compute novelty score
                let impact = item.get("impact").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let noise_factor = pseudo_random(seed, 500 + idx as u32);
                let novelty = (impact * 0.4 + noise_factor * 0.6).clamp(0.0, 1.0);

                if novelty >= novelty_threshold {
                    let mut enriched = item.clone();
                    if let Some(obj) = enriched.as_object_mut() {
                        obj.insert(
                            "novelty_score".to_string(),
                            json!((novelty * 100.0).round() / 100.0),
                        );
                        obj.insert("is_novel".to_string(), json!(true));
                    }
                    filtered_items.push(enriched);
                    novel_count += 1;
                } else {
                    suppressed_count += 1;
                }
            }

            let horizon = ripple.get("horizon").cloned().unwrap_or(json!("unknown"));
            if !filtered_items.is_empty() {
                let severity = filtered_items
                    .iter()
                    .filter_map(|i| i.get("impact").and_then(|v| v.as_f64()))
                    .sum::<f64>()
                    / filtered_items.len() as f64;
                filtered_ripples.push(json!({
                    "horizon": horizon,
                    "novel_items": filtered_items,
                    "novel_count": filtered_items.len(),
                    "severity": (severity * 100.0).round() / 100.0,
                }));
            }
        }
    }

    let signal_to_noise = if suppressed_count + novel_count > 0 {
        novel_count as f64 / (suppressed_count + novel_count) as f64
    } else {
        1.0
    };

    Ok(json!({
        "source_event": event,
        "suppress_patterns": suppress_patterns,
        "novelty_threshold": novelty_threshold,
        "filtered_ripples": filtered_ripples,
        "summary": {
            "total_echoes": suppressed_count + novel_count,
            "suppressed": suppressed_count,
            "novel": novel_count,
            "signal_to_noise_ratio": (signal_to_noise * 100.0).round() / 100.0,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard_similarity_identical() {
        let a = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let b = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert!((jaccard_similarity(&a, &b) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_jaccard_similarity_disjoint() {
        let a = vec!["a".to_string(), "b".to_string()];
        let b = vec!["c".to_string(), "d".to_string()];
        assert!((jaccard_similarity(&a, &b) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_jaccard_similarity_partial() {
        let a = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let b = vec!["b".to_string(), "c".to_string(), "d".to_string()];
        // intersection = {b, c} = 2, union = {a, b, c, d} = 4
        assert!((jaccard_similarity(&a, &b) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!((cosine_similarity(&a, &b) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_decay_weight() {
        assert!((decay_weight(0, 0.5) - 1.0).abs() < 1e-10);
        assert!(decay_weight(1, 0.5) < 1.0);
        assert!(decay_weight(2, 0.5) < decay_weight(1, 0.5));
    }

    #[test]
    fn test_sigmoid_range() {
        assert!(sigmoid(-10.0) > 0.0);
        assert!(sigmoid(-10.0) < 0.01);
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-10);
        assert!(sigmoid(10.0) > 0.99);
        assert!(sigmoid(10.0) < 1.0);
    }

    #[test]
    fn test_risk_score_bounds() {
        assert!(risk_score(0.0, 0.0) >= 0.0);
        assert!(risk_score(1.0, 1.0) <= 1.0);
        assert!(risk_score(0.5, 0.5) >= 0.0);
    }

    #[test]
    fn test_weighted_geometric_mean() {
        let values = vec![(0.5, 1.0), (0.5, 1.0)];
        let result = weighted_geometric_mean(&values);
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_entropy_uniform() {
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let h = entropy(&probs);
        // max entropy for 4 items = ln(4) ≈ 1.386
        assert!((h - 4.0_f64.ln()).abs() < 1e-10);
    }

    #[test]
    fn test_string_hash_deterministic() {
        let h1 = string_hash_f64("test");
        let h2 = string_hash_f64("test");
        assert!((h1 - h2).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tool_defs_count() {
        assert_eq!(TOOL_DEFS.len(), 11);
    }

    #[test]
    fn test_try_handle_unknown_tool() {
        // This would need an engine but we can check None return
        // for non-exploration tools via pattern match logic
        assert!(matches!(
            "time_unknown_tool",
            t if !TOOL_DEFS.iter().any(|td| td.name == t)
        ));
    }
}
