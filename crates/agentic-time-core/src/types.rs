// Core types for AgenticTime

use serde::{Deserialize, Serialize};

/// Priority levels for deadlines.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of a deadline.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeadlineStatus {
    Pending,
    InProgress,
    Completed,
    Missed,
    Cancelled,
}

/// A deadline entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    pub id: u64,
    pub title: String,
    pub due_at: i64,
    pub priority: Priority,
    pub status: DeadlineStatus,
    pub tags: Vec<String>,
    pub depends_on: Vec<u64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// A duration estimate entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationEstimate {
    pub id: u64,
    pub label: String,
    pub estimate_seconds: u64,
    pub confidence: f64,
    pub actual_seconds: Option<u64>,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub created_at: i64,
}

/// A schedule entry entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: u64,
    pub title: String,
    pub recurrence: String,
    pub duration_minutes: u32,
    pub timezone: String,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub created_at: i64,
}

/// Step status within a sequence.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Completed,
    Skipped,
}

/// A single step in a sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStep {
    pub label: String,
    pub duration_minutes: u32,
    pub status: StepStatus,
}

/// An ordered sequence of steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    pub id: u64,
    pub title: String,
    pub steps: Vec<SequenceStep>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Decay curve type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DecayCurve {
    Exponential,
    Linear,
    Step,
}

/// Decay configuration entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayConfig {
    pub id: u64,
    pub name: String,
    pub curve: DecayCurve,
    pub halflife_hours: Option<f64>,
    pub window_hours: Option<f64>,
    pub threshold_hours: Option<f64>,
    pub floor: Option<f64>,
    pub created_at: i64,
}
