//! Write engine — create and update temporal entities.

use chrono::{DateTime, Utc};

use crate::deadline::{Deadline, DeadlineStatus};
use crate::decay::DecayModel;
use crate::duration::DurationEstimate;
use crate::error::{TimeError, TimeResult};
use crate::file_format::{EntityType, TimeFile};
use crate::schedule::{Schedule, ScheduleStatus};
use crate::sequence::Sequence;
use crate::TemporalId;

/// Write engine for temporal operations.
pub struct WriteEngine {
    file: TimeFile,
}

impl WriteEngine {
    /// Create a new write engine wrapping a TimeFile.
    pub fn new(file: TimeFile) -> Self {
        Self { file }
    }

    /// Consume the engine, returning the underlying TimeFile.
    pub fn into_file(self) -> TimeFile {
        self.file
    }

    /// Get a reference to the underlying TimeFile.
    pub fn file(&self) -> &TimeFile {
        &self.file
    }

    /// Get a mutable reference to the underlying TimeFile.
    pub fn file_mut(&mut self) -> &mut TimeFile {
        &mut self.file
    }

    // --- Durations ---

    /// Add a duration estimate.
    pub fn add_duration(&mut self, duration: DurationEstimate) -> TimeResult<TemporalId> {
        let id = duration.id;
        self.file.add(EntityType::Duration, id, &duration)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing duration estimate.
    pub fn update_duration(&mut self, duration: DurationEstimate) -> TimeResult<()> {
        self.file
            .add(EntityType::Duration, duration.id, &duration)?;
        self.file.save()?;
        Ok(())
    }

    // --- Deadlines ---

    /// Add a deadline.
    pub fn add_deadline(&mut self, deadline: Deadline) -> TimeResult<TemporalId> {
        let id = deadline.id;
        self.file.add(EntityType::Deadline, id, &deadline)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing deadline.
    pub fn update_deadline(&mut self, deadline: Deadline) -> TimeResult<()> {
        self.file
            .add(EntityType::Deadline, deadline.id, &deadline)?;
        self.file.save()?;
        Ok(())
    }

    /// Complete a deadline.
    pub fn complete_deadline(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut deadline: Deadline = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        deadline.complete();
        self.update_deadline(deadline)
    }

    /// Cancel a deadline.
    pub fn cancel_deadline(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut deadline: Deadline = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        deadline.status = DeadlineStatus::Cancelled;
        self.update_deadline(deadline)
    }

    // --- Schedules ---

    /// Add a schedule. Returns error if it conflicts with existing non-flexible schedules.
    pub fn add_schedule(&mut self, schedule: Schedule) -> TimeResult<TemporalId> {
        let conflicts = self.check_schedule_conflicts(&schedule)?;
        if !conflicts.is_empty() && !schedule.flexible {
            return Err(TimeError::ScheduleConflict(
                schedule.label.clone(),
                conflicts[0].label.clone(),
            ));
        }

        let id = schedule.id;
        self.file.add(EntityType::Schedule, id, &schedule)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing schedule.
    pub fn update_schedule(&mut self, schedule: Schedule) -> TimeResult<()> {
        self.file
            .add(EntityType::Schedule, schedule.id, &schedule)?;
        self.file.save()?;
        Ok(())
    }

    /// Reschedule to a new start time.
    pub fn reschedule(&mut self, id: &TemporalId, new_start: DateTime<Utc>) -> TimeResult<()> {
        let mut schedule: Schedule = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        schedule.start_at = new_start;
        schedule.status = ScheduleStatus::Rescheduled;
        self.update_schedule(schedule)
    }

    /// Check for conflicting schedules.
    fn check_schedule_conflicts(&self, schedule: &Schedule) -> TimeResult<Vec<Schedule>> {
        let mut conflicts = Vec::new();

        for block in self.file.list_by_type(EntityType::Schedule) {
            let existing: Schedule = block.deserialize()?;
            if existing.id != schedule.id
                && existing.status == ScheduleStatus::Scheduled
                && schedule.conflicts_with(&existing)
            {
                conflicts.push(existing);
            }
        }

        Ok(conflicts)
    }

    // --- Sequences ---

    /// Add a sequence.
    pub fn add_sequence(&mut self, sequence: Sequence) -> TimeResult<TemporalId> {
        let id = sequence.id;
        self.file.add(EntityType::Sequence, id, &sequence)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing sequence.
    pub fn update_sequence(&mut self, sequence: Sequence) -> TimeResult<()> {
        self.file
            .add(EntityType::Sequence, sequence.id, &sequence)?;
        self.file.save()?;
        Ok(())
    }

    /// Advance a sequence to the next step.
    pub fn advance_sequence(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut sequence: Sequence = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        sequence.complete_current_step();
        self.update_sequence(sequence)
    }

    // --- Decay ---

    /// Add a decay model.
    pub fn add_decay(&mut self, decay: DecayModel) -> TimeResult<TemporalId> {
        let id = decay.id;
        self.file.add(EntityType::Decay, id, &decay)?;
        self.file.save()?;
        Ok(id)
    }

    /// Update an existing decay model.
    pub fn update_decay(&mut self, decay: DecayModel) -> TimeResult<()> {
        self.file.add(EntityType::Decay, decay.id, &decay)?;
        self.file.save()?;
        Ok(())
    }

    /// Refresh a decay model (recalculate current value).
    pub fn refresh_decay(&mut self, id: &TemporalId) -> TimeResult<f64> {
        let mut decay: DecayModel = self
            .file
            .get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        decay.update();
        let value = decay.current_value;
        self.update_decay(decay)?;
        Ok(value)
    }

    // --- Batch operations ---

    /// Update all statuses (deadlines + decays) based on current time.
    pub fn update_all_statuses(&mut self) -> TimeResult<UpdateReport> {
        let mut report = UpdateReport::default();

        // Collect deadline blocks to avoid borrow issues
        let deadline_blocks: Vec<_> = self
            .file
            .list_by_type(EntityType::Deadline)
            .into_iter()
            .cloned()
            .collect();

        for block in deadline_blocks {
            let mut deadline: Deadline = block.deserialize()?;
            let old_status = deadline.status;
            deadline.update_status();
            if deadline.status != old_status {
                self.file
                    .add(EntityType::Deadline, deadline.id, &deadline)?;
                report.deadlines_updated += 1;
                if deadline.status == DeadlineStatus::Overdue {
                    report.new_overdue.push(deadline.id);
                }
            }
        }

        // Collect decay blocks
        let decay_blocks: Vec<_> = self
            .file
            .list_by_type(EntityType::Decay)
            .into_iter()
            .cloned()
            .collect();

        for block in decay_blocks {
            let mut decay: DecayModel = block.deserialize()?;
            decay.update();
            self.file.add(EntityType::Decay, decay.id, &decay)?;
            report.decays_updated += 1;
        }

        self.file.save()?;
        Ok(report)
    }
}

/// Report from `update_all_statuses`.
#[derive(Debug, Default)]
pub struct UpdateReport {
    /// Number of deadlines whose status changed.
    pub deadlines_updated: usize,
    /// Number of decay models recalculated.
    pub decays_updated: usize,
    /// IDs of newly overdue deadlines.
    pub new_overdue: Vec<TemporalId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deadline::Deadline;
    use chrono::Duration as ChronoDuration;
    use tempfile::tempdir;

    #[test]
    fn test_add_and_complete_deadline() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.atime");
        let tf = TimeFile::create(&path).unwrap();
        let mut engine = WriteEngine::new(tf);

        let d = Deadline::new("Test deadline", Utc::now() + ChronoDuration::hours(24));
        let id = engine.add_deadline(d).unwrap();

        engine.complete_deadline(&id).unwrap();

        let loaded: Deadline = engine.file().get(&id).unwrap().unwrap();
        assert_eq!(loaded.status, DeadlineStatus::Completed);
    }
}
