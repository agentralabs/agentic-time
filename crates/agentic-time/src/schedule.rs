//! Scheduling with recurrence and conflict detection.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::TemporalId;

/// A scheduled event or task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// Unique identifier.
    pub id: TemporalId,

    /// What is scheduled.
    pub label: String,

    /// When it starts.
    pub start_at: DateTime<Utc>,

    /// Expected duration in seconds.
    pub duration_secs: i64,

    /// Recurrence pattern (if any).
    pub recurrence: Option<Recurrence>,

    /// Priority level.
    pub priority: Priority,

    /// Can this be moved?
    pub flexible: bool,

    /// Buffer time before (seconds).
    pub buffer_before_secs: Option<i64>,

    /// Buffer time after (seconds).
    pub buffer_after_secs: Option<i64>,

    /// Dependencies (must complete before this).
    pub dependencies: Vec<TemporalId>,

    /// Blocked by (cannot start until these complete).
    pub blocked_by: Vec<TemporalId>,

    /// Status.
    pub status: ScheduleStatus,

    /// When created.
    pub created_at: DateTime<Utc>,

    /// Tags.
    pub tags: Vec<String>,
}

/// Recurrence configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurrence {
    /// Pattern type.
    pub pattern: RecurrencePattern,

    /// End condition.
    pub ends: RecurrenceEnd,

    /// Exceptions (dates to skip).
    pub exceptions: Vec<DateTime<Utc>>,
}

/// Recurrence pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrencePattern {
    /// Every N days.
    Daily {
        /// Interval in days.
        interval: u32,
    },

    /// Every N weeks on specific days.
    Weekly {
        /// Interval in weeks.
        interval: u32,
        /// Days of the week (0=Mon, 6=Sun).
        days: Vec<u32>,
    },

    /// Every N months on specific day.
    Monthly {
        /// Interval in months.
        interval: u32,
        /// Day of month (1-31).
        day_of_month: u32,
    },

    /// Custom cron expression.
    Cron {
        /// Cron expression string.
        expression: String,
    },
}

/// When recurrence ends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrenceEnd {
    /// Never ends.
    Never,
    /// Ends after N occurrences.
    After {
        /// Number of occurrences.
        count: u32,
    },
    /// Ends on specific date.
    On {
        /// End date.
        date: DateTime<Utc>,
    },
}

/// Priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    /// Critical priority.
    Critical,
    /// High priority.
    High,
    /// Medium priority.
    Medium,
    /// Low priority.
    Low,
    /// Optional.
    Optional,
}

/// Schedule status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleStatus {
    /// Scheduled but not started.
    Scheduled,
    /// Currently in progress.
    InProgress,
    /// Completed.
    Completed,
    /// Cancelled.
    Cancelled,
    /// Rescheduled to a new time.
    Rescheduled,
}

impl Schedule {
    /// Create a new schedule.
    pub fn new(label: impl Into<String>, start_at: DateTime<Utc>, duration_secs: i64) -> Self {
        Self {
            id: TemporalId::new(),
            label: label.into(),
            start_at,
            duration_secs,
            recurrence: None,
            priority: Priority::Medium,
            flexible: true,
            buffer_before_secs: None,
            buffer_after_secs: None,
            dependencies: Vec::new(),
            blocked_by: Vec::new(),
            status: ScheduleStatus::Scheduled,
            created_at: Utc::now(),
            tags: Vec::new(),
        }
    }

    /// Calculate end time.
    pub fn end_at(&self) -> DateTime<Utc> {
        self.start_at + ChronoDuration::seconds(self.duration_secs)
    }

    /// Check if conflicts with another schedule.
    pub fn conflicts_with(&self, other: &Schedule) -> bool {
        let self_end = self.end_at();
        let other_end = other.end_at();
        self.start_at < other_end && self_end > other.start_at
    }

    /// Get next occurrence (for recurring schedules).
    pub fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.recurrence.as_ref().map(|r| match &r.pattern {
            RecurrencePattern::Daily { interval } => {
                let days_since = (after - self.start_at).num_days();
                let next_interval = ((days_since / *interval as i64) + 1) * *interval as i64;
                self.start_at + ChronoDuration::days(next_interval)
            }
            RecurrencePattern::Weekly { interval, .. } => {
                let weeks_since = (after - self.start_at).num_weeks();
                let next_interval = ((weeks_since / *interval as i64) + 1) * *interval as i64;
                self.start_at + ChronoDuration::weeks(next_interval)
            }
            RecurrencePattern::Monthly { interval, .. } => {
                // Simplified: add N months worth of days
                let months_since = (after - self.start_at).num_days() / 30;
                let next_interval = ((months_since / *interval as i64) + 1) * *interval as i64;
                self.start_at + ChronoDuration::days(next_interval * 30)
            }
            RecurrencePattern::Cron { .. } => {
                // Cron parsing would require a cron crate — return next day as fallback
                after + ChronoDuration::days(1)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_at() {
        let start = Utc::now();
        let s = Schedule::new("Standup", start, 1800); // 30 min
        assert_eq!(s.end_at(), start + ChronoDuration::seconds(1800));
    }

    #[test]
    fn test_conflicts() {
        let now = Utc::now();
        let a = Schedule::new("A", now, 3600);
        let b = Schedule::new("B", now + ChronoDuration::seconds(1800), 3600);
        assert!(a.conflicts_with(&b));
    }

    #[test]
    fn test_no_conflict() {
        let now = Utc::now();
        let a = Schedule::new("A", now, 3600);
        let b = Schedule::new("B", now + ChronoDuration::seconds(7200), 3600);
        assert!(!a.conflicts_with(&b));
    }
}
