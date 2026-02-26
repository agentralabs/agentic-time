//! Error types for AgenticTime.

use thiserror::Error;

/// Errors that can occur in temporal operations.
#[derive(Error, Debug)]
pub enum TimeError {
    /// Entity not found by ID.
    #[error("Temporal entity not found: {0}")]
    NotFound(String),

    /// Invalid time range (start after end).
    #[error("Invalid time range: start {start} is after end {end}")]
    InvalidRange {
        /// Range start.
        start: String,
        /// Range end.
        end: String,
    },

    /// Deadline has already passed.
    #[error("Deadline already passed: {0}")]
    DeadlinePassed(String),

    /// Schedule conflict detected.
    #[error("Schedule conflict: {0} overlaps with {1}")]
    ScheduleConflict(String, String),

    /// Sequence dependency not met.
    #[error("Sequence dependency not met: step {step} depends on {dependency}")]
    DependencyNotMet {
        /// The step that cannot proceed.
        step: String,
        /// The unmet dependency.
        dependency: String,
    },

    /// Invalid recurrence pattern.
    #[error("Invalid recurrence pattern: {0}")]
    InvalidRecurrence(String),

    /// Invalid duration (e.g. negative).
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    /// File format error.
    #[error("File format error: {0}")]
    FileFormat(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Result type alias for temporal operations.
pub type TimeResult<T> = Result<T, TimeError>;
