//! Sequence index — track active sequences for fast lookups.

use std::collections::HashMap;

use crate::sequence::{Sequence, SequenceStatus};
use crate::TemporalId;

/// Sequence index for fast lookups by status.
pub struct SequenceIndex {
    /// Sequences by status.
    by_status: HashMap<SequenceStatus, Vec<TemporalId>>,
}

impl SequenceIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self {
            by_status: HashMap::new(),
        }
    }

    /// Insert a sequence into the index.
    pub fn insert(&mut self, sequence: &Sequence) {
        self.by_status
            .entry(sequence.status)
            .or_default()
            .push(sequence.id);
    }

    /// Update a sequence's status in the index.
    pub fn update(
        &mut self,
        id: &TemporalId,
        old_status: SequenceStatus,
        new_status: SequenceStatus,
    ) {
        if let Some(ids) = self.by_status.get_mut(&old_status) {
            ids.retain(|i| i != id);
        }
        self.by_status.entry(new_status).or_default().push(*id);
    }

    /// Get IDs of in-progress sequences.
    pub fn in_progress(&self) -> &[TemporalId] {
        self.by_status
            .get(&SequenceStatus::InProgress)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

impl Default for SequenceIndex {
    fn default() -> Self {
        Self::new()
    }
}
