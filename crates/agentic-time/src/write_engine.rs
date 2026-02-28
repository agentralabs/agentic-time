//! Write engine — create and update temporal entities.

use chrono::{DateTime, Duration as ChronoDuration, Utc};

use crate::deadline::{Deadline, DeadlineStatus};
use crate::decay::DecayModel;
use crate::duration::DurationEstimate;
use crate::error::{TimeError, TimeResult};
use crate::file_format::{EntityType, TimeFile};
use crate::inventions::*;
use crate::schedule::{Schedule, ScheduleStatus};
use crate::sequence::Sequence;
use crate::TemporalId;

/// Write engine for temporal operations.
pub struct WriteEngine {
    file: TimeFile,
}

impl WriteEngine {
    /// Create a new write engine wrapping a TimeFile.
    pub fn new(file: TimeFile) -> Self {
        Self { file }
    }

    /// Consume the engine, returning the underlying TimeFile.
    pub fn into_file(self) -> TimeFile {
        self.file
    }

    /// Get a reference to the underlying TimeFile.
    pub fn file(&self) -> &TimeFile {
        &self.file
    }

    /// Get a mutable reference to the underlying TimeFile.
    pub fn file_mut(&mut self) -> &mut TimeFile {
        &mut self.file
    }

    // --- Durations ---

    /// Add a duration estimate.
    pub fn add_duration(&mut self, duration: DurationEstimate) -> TimeResult<TemporalId> {
        let id = duration.id;
        self.file.add(EntityType::Duration, id, &duration)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing duration estimate.
    pub fn update_duration(&mut self, duration: DurationEstimate) -> TimeResult<()> {
        self.file
            .add(EntityType::Duration, duration.id, &duration)?;
        self.file.save()?;
        Ok(())
    }

    // --- Deadlines ---

    /// Add a deadline.
    pub fn add_deadline(&mut self, deadline: Deadline) -> TimeResult<TemporalId> {
        let id = deadline.id;
        self.file.add(EntityType::Deadline, id, &deadline)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing deadline.
    pub fn update_deadline(&mut self, deadline: Deadline) -> TimeResult<()> {
        self.file
            .add(EntityType::Deadline, deadline.id, &deadline)?;
        self.file.save()?;
        Ok(())
    }

    /// Complete a deadline.
    pub fn complete_deadline(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut deadline: Deadline = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        deadline.complete();
        self.update_deadline(deadline)
    }

    /// Cancel a deadline.
    pub fn cancel_deadline(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut deadline: Deadline = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        deadline.status = DeadlineStatus::Cancelled;
        self.update_deadline(deadline)
    }

    // --- Schedules ---

    /// Add a schedule. Returns error if it conflicts with existing non-flexible schedules.
    pub fn add_schedule(&mut self, schedule: Schedule) -> TimeResult<TemporalId> {
        let conflicts = self.check_schedule_conflicts(&schedule)?;
        if !conflicts.is_empty() && !schedule.flexible {
            return Err(TimeError::ScheduleConflict(
                schedule.label.clone(),
                conflicts[0].label.clone(),
            ));
        }

        let id = schedule.id;
        self.file.add(EntityType::Schedule, id, &schedule)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing schedule.
    pub fn update_schedule(&mut self, schedule: Schedule) -> TimeResult<()> {
        self.file
            .add(EntityType::Schedule, schedule.id, &schedule)?;
        self.file.save()?;
        Ok(())
    }

    /// Reschedule to a new start time.
    pub fn reschedule(&mut self, id: &TemporalId, new_start: DateTime<Utc>) -> TimeResult<()> {
        let mut schedule: Schedule = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        schedule.start_at = new_start;
        schedule.status = ScheduleStatus::Rescheduled;
        self.update_schedule(schedule)
    }

    /// Check for conflicting schedules.
    fn check_schedule_conflicts(&self, schedule: &Schedule) -> TimeResult<Vec<Schedule>> {
        let mut conflicts = Vec::new();

        for block in self.file.list_by_type(EntityType::Schedule) {
            let existing: Schedule = block.deserialize()?;
            if existing.id != schedule.id
                && existing.status == ScheduleStatus::Scheduled
                && schedule.conflicts_with(&existing)
            {
                conflicts.push(existing);
            }
        }

        Ok(conflicts)
    }

    // --- Sequences ---

    /// Add a sequence.
    pub fn add_sequence(&mut self, sequence: Sequence) -> TimeResult<TemporalId> {
        let id = sequence.id;
        self.file.add(EntityType::Sequence, id, &sequence)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing sequence.
    pub fn update_sequence(&mut self, sequence: Sequence) -> TimeResult<()> {
        self.file
            .add(EntityType::Sequence, sequence.id, &sequence)?;
        self.file.save()?;
        Ok(())
    }

    /// Advance a sequence to the next step.
    pub fn advance_sequence(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut sequence: Sequence = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        sequence.complete_current_step();
        self.update_sequence(sequence)
    }

    // --- Decay ---

    /// Add a decay model.
    pub fn add_decay(&mut self, decay: DecayModel) -> TimeResult<TemporalId> {
        let id = decay.id;
        self.file.add(EntityType::Decay, id, &decay)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing decay model.
    pub fn update_decay(&mut self, decay: DecayModel) -> TimeResult<()> {
        self.file.add(EntityType::Decay, decay.id, &decay)?;
        self.file.save()?;
        Ok(())
    }

    /// Refresh a decay model (recalculate current value).
    pub fn refresh_decay(&mut self, id: &TemporalId) -> TimeResult<f64> {
        let mut decay: DecayModel = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        decay.update();
        let value = decay.current_value;
        self.update_decay(decay)?;
        Ok(value)
    }

    // --- Batch operations ---

    /// Update all statuses (deadlines + decays) based on current time.
    pub fn update_all_statuses(&mut self) -> TimeResult<UpdateReport> {
        let mut report = UpdateReport::default();

        // Collect deadline blocks to avoid borrow issues
        let deadline_blocks: Vec<_> = self
            .file
            .list_by_type(EntityType::Deadline)
            .into_iter()
            .cloned()
            .collect();

        for block in deadline_blocks {
            let mut deadline: Deadline = block.deserialize()?;
            let old_status = deadline.status;
            deadline.update_status();
            if deadline.status != old_status {
                self.file
                    .add(EntityType::Deadline, deadline.id, &deadline)?;
                report.deadlines_updated += 1;
                if deadline.status == DeadlineStatus::Overdue {
                    report.new_overdue.push(deadline.id);
                }
            }
        }

        // Collect decay blocks
        let decay_blocks: Vec<_> = self
            .file
            .list_by_type(EntityType::Decay)
            .into_iter()
            .cloned()
            .collect();

        for block in decay_blocks {
            let mut decay: DecayModel = block.deserialize()?;
            decay.update();
            self.file.add(EntityType::Decay, decay.id, &decay)?;
            report.decays_updated += 1;
        }

        self.file.save()?;
        Ok(report)
    }

    // =====================================================================
    // INVENTION METHODS (16)
    // =====================================================================

    // ── 1. Temporal Replay (Timeline Fork) ──────────────────────────────

    /// Fork the timeline for what-if analysis at a decision point.
    pub fn timeline_fork(
        &self,
        decision: &str,
        alternatives: Vec<(String, String)>,
    ) -> TimeResult<BranchPoint> {
        let now = Utc::now();

        let deadline_count = self.file.list_by_type(EntityType::Deadline).len();
        let schedule_count = self.file.list_by_type(EntityType::Schedule).len();
        let sequence_count = self.file.list_by_type(EntityType::Sequence).len();
        let decay_count = self.file.list_by_type(EntityType::Decay).len();

        let context = serde_json::json!({
            "deadlines": deadline_count,
            "schedules": schedule_count,
            "sequences": sequence_count,
            "decays": decay_count,
            "snapshot_at": now.to_rfc3339(),
        });

        let alts = alternatives
            .into_iter()
            .map(|(label, desc)| Alternative {
                label,
                description: desc,
                was_chosen: false,
            })
            .collect();

        Ok(BranchPoint {
            id: TemporalId::new(),
            decided_at: now,
            decision: decision.to_string(),
            alternatives: alts,
            context_snapshot: context,
            actual_outcome: None,
        })
    }

    // ── 2. Temporal Cloning (Clone Race) ────────────────────────────────

    /// Create a clone race: parallel explorations of alternative approaches.
    pub fn clone_race(&self, approaches: Vec<String>) -> TimeResult<CloneRace> {
        let clone_ids: Vec<TemporalId> = approaches.iter().map(|_| TemporalId::new()).collect();

        Ok(CloneRace {
            id: TemporalId::new(),
            clones: clone_ids,
            winner: None,
            started_at: Utc::now(),
        })
    }

    // ── 3. Temporal Echoes ──────────────────────────────────────────────

    /// Detect temporal echoes — ripple effects from time-changing events.
    pub fn echo_detect(&self, event_description: &str) -> TimeResult<TemporalEcho> {
        let now = Utc::now();
        let mut ripples = Vec::new();

        // Analyze deadlines for potential ripple effects
        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.status == DeadlineStatus::Pending || d.status == DeadlineStatus::Warning {
                let severity = if d.status == DeadlineStatus::Warning {
                    0.8
                } else {
                    0.4
                };
                ripples.push(Ripple {
                    time_horizon: TimeHorizon::ShortTerm,
                    affected_items: vec![AffectedItem {
                        entity_id: d.id,
                        description: format!(
                            "Deadline '{}' due {} may be affected",
                            d.label,
                            d.due_at.to_rfc3339()
                        ),
                        impact: severity,
                    }],
                    severity,
                    reversible: true,
                });
            }
        }

        // Analyze schedules
        for block in self.file.list_by_type(EntityType::Schedule) {
            let s: Schedule = block.deserialize()?;
            if s.status == ScheduleStatus::Scheduled && s.start_at > now {
                ripples.push(Ripple {
                    time_horizon: TimeHorizon::MediumTerm,
                    affected_items: vec![AffectedItem {
                        entity_id: s.id,
                        description: format!(
                            "Scheduled event '{}' at {} may need adjustment",
                            s.label,
                            s.start_at.to_rfc3339()
                        ),
                        impact: 0.3,
                    }],
                    severity: 0.3,
                    reversible: true,
                });
            }
        }

        let impact = if ripples.is_empty() {
            "No ripple effects detected".to_string()
        } else {
            format!("{} entities potentially affected", ripples.len())
        };

        Ok(TemporalEcho {
            id: TemporalId::new(),
            source_change: event_description.to_string(),
            ripples,
            impact_summary: impact,
        })
    }

    // ── 4. Deadline Prophecy ────────────────────────────────────────────

    /// Predict deadline outcomes based on current trajectory.
    pub fn deadline_prophecy(&self) -> TimeResult<Vec<DeadlineProphecy>> {
        let now = Utc::now();
        let mut prophecies = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.status != DeadlineStatus::Pending && d.status != DeadlineStatus::Warning {
                continue;
            }

            let remaining_secs = (d.due_at - now).num_seconds();
            let total_secs = (d.due_at - d.created_at).num_seconds();
            let elapsed_ratio = if total_secs > 0 {
                1.0 - (remaining_secs as f64 / total_secs as f64)
            } else {
                1.0
            };

            let success_probability = if remaining_secs <= 0 {
                0.05
            } else if elapsed_ratio > 0.8 {
                0.3
            } else if elapsed_ratio > 0.5 {
                0.6
            } else {
                0.85
            };

            let factors = vec![ProphecyFactor {
                name: "Time elapsed ratio".to_string(),
                impact: elapsed_ratio - 0.5, // negative if ahead, positive if behind
                confidence: 0.8,
                evidence: format!(
                    "{:.0}% of allocated time used, {:.0} hours remaining",
                    elapsed_ratio * 100.0,
                    remaining_secs as f64 / 3600.0
                ),
            }];

            let interventions = if elapsed_ratio > 0.5 {
                vec![Intervention {
                    action: "Reduce scope or get help".to_string(),
                    success_probability_after: (success_probability + 0.2f64).min(1.0),
                    cost: "Moderate effort".to_string(),
                    recommended: elapsed_ratio > 0.7,
                }]
            } else {
                Vec::new()
            };

            prophecies.push(DeadlineProphecy {
                id: TemporalId::new(),
                deadline_id: d.id,
                success_probability,
                factors,
                interventions,
                prophesied_at: now,
                historical_accuracy: 0.7,
            });
        }

        Ok(prophecies)
    }

    // ── 5. Causal Archaeology ───────────────────────────────────────────

    /// Excavate cause-effect chains from temporal history.
    pub fn causal_archaeology(
        &self,
        problem: &str,
        window_secs: i64,
    ) -> TimeResult<CausalExcavation> {
        let now = Utc::now();
        let cutoff = now - ChronoDuration::seconds(window_secs);
        let mut events = Vec::new();

        // Collect deadlines as causal events
        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.created_at >= cutoff {
                let event_type = match d.status {
                    DeadlineStatus::Overdue => CausalEventType::Inaction,
                    _ => CausalEventType::Decision,
                };
                events.push(CausalEvent {
                    timestamp: d.created_at,
                    description: format!("Deadline '{}': {:?}", d.label, d.status),
                    event_type,
                    led_to: Vec::new(),
                });
            }
        }

        // Collect schedules as causal events
        for block in self.file.list_by_type(EntityType::Schedule) {
            let s: Schedule = block.deserialize()?;
            if s.start_at >= cutoff {
                events.push(CausalEvent {
                    timestamp: s.start_at,
                    description: format!("Schedule '{}': {:?}", s.label, s.status),
                    event_type: CausalEventType::Decision,
                    led_to: Vec::new(),
                });
            }
        }

        events.sort_by_key(|e| e.timestamp);

        let root_cause = events.first().cloned().unwrap_or_else(|| CausalEvent {
            timestamp: now,
            description: "No events found in window".to_string(),
            event_type: CausalEventType::Inaction,
            led_to: Vec::new(),
        });

        let event_count = events.len();

        Ok(CausalExcavation {
            id: TemporalId::new(),
            problem: problem.to_string(),
            root_cause,
            causal_chain: events,
            inflection_points: Vec::new(),
            patterns: if event_count > 3 {
                vec![CausalPattern {
                    description: "Sequential events detected".to_string(),
                    occurrences: event_count as u32,
                    severity: 0.6,
                }]
            } else {
                Vec::new()
            },
        })
    }

    // ── 6. Future Memory ────────────────────────────────────────────────

    /// Precompute future context for an upcoming point in time.
    pub fn future_memory(&self, target_time: DateTime<Utc>) -> TimeResult<FutureMemory> {
        let now = Utc::now();
        let mut context_items = Vec::new();
        let mut suggested_focus = Vec::new();

        // Deadlines active at target time
        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.due_at > now && d.due_at <= target_time {
                context_items.push(ContextItem {
                    label: d.label.clone(),
                    content_type: ContextType::RelatedTask,
                    content: format!("Deadline due {}", d.due_at.to_rfc3339()),
                    relevance: 0.9,
                });
                suggested_focus.push(format!("Complete '{}'", d.label));
            }
        }

        // Schedules in the window
        for block in self.file.list_by_type(EntityType::Schedule) {
            let s: Schedule = block.deserialize()?;
            if s.start_at >= now && s.start_at <= target_time {
                context_items.push(ContextItem {
                    label: s.label.clone(),
                    content_type: ContextType::RelatedTask,
                    content: format!("Scheduled at {}", s.start_at.to_rfc3339()),
                    relevance: 0.7,
                });
            }
        }

        // Decays to monitor
        for block in self.file.list_by_type(EntityType::Decay) {
            let d: DecayModel = block.deserialize()?;
            context_items.push(ContextItem {
                label: d.label.clone(),
                content_type: ContextType::Metric,
                content: format!("Current value: {:.2}", d.current_value),
                relevance: 0.5,
            });
        }

        context_items.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(FutureMemory {
            id: TemporalId::new(),
            event_id: TemporalId::new(),
            event_time: target_time,
            context_items,
            precomputed: Vec::new(),
            suggested_focus,
            created_at: now,
        })
    }

    // ── 7. Temporal Debt ────────────────────────────────────────────────

    /// Analyze temporal debt across all overdue and deferred items.
    pub fn debt_analyze(&self) -> TimeResult<DebtReport> {
        let mut items = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.status == DeadlineStatus::Overdue {
                let overdue_secs = d.overdue_by().map(|dur| dur.num_seconds()).unwrap_or(0);
                let debt_minutes = (overdue_secs / 60).max(10) as u32;

                items.push(TemporalDebt::new(
                    &format!("Overdue deadline: {}", d.label),
                    debt_minutes,
                    DebtCategory::Testing,
                ));
            }
        }

        let total_debt: u32 = items.iter().map(|d| d.calculate_current_total()).sum();
        let interest_per_week = (total_debt as f64 * 0.1).round() as u32;

        let recommendations = items
            .iter()
            .map(|d| PayoffRecommendation {
                debt_id: d.id,
                current_cost_minutes: d.calculate_current_total(),
                future_cost_if_unpaid: (d.calculate_current_total() as f64 * 1.1).round() as u32,
                roi_percentage: 10.0,
            })
            .collect();

        Ok(DebtReport {
            total_debt_minutes: total_debt,
            items,
            interest_per_week_minutes: interest_per_week,
            recommendations,
        })
    }

    // ── 8. Chrono-Gravity ───────────────────────────────────────────────

    /// Detect chrono-gravity wells — high-mass events pulling everything toward them.
    pub fn gravity_detect(&self) -> TimeResult<Vec<ChronoGravity>> {
        let now = Utc::now();
        let mut wells = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.status != DeadlineStatus::Pending && d.status != DeadlineStatus::Warning {
                continue;
            }

            let remaining_hours = (d.due_at - now).num_hours();
            if remaining_hours <= 0 {
                continue;
            }

            let mass = (168.0 / remaining_hours as f64).min(10.0);
            if mass < 1.0 {
                continue;
            }

            let radius_days = (remaining_hours / 24).max(1) as u32;

            let mut attracted = Vec::new();
            for sblock in self.file.list_by_type(EntityType::Schedule) {
                let s: Schedule = sblock.deserialize()?;
                if s.status == ScheduleStatus::Scheduled {
                    let gap_hours = (d.due_at - s.start_at).num_hours().abs();
                    if gap_hours < (radius_days as i64 * 24) {
                        let pull = 1.0 - (gap_hours as f64 / (radius_days as f64 * 24.0));
                        attracted.push(AttractedItem {
                            item_id: s.id,
                            item_type: "schedule".to_string(),
                            pull_strength: pull.max(0.0),
                            will_reach_event: s.start_at < d.due_at,
                        });
                    }
                }
            }

            wells.push(ChronoGravity {
                id: TemporalId::new(),
                event_id: d.id,
                mass,
                radius_days,
                event_horizon: if mass > 5.0 {
                    Some(d.due_at - ChronoDuration::hours(24))
                } else {
                    None
                },
                attracted_items: attracted,
                time_dilation_factor: 1.0 + (mass * 0.1),
            });
        }

        wells.sort_by(|a, b| {
            b.mass
                .partial_cmp(&a.mass)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(wells)
    }

    // ── 9. Temporal Entanglement ────────────────────────────────────────

    /// Create a temporal entanglement between entities.
    pub fn entanglement_create(
        &self,
        entity_ids: Vec<TemporalId>,
        entanglement_type: EntanglementType,
    ) -> TimeResult<TemporalEntanglement> {
        if entity_ids.len() < 2 {
            return Err(TimeError::InvalidInput(
                "Entanglement requires at least 2 entities".to_string(),
            ));
        }

        Ok(TemporalEntanglement {
            id: TemporalId::new(),
            entities: entity_ids,
            entanglement_type,
            active: true,
            created_at: Utc::now(),
            notify: Vec::new(),
        })
    }

    // ── 10. Temporal Immune System (Anomaly Scan) ───────────────────────

    /// Scan the temporal graph for anomalies.
    pub fn anomaly_scan(&self) -> TimeResult<ImmuneStatus> {
        let now = Utc::now();
        let mut anomalies = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.status == DeadlineStatus::Overdue {
                anomalies.push(TemporalAnomaly {
                    id: TemporalId::new(),
                    anomaly_type: AnomalyType::ImpossibleDeadline,
                    severity: 0.8,
                    involved_entities: vec![d.id],
                    description: format!("Deadline '{}' is overdue", d.label),
                    resolutions: vec![Resolution {
                        action: "Extend deadline or cancel".to_string(),
                        removes_anomaly: true,
                        side_effects: vec!["May affect dependent tasks".to_string()],
                    }],
                    detected_at: now,
                });
            }
        }

        let schedules: Vec<Schedule> = self
            .file
            .list_by_type(EntityType::Schedule)
            .iter()
            .filter_map(|b| b.deserialize().ok())
            .filter(|s: &Schedule| s.status == ScheduleStatus::Scheduled)
            .collect();

        for i in 0..schedules.len() {
            let overlap_count = (i + 1..schedules.len())
                .filter(|&j| schedules[i].conflicts_with(&schedules[j]))
                .count();

            if overlap_count >= 2 {
                anomalies.push(TemporalAnomaly {
                    id: TemporalId::new(),
                    anomaly_type: AnomalyType::CapacityOverflow,
                    severity: 0.6,
                    involved_entities: vec![schedules[i].id],
                    description: format!(
                        "Schedule '{}' has {} conflicting events",
                        schedules[i].label,
                        overlap_count + 1
                    ),
                    resolutions: vec![Resolution {
                        action: "Reschedule conflicting events".to_string(),
                        removes_anomaly: true,
                        side_effects: vec!["Rescheduling cascades".to_string()],
                    }],
                    detected_at: now,
                });
            }
        }

        let anomaly_count = anomalies.len() as u32;
        let health_score = if anomaly_count == 0 {
            1.0
        } else {
            (1.0 - (anomaly_count as f64 * 0.1)).max(0.0)
        };

        Ok(ImmuneStatus {
            anomaly_count,
            anomalies,
            last_scan: now,
            health_score,
        })
    }

    // ── 11. Decay Reversal ──────────────────────────────────────────────

    /// Analyze reversal options for a decaying value.
    pub fn decay_reversal(&self, decay_id: &TemporalId) -> TimeResult<DecayReversal> {
        let model: DecayModel = self
            .file
            .get(decay_id)?
            .ok_or_else(|| TimeError::NotFound(decay_id.0.to_string()))?;

        let options = vec![
            ReversalOption {
                name: "Full refresh".to_string(),
                time_required_secs: 3600,
                restored_value: model.initial_value,
                steps: vec![
                    "Identify decay source".to_string(),
                    "Apply full refresh".to_string(),
                    "Verify restoration".to_string(),
                ],
                durability_secs: 86400,
            },
            ReversalOption {
                name: "Partial boost".to_string(),
                time_required_secs: 900,
                restored_value: model.current_value
                    + (model.initial_value - model.current_value) * 0.5,
                steps: vec![
                    "Apply quick boost".to_string(),
                    "Monitor value stability".to_string(),
                ],
                durability_secs: 43200,
            },
            ReversalOption {
                name: "Slow recovery".to_string(),
                time_required_secs: 7200,
                restored_value: model.initial_value * 0.9,
                steps: vec![
                    "Gradual re-engagement".to_string(),
                    "Incremental value restoration".to_string(),
                    "Sustained maintenance".to_string(),
                ],
                durability_secs: 172800,
            },
        ];

        Ok(DecayReversal {
            id: TemporalId::new(),
            decay_id: *decay_id,
            current_value: model.current_value,
            target_value: model.initial_value,
            options,
            recommended: Some(0),
        })
    }

    // ── 12. Time Dilation Awareness ─────────────────────────────────────

    /// Analyze time dilation patterns based on schedule and deadline data.
    pub fn dilation_analyze(&self) -> TimeResult<SubjectiveTimeReport> {
        let now = Utc::now();
        let period_start = now - ChronoDuration::hours(24);
        let period_end = now;

        let mut scheduled_minutes = 0i64;
        let mut deadline_pressure = 0.0f64;

        for block in self.file.list_by_type(EntityType::Schedule) {
            let s: Schedule = block.deserialize()?;
            if s.start_at >= period_start && s.start_at <= period_end {
                scheduled_minutes += s.duration_secs / 60;
            }
        }

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.due_at >= period_start && d.due_at <= period_end + ChronoDuration::hours(24) {
                deadline_pressure += 1.0;
            }
        }

        let clock_hours = 24.0;
        let productive_hours = (scheduled_minutes as f64 / 60.0).min(clock_hours);
        let wasted = clock_hours - productive_hours;

        let dilation_factor = 1.0 + (deadline_pressure * 0.1);
        let perceived_total = clock_hours * dilation_factor;

        let mut recommendations = Vec::new();
        if deadline_pressure > 3.0 {
            recommendations
                .push("High deadline density — consider spacing out deadlines".to_string());
        }
        if productive_hours < 4.0 {
            recommendations
                .push("Low scheduled time — consider time-blocking more activities".to_string());
        }

        Ok(SubjectiveTimeReport {
            period_start,
            period_end,
            clock_hours,
            perceived_productive_hours: productive_hours,
            perceived_wasted_hours: wasted,
            total_perceived_hours: perceived_total,
            recommendations,
        })
    }

    // ── 13. Temporal Jump ───────────────────────────────────────────────

    /// Create a temporal jump to a specific point in time.
    pub fn temporal_jump(&self, destination: DateTime<Utc>) -> TimeResult<TemporalJump> {
        let mut jump = TemporalJump::new(destination);

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.created_at <= destination {
                jump.known_facts.push(KnownFact {
                    fact: format!(
                        "Deadline '{}' existed (due {})",
                        d.label,
                        d.due_at.to_rfc3339()
                    ),
                    source: "deadline".to_string(),
                    confidence_at_time: 1.0,
                });

                if d.due_at <= destination && d.status == DeadlineStatus::Overdue {
                    jump.unknown_facts.push(UnknownFact {
                        fact: format!("Deadline '{}' would become overdue", d.label),
                        discovered_at: d.due_at,
                        impact: "Missed deadline".to_string(),
                        missed_signals: vec!["Time tracking".to_string()],
                    });
                }
            }
        }

        Ok(jump)
    }

    // ── 14. Temporal Anchors ────────────────────────────────────────────

    /// Create a temporal anchor (savepoint) capturing current state.
    pub fn anchor_create(&self, name: &str, reason: &str) -> TimeResult<TemporalAnchor> {
        let mut anchor = TemporalAnchor::new(name, reason);

        let mut active_deadlines = Vec::new();
        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            if d.status == DeadlineStatus::Pending || d.status == DeadlineStatus::Warning {
                active_deadlines.push(d.id);
            }
        }

        let mut active_schedules = Vec::new();
        for block in self.file.list_by_type(EntityType::Schedule) {
            let s: Schedule = block.deserialize()?;
            if s.status == ScheduleStatus::Scheduled {
                active_schedules.push(s.id);
            }
        }

        let mut active_sequences = Vec::new();
        for block in self.file.list_by_type(EntityType::Sequence) {
            let s: Sequence = block.deserialize()?;
            if s.status != crate::sequence::SequenceStatus::Completed {
                active_sequences.push(s.id);
            }
        }

        let mut decay_values = Vec::new();
        for block in self.file.list_by_type(EntityType::Decay) {
            let d: DecayModel = block.deserialize()?;
            decay_values.push((d.id, d.current_value));
        }

        anchor.state_snapshot = StateSnapshot {
            codebase_state: None,
            memory_state: None,
            active_deadlines,
            active_schedules,
            active_sequences,
            decay_values,
            mental_model: None,
            checksum: String::new(),
        };

        Ok(anchor)
    }

    // ── 15. Time Loops ──────────────────────────────────────────────────

    /// Detect time loop patterns from repeated similar deadlines/events.
    pub fn loop_detect(&self) -> TimeResult<Vec<TimeLoop>> {
        let mut loops = Vec::new();
        let mut label_counts: std::collections::HashMap<String, Vec<Deadline>> =
            std::collections::HashMap::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let d: Deadline = block.deserialize()?;
            let normalized = d.label.to_lowercase();
            label_counts.entry(normalized).or_default().push(d);
        }

        for (label, deadlines) in &label_counts {
            if deadlines.len() >= 2 {
                let overdue_count = deadlines
                    .iter()
                    .filter(|d| d.status == DeadlineStatus::Overdue)
                    .count();

                if overdue_count >= 2 {
                    let pattern = LoopPattern {
                        repeating_actions: vec![format!("Creating deadline '{}'", label)],
                        trigger: "Recurring task that keeps being missed".to_string(),
                        expected_outcome: "Complete on time".to_string(),
                        actual_outcome: "Deadline missed repeatedly".to_string(),
                        similarity_score: 0.9,
                    };

                    let mut time_loop = TimeLoop::new(pattern);
                    for (i, d) in deadlines.iter().enumerate() {
                        time_loop.add_iteration(LoopIteration {
                            number: i as u32 + 1,
                            started_at: d.created_at,
                            ended_at: Some(d.due_at),
                            action_taken: format!("Set deadline '{}'", d.label),
                            result: format!("{:?}", d.status),
                            duration_secs: (d.due_at - d.created_at).num_seconds(),
                        });
                    }

                    time_loop.root_cause = Some(RootCauseAnalysis {
                        root_cause: "Task scope may be underestimated".to_string(),
                        why_fix_fails: "Deadline set with same timeframe each time".to_string(),
                        evidence: vec![format!(
                            "{} iterations, {} overdue",
                            deadlines.len(),
                            overdue_count
                        )],
                        confidence: 0.7,
                    });

                    time_loop.breaker = Some(LoopBreaker {
                        action: "Increase time allocation or break task into smaller pieces"
                            .to_string(),
                        rationale: "Repeated overdue suggests underestimation".to_string(),
                        estimated_time_secs: 1800,
                        expected_outcome: "Realistic deadline that can be met".to_string(),
                        confidence: 0.7,
                    });

                    loops.push(time_loop);
                }
            }
        }

        Ok(loops)
    }

    // ── 16. Temporal Wormholes ──────────────────────────────────────────

    /// Create a temporal wormhole connecting two distant time points.
    #[allow(clippy::too_many_arguments)]
    pub fn wormhole_create(
        &self,
        label_a: &str,
        time_a: DateTime<Utc>,
        desc_a: &str,
        label_b: &str,
        time_b: DateTime<Utc>,
        desc_b: &str,
        relationship: &str,
    ) -> TimeResult<TemporalWormhole> {
        let ep_a = WormholeEndpoint {
            timestamp: time_a,
            label: label_a.to_string(),
            event_description: desc_a.to_string(),
            related_entities: Vec::new(),
            context_summary: String::new(),
        };
        let ep_b = WormholeEndpoint {
            timestamp: time_b,
            label: label_b.to_string(),
            event_description: desc_b.to_string(),
            related_entities: Vec::new(),
            context_summary: String::new(),
        };

        Ok(TemporalWormhole::new(ep_a, ep_b, relationship))
    }
}

/// Report from `update_all_statuses`.
#[derive(Debug, Default)]
pub struct UpdateReport {
    /// Number of deadlines whose status changed.
    pub deadlines_updated: usize,
    /// Number of decay models recalculated.
    pub decays_updated: usize,
    /// IDs of newly overdue deadlines.
    pub new_overdue: Vec<TemporalId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deadline::Deadline;
    use chrono::Duration as ChronoDuration;
    use tempfile::tempdir;

    #[test]
    fn test_add_and_complete_deadline() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.atime");
        let tf = TimeFile::create(&path).unwrap();
        let mut engine = WriteEngine::new(tf);

        let d = Deadline::new("Test deadline", Utc::now() + ChronoDuration::hours(24));
        let id = engine.add_deadline(d).unwrap();

        engine.complete_deadline(&id).unwrap();

        let loaded: Deadline = engine.file().get(&id).unwrap().unwrap();
        assert_eq!(loaded.status, DeadlineStatus::Completed);
    }
}
