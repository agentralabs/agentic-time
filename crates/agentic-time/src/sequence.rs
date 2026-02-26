//! Sequence constraints — ordered temporal events with dependencies.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::TemporalId;

/// A sequence of ordered temporal events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    /// Unique identifier.
    pub id: TemporalId,

    /// Sequence name.
    pub label: String,

    /// Ordered steps.
    pub steps: Vec<SequenceStep>,

    /// Current step index.
    pub current_step: usize,

    /// Overall status.
    pub status: SequenceStatus,

    /// When sequence started.
    pub started_at: Option<DateTime<Utc>>,

    /// When sequence completed.
    pub completed_at: Option<DateTime<Utc>>,

    /// Allow parallel execution?
    pub allow_parallel: bool,

    /// Created at.
    pub created_at: DateTime<Utc>,

    /// Tags.
    pub tags: Vec<String>,
}

/// A single step in a sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStep {
    /// Step identifier.
    pub id: TemporalId,

    /// Step label.
    pub label: String,

    /// Order in sequence (0-indexed).
    pub order: u32,

    /// Expected duration in seconds.
    pub duration_secs: Option<i64>,

    /// Step status.
    pub status: StepStatus,

    /// Can be done in parallel with next step?
    pub parallel_with_next: bool,

    /// Dependencies within sequence (order indices).
    pub depends_on: Vec<u32>,

    /// When step started.
    pub started_at: Option<DateTime<Utc>>,

    /// When step completed.
    pub completed_at: Option<DateTime<Utc>>,
}

/// Overall sequence status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SequenceStatus {
    /// Not yet started.
    NotStarted,
    /// Currently in progress.
    InProgress,
    /// All steps completed.
    Completed,
    /// A step failed.
    Failed,
    /// Cancelled.
    Cancelled,
}

/// Individual step status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    /// Waiting for dependencies.
    Pending,
    /// Dependencies met, can start.
    Ready,
    /// Currently running.
    InProgress,
    /// Done.
    Completed,
    /// Skipped.
    Skipped,
    /// Failed.
    Failed,
}

impl Sequence {
    /// Create a new sequence.
    pub fn new(label: impl Into<String>, steps: Vec<SequenceStep>) -> Self {
        Self {
            id: TemporalId::new(),
            label: label.into(),
            steps,
            current_step: 0,
            status: SequenceStatus::NotStarted,
            started_at: None,
            completed_at: None,
            allow_parallel: false,
            created_at: Utc::now(),
            tags: Vec::new(),
        }
    }

    /// Get steps that are ready to execute.
    pub fn ready_steps(&self) -> Vec<&SequenceStep> {
        self.steps
            .iter()
            .filter(|s| {
                s.status == StepStatus::Ready
                    || (s.status == StepStatus::Pending && self.dependencies_met(s))
            })
            .collect()
    }

    /// Check if all dependencies for a step are met.
    pub fn dependencies_met(&self, step: &SequenceStep) -> bool {
        step.depends_on.iter().all(|dep_order| {
            self.steps
                .iter()
                .find(|s| s.order == *dep_order)
                .map(|s| s.status == StepStatus::Completed)
                .unwrap_or(true)
        })
    }

    /// Calculate total expected duration.
    pub fn total_duration(&self) -> ChronoDuration {
        if self.allow_parallel {
            self.steps
                .iter()
                .filter_map(|s| s.duration_secs)
                .map(ChronoDuration::seconds)
                .max()
                .unwrap_or(ChronoDuration::zero())
        } else {
            self.steps
                .iter()
                .filter_map(|s| s.duration_secs)
                .map(ChronoDuration::seconds)
                .fold(ChronoDuration::zero(), |acc, d| acc + d)
        }
    }

    /// Complete the current step and advance.
    pub fn complete_current_step(&mut self) {
        if self.status == SequenceStatus::NotStarted {
            self.status = SequenceStatus::InProgress;
            self.started_at = Some(Utc::now());
        }

        if let Some(step) = self.steps.get_mut(self.current_step) {
            step.status = StepStatus::Completed;
            step.completed_at = Some(Utc::now());
        }

        self.current_step += 1;

        if self.current_step >= self.steps.len() {
            self.status = SequenceStatus::Completed;
            self.completed_at = Some(Utc::now());
        }

        self.update_ready_steps();
    }

    /// Update which steps are ready based on dependency state.
    pub fn update_ready_steps(&mut self) {
        let completed_orders: Vec<u32> = self
            .steps
            .iter()
            .filter(|s| s.status == StepStatus::Completed)
            .map(|s| s.order)
            .collect();

        for step in &mut self.steps {
            if step.status == StepStatus::Pending {
                let deps_met = step
                    .depends_on
                    .iter()
                    .all(|dep| completed_orders.contains(dep));
                if deps_met {
                    step.status = StepStatus::Ready;
                }
            }
        }
    }
}

impl SequenceStep {
    /// Create a new sequence step.
    pub fn new(label: impl Into<String>, order: u32) -> Self {
        Self {
            id: TemporalId::new(),
            label: label.into(),
            order,
            duration_secs: None,
            status: StepStatus::Pending,
            parallel_with_next: false,
            depends_on: Vec::new(),
            started_at: None,
            completed_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_advance() {
        let steps = vec![
            SequenceStep::new("Step 1", 0),
            SequenceStep::new("Step 2", 1),
        ];
        let mut seq = Sequence::new("Deploy", steps);
        assert_eq!(seq.status, SequenceStatus::NotStarted);

        seq.complete_current_step();
        assert_eq!(seq.status, SequenceStatus::InProgress);
        assert_eq!(seq.steps[0].status, StepStatus::Completed);

        seq.complete_current_step();
        assert_eq!(seq.status, SequenceStatus::Completed);
    }

    #[test]
    fn test_total_duration_sequential() {
        let mut steps = vec![
            SequenceStep::new("Step 1", 0),
            SequenceStep::new("Step 2", 1),
        ];
        steps[0].duration_secs = Some(60);
        steps[1].duration_secs = Some(120);

        let seq = Sequence::new("Pipeline", steps);
        assert_eq!(seq.total_duration().num_seconds(), 180);
    }
}
