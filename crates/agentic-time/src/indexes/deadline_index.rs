//! Deadline index — sorted by due date for fast temporal lookups.

use std::collections::{BTreeMap, HashMap};

use crate::deadline::{Deadline, DeadlineStatus};
use crate::TemporalId;

/// Deadline index for fast temporal queries.
pub struct DeadlineIndex {
    /// Deadlines sorted by due date (Unix timestamp → IDs).
    by_due_date: BTreeMap<i64, Vec<TemporalId>>,
    /// Deadlines by status.
    by_status: HashMap<DeadlineStatus, Vec<TemporalId>>,
}

impl DeadlineIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self {
            by_due_date: BTreeMap::new(),
            by_status: HashMap::new(),
        }
    }

    /// Insert a deadline into the index.
    pub fn insert(&mut self, deadline: &Deadline) {
        let timestamp = deadline.due_at.timestamp();
        self.by_due_date
            .entry(timestamp)
            .or_default()
            .push(deadline.id);

        self.by_status
            .entry(deadline.status)
            .or_default()
            .push(deadline.id);
    }

    /// Remove a deadline from the index.
    pub fn remove(&mut self, deadline: &Deadline) {
        let timestamp = deadline.due_at.timestamp();
        if let Some(ids) = self.by_due_date.get_mut(&timestamp) {
            ids.retain(|id| *id != deadline.id);
        }

        if let Some(ids) = self.by_status.get_mut(&deadline.status) {
            ids.retain(|id| *id != deadline.id);
        }
    }

    /// Get deadline IDs due before a given timestamp.
    pub fn due_before(&self, timestamp: i64) -> Vec<TemporalId> {
        self.by_due_date
            .range(..timestamp)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }

    /// Get deadline IDs due in a range.
    pub fn due_in_range(&self, start: i64, end: i64) -> Vec<TemporalId> {
        self.by_due_date
            .range(start..end)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }

    /// Get the next upcoming deadline ID.
    pub fn next_deadline(&self) -> Option<TemporalId> {
        self.by_due_date
            .iter()
            .flat_map(|(_, ids)| ids.first().copied())
            .next()
    }
}

impl Default for DeadlineIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deadline::Deadline;
    use chrono::{Duration as ChronoDuration, Utc};

    #[test]
    fn test_insert_and_query() {
        let mut idx = DeadlineIndex::new();
        let d = Deadline::new("Test", Utc::now() + ChronoDuration::hours(2));
        idx.insert(&d);

        let due = idx.due_before((Utc::now() + ChronoDuration::hours(3)).timestamp());
        assert_eq!(due.len(), 1);
        assert_eq!(due[0], d.id);
    }
}
