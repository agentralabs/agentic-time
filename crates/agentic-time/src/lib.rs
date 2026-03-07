//! # AgenticTime
//!
//! Temporal reasoning for AI agents. Models durations, deadlines,
//! schedules, sequences, and decay in a single `.atime` file.

pub mod bridges;
pub mod cache;
pub mod contracts;
pub mod deadline;
pub mod decay;
pub mod duration;
pub mod error;
pub mod file_format;
pub mod indexes;
pub mod inventions;
pub mod metrics;
pub mod query;
pub mod query_engine;
pub mod schedule;
pub mod sequence;
pub mod write_engine;

pub use deadline::{Consequence, Deadline, DeadlineStatus, DeadlineType};
pub use decay::{DecayModel, DecayType};
pub use duration::{DurationEstimate, DurationSource};
pub use error::{TimeError, TimeResult};
pub use file_format::{EntityType, FileHeader, TimeFile};
pub use indexes::{DeadlineIndex, DecayIndex, SequenceIndex};
pub use query_engine::{QueryEngine, TimeSlot, TimeStats};
pub use schedule::{
    Priority, Recurrence, RecurrenceEnd, RecurrencePattern, Schedule, ScheduleStatus,
};
pub use sequence::{Sequence, SequenceStatus, SequenceStep, StepStatus};
pub use write_engine::{UpdateReport, WriteEngine};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for temporal entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemporalId(pub Uuid);

impl TemporalId {
    /// Generate a new random temporal ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TemporalId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TemporalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TemporalId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}
