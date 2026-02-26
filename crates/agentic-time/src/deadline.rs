//! Deadline management with urgency and consequences.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::TemporalId;

/// A deadline with urgency and consequences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    /// Unique identifier.
    pub id: TemporalId,

    /// What this deadline is for.
    pub label: String,

    /// The deadline timestamp.
    pub due_at: DateTime<Utc>,

    /// Soft deadline (warning threshold).
    pub warn_at: Option<DateTime<Utc>>,

    /// Type of deadline.
    pub deadline_type: DeadlineType,

    /// What happens if missed.
    pub consequence: Consequence,

    /// Current status.
    pub status: DeadlineStatus,

    /// Related temporal entities.
    pub dependencies: Vec<TemporalId>,

    /// When this deadline was created.
    pub created_at: DateTime<Utc>,

    /// When this deadline was completed (if completed).
    pub completed_at: Option<DateTime<Utc>>,

    /// Tags for categorization.
    pub tags: Vec<String>,

    /// Additional context.
    pub context: Option<String>,
}

/// Type of deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlineType {
    /// Hard deadline — cannot be moved.
    Hard,
    /// Soft deadline — can be negotiated.
    Soft,
    /// Target — aspirational, not committed.
    Target,
    /// Recurring — repeats on schedule.
    Recurring,
}

/// Consequence of missing a deadline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Consequence {
    /// No significant consequence.
    None,
    /// Minor inconvenience.
    Minor {
        /// Description of the consequence.
        description: String,
    },
    /// Moderate impact.
    Moderate {
        /// Description of the consequence.
        description: String,
    },
    /// Severe impact.
    Severe {
        /// Description of the consequence.
        description: String,
    },
    /// Critical failure.
    Critical {
        /// Description of the consequence.
        description: String,
    },
    /// Quantified cost.
    Quantified {
        /// Description of the consequence.
        description: String,
        /// Estimated cost.
        cost_estimate: f64,
        /// Cost unit (e.g. "USD", "hours").
        cost_unit: String,
    },
}

/// Status of a deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeadlineStatus {
    /// Not yet due.
    Pending,
    /// Warning threshold reached.
    Warning,
    /// Due now or overdue.
    Overdue,
    /// Completed on time.
    Completed,
    /// Completed late.
    CompletedLate,
    /// Cancelled.
    Cancelled,
}

impl Deadline {
    /// Create a new deadline.
    pub fn new(label: impl Into<String>, due_at: DateTime<Utc>) -> Self {
        Self {
            id: TemporalId::new(),
            label: label.into(),
            due_at,
            warn_at: None,
            deadline_type: DeadlineType::Soft,
            consequence: Consequence::None,
            status: DeadlineStatus::Pending,
            dependencies: Vec::new(),
            created_at: Utc::now(),
            completed_at: None,
            tags: Vec::new(),
            context: None,
        }
    }

    /// Calculate time remaining until due.
    pub fn time_remaining(&self) -> Option<ChronoDuration> {
        if self.status == DeadlineStatus::Completed
            || self.status == DeadlineStatus::CompletedLate
            || self.status == DeadlineStatus::Cancelled
        {
            return None;
        }
        let now = Utc::now();
        if now < self.due_at {
            Some(self.due_at - now)
        } else {
            None
        }
    }

    /// Calculate how long past due.
    pub fn overdue_by(&self) -> Option<ChronoDuration> {
        let now = Utc::now();
        if now > self.due_at
            && self.status != DeadlineStatus::Completed
            && self.status != DeadlineStatus::CompletedLate
            && self.status != DeadlineStatus::Cancelled
        {
            Some(now - self.due_at)
        } else {
            None
        }
    }

    /// Update status based on current time.
    pub fn update_status(&mut self) {
        if self.status == DeadlineStatus::Completed
            || self.status == DeadlineStatus::CompletedLate
            || self.status == DeadlineStatus::Cancelled
        {
            return;
        }

        let now = Utc::now();

        if now > self.due_at {
            self.status = DeadlineStatus::Overdue;
        } else if let Some(warn_at) = self.warn_at {
            if now > warn_at {
                self.status = DeadlineStatus::Warning;
            }
        }
    }

    /// Mark as completed.
    pub fn complete(&mut self) {
        let now = Utc::now();
        self.completed_at = Some(now);

        if now > self.due_at {
            self.status = DeadlineStatus::CompletedLate;
        } else {
            self.status = DeadlineStatus::Completed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deadline() {
        let due = Utc::now() + ChronoDuration::hours(24);
        let d = Deadline::new("Ship v1", due);
        assert_eq!(d.status, DeadlineStatus::Pending);
        assert!(d.time_remaining().is_some());
    }

    #[test]
    fn test_complete_on_time() {
        let due = Utc::now() + ChronoDuration::hours(24);
        let mut d = Deadline::new("Test", due);
        d.complete();
        assert_eq!(d.status, DeadlineStatus::Completed);
        assert!(d.completed_at.is_some());
    }

    #[test]
    fn test_overdue_status() {
        let due = Utc::now() - ChronoDuration::hours(1);
        let mut d = Deadline::new("Late", due);
        d.update_status();
        assert_eq!(d.status, DeadlineStatus::Overdue);
    }
}
