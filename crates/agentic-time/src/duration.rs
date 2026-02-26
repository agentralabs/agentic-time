//! Duration estimation with uncertainty (PERT model).

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::TemporalId;

/// A duration estimate with uncertainty using the PERT model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationEstimate {
    /// Unique identifier.
    pub id: TemporalId,

    /// What this duration is for.
    pub label: String,

    /// Optimistic estimate (best case) in seconds.
    pub optimistic_secs: i64,

    /// Most likely estimate in seconds.
    pub expected_secs: i64,

    /// Pessimistic estimate (worst case) in seconds.
    pub pessimistic_secs: i64,

    /// Confidence level (0.0 - 1.0).
    pub confidence: f64,

    /// Source of this estimate.
    pub source: DurationSource,

    /// When this estimate was created.
    pub created_at: DateTime<Utc>,

    /// Tags for categorization.
    pub tags: Vec<String>,
}

/// Source of a duration estimate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurationSource {
    /// User provided the estimate.
    UserEstimate,
    /// Derived from historical data.
    Historical {
        /// Number of historical samples.
        sample_count: u32,
    },
    /// AI predicted.
    Predicted {
        /// Model used.
        model: String,
        /// Prediction confidence.
        confidence: f64,
    },
    /// Default/fallback.
    Default,
}

impl DurationEstimate {
    /// Create a new duration estimate.
    pub fn new(label: impl Into<String>, optimistic: i64, expected: i64, pessimistic: i64) -> Self {
        Self {
            id: TemporalId::new(),
            label: label.into(),
            optimistic_secs: optimistic,
            expected_secs: expected,
            pessimistic_secs: pessimistic,
            confidence: 0.8,
            source: DurationSource::UserEstimate,
            created_at: Utc::now(),
            tags: Vec::new(),
        }
    }

    /// Calculate PERT estimate: (O + 4M + P) / 6
    pub fn pert_estimate(&self) -> ChronoDuration {
        let pert = (self.optimistic_secs + 4 * self.expected_secs + self.pessimistic_secs) / 6;
        ChronoDuration::seconds(pert)
    }

    /// Calculate standard deviation: (P - O) / 6
    pub fn std_deviation(&self) -> ChronoDuration {
        let sd = (self.pessimistic_secs - self.optimistic_secs) / 6;
        ChronoDuration::seconds(sd)
    }

    /// Get optimistic duration.
    pub fn optimistic(&self) -> ChronoDuration {
        ChronoDuration::seconds(self.optimistic_secs)
    }

    /// Get expected duration.
    pub fn expected(&self) -> ChronoDuration {
        ChronoDuration::seconds(self.expected_secs)
    }

    /// Get pessimistic duration.
    pub fn pessimistic(&self) -> ChronoDuration {
        ChronoDuration::seconds(self.pessimistic_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pert_estimate() {
        let d = DurationEstimate::new("test", 60, 120, 300);
        // (60 + 4*120 + 300) / 6 = (60 + 480 + 300) / 6 = 840 / 6 = 140
        assert_eq!(d.pert_estimate().num_seconds(), 140);
    }

    #[test]
    fn test_std_deviation() {
        let d = DurationEstimate::new("test", 60, 120, 300);
        // (300 - 60) / 6 = 240 / 6 = 40
        assert_eq!(d.std_deviation().num_seconds(), 40);
    }
}
