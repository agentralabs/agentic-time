//! Query engine — temporal queries over the `.atime` graph.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::deadline::{Deadline, DeadlineStatus};
use crate::decay::DecayModel;
use crate::duration::{DurationEstimate, DurationSource};
use crate::error::TimeResult;
use crate::file_format::{EntityType, TimeFile};
use crate::schedule::Schedule;
use crate::sequence::{Sequence, SequenceStatus, StepStatus};
use crate::TemporalId;

/// Query engine for temporal data.
pub struct QueryEngine<'a> {
    file: &'a TimeFile,
}

impl<'a> QueryEngine<'a> {
    /// Create a new query engine over a TimeFile.
    pub fn new(file: &'a TimeFile) -> Self {
        Self { file }
    }

    // --- Deadline queries ---

    /// Get all overdue deadlines.
    pub fn overdue_deadlines(&self) -> TimeResult<Vec<Deadline>> {
        let now = Utc::now();
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            if deadline.due_at < now
                && deadline.status != DeadlineStatus::Completed
                && deadline.status != DeadlineStatus::CompletedLate
                && deadline.status != DeadlineStatus::Cancelled
            {
                results.push(deadline);
            }
        }

        results.sort_by_key(|d| d.due_at);
        Ok(results)
    }

    /// Get deadlines due within a given duration from now.
    pub fn deadlines_due_within(&self, duration: ChronoDuration) -> TimeResult<Vec<Deadline>> {
        let now = Utc::now();
        let cutoff = now + duration;
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            if deadline.due_at > now
                && deadline.due_at <= cutoff
                && deadline.status == DeadlineStatus::Pending
            {
                results.push(deadline);
            }
        }

        results.sort_by_key(|d| d.due_at);
        Ok(results)
    }

    /// Get deadlines by status.
    pub fn deadlines_by_status(&self, status: DeadlineStatus) -> TimeResult<Vec<Deadline>> {
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            if deadline.status == status {
                results.push(deadline);
            }
        }

        Ok(results)
    }

    // --- Schedule queries ---

    /// Get schedules in a time range.
    pub fn schedules_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> TimeResult<Vec<Schedule>> {
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Schedule) {
            let schedule: Schedule = block.deserialize()?;
            let schedule_end = schedule.end_at();

            if schedule.start_at < end && schedule_end > start {
                results.push(schedule);
            }
        }

        results.sort_by_key(|s| s.start_at);
        Ok(results)
    }

    /// Get conflicting schedules for a proposed schedule.
    pub fn schedule_conflicts(&self, schedule: &Schedule) -> TimeResult<Vec<Schedule>> {
        let mut conflicts = Vec::new();

        for block in self.file.list_by_type(EntityType::Schedule) {
            let existing: Schedule = block.deserialize()?;
            if existing.id != schedule.id && schedule.conflicts_with(&existing) {
                conflicts.push(existing);
            }
        }

        Ok(conflicts)
    }

    /// Get available time slots in a range.
    pub fn available_slots(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        min_duration: ChronoDuration,
    ) -> TimeResult<Vec<TimeSlot>> {
        let scheduled = self.schedules_in_range(start, end)?;
        let mut slots = Vec::new();
        let mut current = start;

        for schedule in scheduled {
            if schedule.start_at > current {
                let gap = schedule.start_at - current;
                if gap >= min_duration {
                    slots.push(TimeSlot {
                        start: current,
                        end: schedule.start_at,
                        duration_secs: gap.num_seconds(),
                    });
                }
            }
            current = schedule.end_at().max(current);
        }

        if current < end {
            let gap = end - current;
            if gap >= min_duration {
                slots.push(TimeSlot {
                    start: current,
                    end,
                    duration_secs: gap.num_seconds(),
                });
            }
        }

        Ok(slots)
    }

    // --- Sequence queries ---

    /// Get a sequence by ID.
    pub fn sequence(&self, id: &TemporalId) -> TimeResult<Option<Sequence>> {
        self.file.get(id)
    }

    /// Get all active sequences.
    pub fn active_sequences(&self) -> TimeResult<Vec<Sequence>> {
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Sequence) {
            let sequence: Sequence = block.deserialize()?;
            if sequence.status == SequenceStatus::InProgress {
                results.push(sequence);
            }
        }

        Ok(results)
    }

    /// Get blocked sequences (steps waiting on dependencies).
    pub fn blocked_sequences(&self) -> TimeResult<Vec<(Sequence, Vec<TemporalId>)>> {
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Sequence) {
            let sequence: Sequence = block.deserialize()?;
            let blocked: Vec<_> = sequence
                .steps
                .iter()
                .filter(|s| s.status == StepStatus::Pending)
                .flat_map(|s| &s.depends_on)
                .filter(|&dep_order| {
                    sequence
                        .steps
                        .iter()
                        .find(|s| s.order == *dep_order)
                        .map(|s| s.status != StepStatus::Completed)
                        .unwrap_or(false)
                })
                .map(|_| sequence.id)
                .collect();

            if !blocked.is_empty() {
                results.push((sequence, blocked));
            }
        }

        Ok(results)
    }

    // --- Decay queries ---

    /// Get decay models below a threshold.
    pub fn decays_below_threshold(&self, threshold: f64) -> TimeResult<Vec<DecayModel>> {
        let mut results = Vec::new();
        let now = Utc::now();

        for block in self.file.list_by_type(EntityType::Decay) {
            let decay: DecayModel = block.deserialize()?;
            let current_value = decay.calculate_value(now);
            if current_value < threshold {
                results.push(decay);
            }
        }

        Ok(results)
    }

    /// Get decays that will reach threshold within a given duration.
    pub fn decays_reaching_threshold(
        &self,
        threshold: f64,
        within: ChronoDuration,
    ) -> TimeResult<Vec<(DecayModel, ChronoDuration)>> {
        let mut results = Vec::new();

        for block in self.file.list_by_type(EntityType::Decay) {
            let decay: DecayModel = block.deserialize()?;
            if let Some(time_until) = decay.time_until(threshold) {
                if time_until <= within {
                    results.push((decay, time_until));
                }
            }
        }

        results.sort_by_key(|(_, d)| *d);
        Ok(results)
    }

    // --- Duration queries ---

    /// Estimate total duration for a set of tasks.
    pub fn estimate_total(&self, ids: &[TemporalId]) -> TimeResult<DurationEstimate> {
        let mut optimistic = 0i64;
        let mut expected = 0i64;
        let mut pessimistic = 0i64;

        for id in ids {
            if let Some(duration) = self.file.get::<DurationEstimate>(id)? {
                optimistic += duration.optimistic_secs;
                expected += duration.expected_secs;
                pessimistic += duration.pessimistic_secs;
            }
        }

        Ok(DurationEstimate {
            id: TemporalId::new(),
            label: "Combined estimate".to_string(),
            optimistic_secs: optimistic,
            expected_secs: expected,
            pessimistic_secs: pessimistic,
            confidence: 0.5,
            source: DurationSource::Predicted {
                model: "aggregation".to_string(),
                confidence: 0.5,
            },
            created_at: Utc::now(),
            tags: vec![],
        })
    }

    // --- Stats ---

    /// Get temporal statistics.
    pub fn stats(&self) -> TimeResult<TimeStats> {
        let mut stats = TimeStats::default();

        for _block in self.file.list_by_type(EntityType::Duration) {
            stats.duration_count += 1;
        }

        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            stats.deadline_count += 1;
            match deadline.status {
                DeadlineStatus::Overdue => stats.overdue_count += 1,
                DeadlineStatus::Completed | DeadlineStatus::CompletedLate => {
                    stats.completed_count += 1
                }
                DeadlineStatus::Warning => stats.warning_count += 1,
                _ => stats.pending_count += 1,
            }
        }

        for _block in self.file.list_by_type(EntityType::Schedule) {
            stats.schedule_count += 1;
        }

        for _block in self.file.list_by_type(EntityType::Sequence) {
            stats.sequence_count += 1;
        }

        for _block in self.file.list_by_type(EntityType::Decay) {
            stats.decay_count += 1;
        }

        Ok(stats)
    }
}

/// A free time slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    /// Slot start.
    pub start: DateTime<Utc>,
    /// Slot end.
    pub end: DateTime<Utc>,
    /// Duration in seconds.
    pub duration_secs: i64,
}

/// Temporal statistics.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeStats {
    /// Number of duration estimates.
    pub duration_count: usize,
    /// Number of deadlines.
    pub deadline_count: usize,
    /// Number of schedules.
    pub schedule_count: usize,
    /// Number of sequences.
    pub sequence_count: usize,
    /// Number of decay models.
    pub decay_count: usize,
    /// Number of overdue deadlines.
    pub overdue_count: usize,
    /// Number of warning-state deadlines.
    pub warning_count: usize,
    /// Number of pending deadlines.
    pub pending_count: usize,
    /// Number of completed deadlines.
    pub completed_count: usize,
}
