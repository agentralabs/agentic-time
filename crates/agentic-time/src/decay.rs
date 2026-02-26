//! Decay modeling — value/relevance degradation over time.

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::TemporalId;

/// Models how value/relevance decays over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayModel {
    /// Unique identifier.
    pub id: TemporalId,

    /// What this decay applies to.
    pub label: String,

    /// Initial value (at t=0).
    pub initial_value: f64,

    /// Current calculated value.
    pub current_value: f64,

    /// Reference time (when value was initial_value).
    pub reference_time: DateTime<Utc>,

    /// Decay function.
    pub decay_type: DecayType,

    /// Minimum value (floor).
    pub floor: f64,

    /// When last calculated.
    pub last_calculated: DateTime<Utc>,

    /// Tags.
    pub tags: Vec<String>,
}

/// Type of decay function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecayType {
    /// Linear decay: value = initial - (rate * time).
    Linear {
        /// Units lost per second.
        rate: f64,
    },

    /// Exponential decay: value = initial * e^(-λt).
    Exponential {
        /// Decay constant (λ).
        lambda: f64,
    },

    /// Half-life decay: value = initial * (0.5)^(t/half_life).
    HalfLife {
        /// Half-life in seconds.
        half_life_secs: i64,
    },

    /// Step decay: drops at specific intervals.
    Step {
        /// Drop amount at each interval.
        drop_amount: f64,
        /// Interval between drops in seconds.
        interval_secs: i64,
    },

    /// Custom decay curve.
    Custom {
        /// Points defining the curve (time_offset_secs, value).
        points: Vec<(i64, f64)>,
    },
}

impl DecayModel {
    /// Create a new decay model.
    pub fn new(label: impl Into<String>, initial_value: f64, decay_type: DecayType) -> Self {
        let now = Utc::now();
        Self {
            id: TemporalId::new(),
            label: label.into(),
            initial_value,
            current_value: initial_value,
            reference_time: now,
            decay_type,
            floor: 0.0,
            last_calculated: now,
            tags: Vec::new(),
        }
    }

    /// Calculate value at a specific time.
    pub fn calculate_value(&self, at: DateTime<Utc>) -> f64 {
        let elapsed_secs = (at - self.reference_time).num_seconds() as f64;

        if elapsed_secs < 0.0 {
            return self.initial_value;
        }

        let decayed = match &self.decay_type {
            DecayType::Linear { rate } => self.initial_value - (rate * elapsed_secs),
            DecayType::Exponential { lambda } => {
                self.initial_value * (-lambda * elapsed_secs).exp()
            }
            DecayType::HalfLife { half_life_secs } => {
                let half_life = *half_life_secs as f64;
                self.initial_value * (0.5_f64).powf(elapsed_secs / half_life)
            }
            DecayType::Step {
                drop_amount,
                interval_secs,
            } => {
                let interval = *interval_secs as f64;
                let intervals = (elapsed_secs / interval).floor();
                self.initial_value - (drop_amount * intervals)
            }
            DecayType::Custom { points } => self.interpolate_custom(elapsed_secs as i64, points),
        };

        decayed.max(self.floor)
    }

    fn interpolate_custom(&self, elapsed: i64, points: &[(i64, f64)]) -> f64 {
        if points.is_empty() {
            return self.initial_value;
        }

        let mut prev = (0i64, self.initial_value);
        for &(t, v) in points {
            if t > elapsed {
                let ratio = (elapsed - prev.0) as f64 / (t - prev.0) as f64;
                return prev.1 + ratio * (v - prev.1);
            }
            prev = (t, v);
        }

        prev.1
    }

    /// Update current_value to now.
    pub fn update(&mut self) {
        let now = Utc::now();
        self.current_value = self.calculate_value(now);
        self.last_calculated = now;
    }

    /// Time until value reaches threshold.
    pub fn time_until(&self, threshold: f64) -> Option<ChronoDuration> {
        if threshold <= self.floor {
            return None; // Will never reach below floor
        }

        if self.current_value <= threshold {
            return Some(ChronoDuration::zero());
        }

        let secs = match &self.decay_type {
            DecayType::Linear { rate } => {
                if *rate <= 0.0 {
                    return None;
                }
                (self.initial_value - threshold) / rate
            }
            DecayType::Exponential { lambda } => {
                if *lambda <= 0.0 {
                    return None;
                }
                -(threshold / self.initial_value).ln() / lambda
            }
            DecayType::HalfLife { half_life_secs } => {
                let half_life = *half_life_secs as f64;
                half_life * (threshold / self.initial_value).log2().abs()
            }
            _ => return None, // Complex calculation for Step/Custom
        };

        Some(ChronoDuration::seconds(secs as i64))
    }

    /// Reset decay (refresh the value back to initial).
    pub fn refresh(&mut self) {
        self.reference_time = Utc::now();
        self.current_value = self.initial_value;
        self.last_calculated = self.reference_time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_decay() {
        let d = DecayModel::new("test", 100.0, DecayType::Linear { rate: 1.0 });
        let future = d.reference_time + ChronoDuration::seconds(50);
        assert!((d.calculate_value(future) - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_floor() {
        let mut d = DecayModel::new("test", 100.0, DecayType::Linear { rate: 1.0 });
        d.floor = 10.0;
        let far_future = d.reference_time + ChronoDuration::seconds(200);
        assert!((d.calculate_value(far_future) - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_exponential_decay() {
        let d = DecayModel::new("test", 100.0, DecayType::Exponential { lambda: 0.01 });
        let at = d.reference_time + ChronoDuration::seconds(100);
        // 100 * e^(-0.01 * 100) = 100 * e^(-1) ≈ 36.79
        let val = d.calculate_value(at);
        assert!((val - 36.79).abs() < 1.0);
    }

    #[test]
    fn test_half_life_decay() {
        let d = DecayModel::new(
            "test",
            100.0,
            DecayType::HalfLife {
                half_life_secs: 3600,
            },
        );
        let at = d.reference_time + ChronoDuration::seconds(3600);
        let val = d.calculate_value(at);
        assert!((val - 50.0).abs() < 0.01);
    }
}
