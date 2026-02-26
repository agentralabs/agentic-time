//! Decay index — track decay models by current value.

use std::collections::BTreeMap;

use crate::decay::DecayModel;
use crate::TemporalId;

/// Decay index for fast value-based queries.
pub struct DecayIndex {
    /// Decays sorted by current value (value × 1000 as integer key → IDs).
    by_value: BTreeMap<i64, Vec<TemporalId>>,
}

impl DecayIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self {
            by_value: BTreeMap::new(),
        }
    }

    /// Insert a decay model into the index.
    pub fn insert(&mut self, decay: &DecayModel) {
        let key = (decay.current_value * 1000.0) as i64;
        self.by_value.entry(key).or_default().push(decay.id);
    }

    /// Get decay IDs below a threshold.
    pub fn below_threshold(&self, threshold: f64) -> Vec<TemporalId> {
        let key = (threshold * 1000.0) as i64;
        self.by_value
            .range(..key)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }

    /// Get the decay with the lowest current value.
    pub fn lowest(&self) -> Option<TemporalId> {
        self.by_value
            .iter()
            .flat_map(|(_, ids)| ids.first().copied())
            .next()
    }
}

impl Default for DecayIndex {
    fn default() -> Self {
        Self::new()
    }
}
