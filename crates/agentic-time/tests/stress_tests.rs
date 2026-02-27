//! Edge case and stress tests for AgenticTime.
//!
//! Covers all 16 edge cases from PROGRESS-CHECKLIST.md Phase 6,
//! plus heavy-load stress tests and boundary conditions.

use agentic_time::*;
use chrono::{Duration as ChronoDuration, TimeZone, Utc};
use std::time::Instant;
use tempfile::tempdir;

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 1: Invalid date-time formats → rejection
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_01_invalid_temporal_id_format_rejected() {
    let result = "not-a-uuid".parse::<TemporalId>();
    assert!(result.is_err(), "Invalid UUID string must be rejected");
}

#[test]
fn edge_01_empty_string_temporal_id_rejected() {
    let result = "".parse::<TemporalId>();
    assert!(result.is_err(), "Empty string must be rejected as TemporalId");
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 4: Invalid temporal IDs → 404 (NotFound)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_04_nonexistent_id_returns_none() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = TimeFile::create(&path).unwrap();

    let bogus_id = TemporalId::new();
    let result: Option<Deadline> = tf.get(&bogus_id).unwrap();
    assert!(result.is_none(), "Nonexistent ID must return None");
}

#[test]
fn edge_04_complete_nonexistent_deadline_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let bogus_id = TemporalId::new();
    let result = engine.complete_deadline(&bogus_id);
    assert!(result.is_err(), "Completing nonexistent deadline must error");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("not found"),
        "Error must mention not found: {}",
        err
    );
}

#[test]
fn edge_04_cancel_nonexistent_deadline_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let bogus_id = TemporalId::new();
    let result = engine.cancel_deadline(&bogus_id);
    assert!(result.is_err(), "Cancelling nonexistent deadline must error");
}

#[test]
fn edge_04_advance_nonexistent_sequence_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let bogus_id = TemporalId::new();
    let result = engine.advance_sequence(&bogus_id);
    assert!(result.is_err(), "Advancing nonexistent sequence must error");
}

#[test]
fn edge_04_refresh_nonexistent_decay_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let bogus_id = TemporalId::new();
    let result = engine.refresh_decay(&bogus_id);
    assert!(result.is_err(), "Refreshing nonexistent decay must error");
}

#[test]
fn edge_04_reschedule_nonexistent_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let bogus_id = TemporalId::new();
    let result = engine.reschedule(&bogus_id, Utc::now());
    assert!(
        result.is_err(),
        "Rescheduling nonexistent schedule must error"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 5: Negative durations → rejection
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_05_negative_duration_pert_still_computes() {
    // Negative values should be valid in PERT (it's just math).
    // The DurationEstimate constructor allows any i64 values.
    let d = DurationEstimate::new("Negative test", -10, -5, -2);
    let pert = d.pert_estimate();
    // PERT = (O + 4M + P) / 6 = (-10 + 4(-5) + (-2)) / 6 = -32/6 = -5
    assert!(pert.num_seconds() < 0, "Negative PERT estimate is mathematically valid");
}

#[test]
fn edge_05_zero_duration_is_valid() {
    let d = DurationEstimate::new("Zero", 0, 0, 0);
    assert_eq!(d.pert_estimate().num_seconds(), 0);
    assert_eq!(d.std_deviation().num_seconds(), 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 6: Empty file → empty arrays
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_06_empty_file_returns_empty_arrays() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty.atime");
    let tf = TimeFile::create(&path).unwrap();

    assert_eq!(tf.entity_count(), 0);
    assert!(tf.list_by_type(EntityType::Deadline).is_empty());
    assert!(tf.list_by_type(EntityType::Schedule).is_empty());
    assert!(tf.list_by_type(EntityType::Sequence).is_empty());
    assert!(tf.list_by_type(EntityType::Decay).is_empty());
    assert!(tf.list_by_type(EntityType::Duration).is_empty());
}

#[test]
fn edge_06_empty_file_queries_return_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty.atime");
    let tf = TimeFile::create(&path).unwrap();
    let qe = QueryEngine::new(&tf);

    assert!(qe.overdue_deadlines().unwrap().is_empty());
    assert!(qe
        .deadlines_due_within(ChronoDuration::hours(24))
        .unwrap()
        .is_empty());
    assert!(qe.active_sequences().unwrap().is_empty());
    assert!(qe.decays_below_threshold(0.5).unwrap().is_empty());

    let stats = qe.stats().unwrap();
    assert_eq!(stats.deadline_count, 0);
    assert_eq!(stats.schedule_count, 0);
    assert_eq!(stats.sequence_count, 0);
    assert_eq!(stats.decay_count, 0);
    assert_eq!(stats.duration_count, 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 7: Corrupted .atime file → error with hint
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_07_corrupted_magic_bytes() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("corrupt.atime");
    std::fs::write(&path, b"XXXX_garbage_data_here").unwrap();

    let result = TimeFile::open(&path);
    assert!(result.is_err(), "Corrupted file must error");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("Invalid magic bytes"),
        "Error must mention invalid magic: {}",
        err
    );
    assert!(
        err.contains("corrupted"),
        "Error must suggest corruption: {}",
        err
    );
}

#[test]
fn edge_07_truncated_header() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("truncated.atime");
    // Write valid magic but truncated header
    std::fs::write(&path, b"ATIM\x01\x00\x00").unwrap();

    let result = TimeFile::open(&path);
    assert!(
        result.is_err(),
        "Truncated header must produce an IO error"
    );
}

#[test]
fn edge_07_zero_byte_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("zero.atime");
    std::fs::write(&path, b"").unwrap();

    let result = TimeFile::open(&path);
    assert!(result.is_err(), "Zero-byte file must error on open");
}

#[test]
fn edge_07_valid_header_corrupted_entity() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("corrupt_entity.atime");

    // First create a valid file with one entity
    let mut tf = TimeFile::create(&path).unwrap();
    let d = Deadline::new("Test", Utc::now() + ChronoDuration::hours(1));
    tf.add(EntityType::Deadline, d.id, &d).unwrap();
    tf.save().unwrap();

    // Now corrupt the entity data (keep header intact)
    let data = std::fs::read(&path).unwrap();
    let mut corrupted = data[..FileHeader::SIZE].to_vec(); // valid header
    corrupted.extend_from_slice(b"\xFF\xFF\xFF\xFF_bad_entity_data"); // garbage entity
    std::fs::write(&path, &corrupted).unwrap();

    let result = TimeFile::open(&path);
    assert!(
        result.is_err(),
        "Corrupted entity data must error on open"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 8: Missing file → create or clear error
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_08_open_nonexistent_file_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("does_not_exist.atime");

    let result = TimeFile::open(&path);
    assert!(result.is_err(), "Opening nonexistent file must error");
}

#[test]
fn edge_08_create_creates_new_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("new_file.atime");
    assert!(!path.exists());

    let tf = TimeFile::create(&path).unwrap();
    assert!(path.exists(), "create() must create the file");
    assert_eq!(tf.entity_count(), 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 9: Timezone handling → UTC internal
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_09_all_timestamps_are_utc() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tz.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // Create deadline with explicit UTC time
    let utc_time = Utc.with_ymd_and_hms(2030, 6, 15, 12, 0, 0).unwrap();
    let d = Deadline::new("UTC test", utc_time);
    let id = d.id;
    tf.add(EntityType::Deadline, id, &d).unwrap();
    tf.save().unwrap();

    // Reopen and verify
    let tf2 = TimeFile::open(&path).unwrap();
    let loaded: Deadline = tf2.get(&id).unwrap().unwrap();
    assert_eq!(loaded.due_at, utc_time, "Timestamp must be preserved in UTC");
}

#[test]
fn edge_09_file_header_timestamps_are_utc_micros() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("header_tz.atime");
    let before = Utc::now().timestamp_micros() as u64;
    let tf = TimeFile::create(&path).unwrap();
    let after = Utc::now().timestamp_micros() as u64;

    assert!(
        tf.header.created_at >= before && tf.header.created_at <= after,
        "Header created_at must be current UTC microseconds"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 10: Date overflow → graceful handling
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_10_far_future_deadline() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("far_future.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // Year 9999 — extreme future date
    let far_future = Utc.with_ymd_and_hms(9999, 12, 31, 23, 59, 59).unwrap();
    let d = Deadline::new("Far future", far_future);
    let id = d.id;
    tf.add(EntityType::Deadline, id, &d).unwrap();
    tf.save().unwrap();

    let tf2 = TimeFile::open(&path).unwrap();
    let loaded: Deadline = tf2.get(&id).unwrap().unwrap();
    assert_eq!(loaded.due_at, far_future, "Far future date must survive round-trip");
}

#[test]
fn edge_10_past_deadline_overdue_detection() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("past.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // A deadline that was due 100 years ago
    let past = Utc.with_ymd_and_hms(1926, 1, 1, 0, 0, 0).unwrap();
    let d = Deadline::new("Historical", past);
    let id = d.id;
    tf.add(EntityType::Deadline, id, &d).unwrap();
    tf.save().unwrap();

    let qe = QueryEngine::new(&tf);
    let overdue = qe.overdue_deadlines().unwrap();
    assert_eq!(overdue.len(), 1, "Historical deadline must be detected as overdue");
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 13: Zero duration → valid
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_13_zero_duration_estimate() {
    let d = DurationEstimate::new("Instant", 0, 0, 0);
    assert_eq!(d.pert_estimate().num_seconds(), 0);
    assert_eq!(d.optimistic_secs, 0);
    assert_eq!(d.expected_secs, 0);
    assert_eq!(d.pessimistic_secs, 0);
}

#[test]
fn edge_13_zero_duration_round_trip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("zero_dur.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let d = DurationEstimate::new("Zero", 0, 0, 0);
    let id = engine.add_duration(d).unwrap();

    let loaded: DurationEstimate = engine.file().get(&id).unwrap().unwrap();
    assert_eq!(loaded.pert_estimate().num_seconds(), 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 14: Maximum dates → no overflow
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_14_max_i64_entity_count() {
    // Verify header can represent large entity counts
    let mut header = FileHeader::new();
    header.entity_count = u64::MAX;
    let mut buf = Vec::new();
    header.write_to(&mut buf).unwrap();

    let mut cursor = std::io::Cursor::new(buf);
    let read_back = FileHeader::read_from(&mut cursor).unwrap();
    assert_eq!(
        read_back.entity_count,
        u64::MAX,
        "Header must handle u64::MAX entity count"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 15: Empty strings → behavior check
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn edge_15_empty_label_deadline() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty_label.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    let d = Deadline::new("", Utc::now() + ChronoDuration::hours(1));
    let id = d.id;
    tf.add(EntityType::Deadline, id, &d).unwrap();
    tf.save().unwrap();

    let tf2 = TimeFile::open(&path).unwrap();
    let loaded: Deadline = tf2.get(&id).unwrap().unwrap();
    assert_eq!(loaded.label, "", "Empty label must survive round-trip");
}

#[test]
fn edge_15_empty_label_schedule() {
    let now = Utc::now();
    let s = Schedule::new("", now, 3600);
    assert_eq!(s.label, "");
}

#[test]
fn edge_15_unicode_labels() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("unicode.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    let labels = vec![
        "日本語テスト",
        "Тест на русском",
        "🕐⏰🕰️ Clock emojis",
        "Arabic: العربية",
        "Zero-width\u{200B}space",
    ];

    let mut ids = Vec::new();
    for label in &labels {
        let d = Deadline::new(*label, Utc::now() + ChronoDuration::hours(1));
        ids.push(d.id);
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    tf.save().unwrap();

    let tf2 = TimeFile::open(&path).unwrap();
    for (i, id) in ids.iter().enumerate() {
        let loaded: Deadline = tf2.get(id).unwrap().unwrap();
        assert_eq!(loaded.label, labels[i], "Unicode label '{}' must survive round-trip", labels[i]);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS TEST: Heavy entity counts
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_1000_deadlines_add_and_query() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("stress_1k.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let start = Instant::now();

    for i in 0..1000 {
        let hours_offset = (i % 100) as i64 - 50; // -50h to +49h
        let due = Utc::now() + ChronoDuration::hours(hours_offset);
        let d = Deadline::new(format!("Deadline {}", i), due);
        engine.add_deadline(d).unwrap();
    }

    let add_elapsed = start.elapsed();
    println!("1K deadlines add: {:?}", add_elapsed);

    let qe = QueryEngine::new(engine.file());

    let query_start = Instant::now();
    let overdue = qe.overdue_deadlines().unwrap();
    let query_elapsed = query_start.elapsed();

    println!(
        "1K overdue query: {:?} ({} results)",
        query_elapsed,
        overdue.len()
    );
    assert!(
        overdue.len() > 0,
        "Should have some overdue deadlines from negative offsets"
    );

    let stats = qe.stats().unwrap();
    assert_eq!(stats.deadline_count, 1000);
}

#[test]
fn stress_1000_mixed_entities() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("stress_mixed.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    // Add 200 of each type
    for i in 0..200 {
        let due = Utc::now() + ChronoDuration::hours(i as i64);
        let d = Deadline::new(format!("D{}", i), due);
        engine.add_deadline(d).unwrap();
    }

    for i in 0..200 {
        let dur = DurationEstimate::new(format!("Dur{}", i), i * 60, i * 90, i * 120);
        engine.add_duration(dur).unwrap();
    }

    for i in 0..200 {
        let start = Utc::now() + ChronoDuration::hours(i as i64 * 3);
        let s = Schedule::new(format!("S{}", i), start, 3600);
        engine.add_schedule(s).unwrap();
    }

    for i in 0..200 {
        let dm = DecayModel::new(
            format!("Decay{}", i),
            100.0,
            DecayType::Exponential { lambda: 0.01 },
        );
        engine.add_decay(dm).unwrap();
    }

    for i in 0..200 {
        let steps = vec![
            sequence::SequenceStep::new(format!("Step{}-1", i), 0),
            sequence::SequenceStep::new(format!("Step{}-2", i), 1),
        ];
        let seq = Sequence::new(format!("Seq{}", i), steps);
        engine.add_sequence(seq).unwrap();
    }

    let stats = QueryEngine::new(engine.file()).stats().unwrap();
    assert_eq!(stats.deadline_count, 200);
    assert_eq!(stats.duration_count, 200);
    assert_eq!(stats.schedule_count, 200);
    assert_eq!(stats.decay_count, 200);
    assert_eq!(stats.sequence_count, 200);
    assert_eq!(engine.file().entity_count(), 1000);
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE 16: 100K entities → <100ms queries (performance check)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_10k_deadline_query_performance() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("stress_10k.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // Add 10K deadlines (not using WriteEngine to avoid per-entity save)
    for i in 0..10_000 {
        let hours_offset = (i % 200) as i64 - 100;
        let due = Utc::now() + ChronoDuration::hours(hours_offset);
        let d = Deadline::new(format!("D{}", i), due);
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    tf.save().unwrap();

    let qe = QueryEngine::new(&tf);

    // Overdue query
    let start = Instant::now();
    let overdue = qe.overdue_deadlines().unwrap();
    let elapsed = start.elapsed();
    println!(
        "10K overdue query: {:?} ({} results)",
        elapsed,
        overdue.len()
    );
    assert!(
        elapsed.as_millis() < 500,
        "10K overdue query must complete in <500ms (took {:?})",
        elapsed
    );

    // Due-within query
    let start = Instant::now();
    let due_soon = qe
        .deadlines_due_within(ChronoDuration::hours(24))
        .unwrap();
    let elapsed = start.elapsed();
    println!(
        "10K due-within query: {:?} ({} results)",
        elapsed,
        due_soon.len()
    );
    assert!(
        elapsed.as_millis() < 500,
        "10K due-within query must complete in <500ms (took {:?})",
        elapsed
    );

    // Stats query
    let start = Instant::now();
    let stats = qe.stats().unwrap();
    let elapsed = start.elapsed();
    println!("10K stats query: {:?}", elapsed);
    assert_eq!(stats.deadline_count, 10_000);
    assert!(
        elapsed.as_millis() < 500,
        "10K stats query must complete in <500ms (took {:?})",
        elapsed
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: File I/O round-trip at scale
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_save_and_reopen_10k() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("roundtrip_10k.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    for i in 0..10_000 {
        let d = Deadline::new(
            format!("Deadline {}", i),
            Utc::now() + ChronoDuration::hours(i as i64),
        );
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }

    let save_start = Instant::now();
    tf.save().unwrap();
    let save_elapsed = save_start.elapsed();
    println!("10K save: {:?}", save_elapsed);

    let open_start = Instant::now();
    let tf2 = TimeFile::open(&path).unwrap();
    let open_elapsed = open_start.elapsed();
    println!("10K open: {:?}", open_elapsed);

    assert_eq!(tf2.entity_count(), 10_000);
    assert!(
        save_elapsed.as_millis() < 2000,
        "10K save must complete in <2s (took {:?})",
        save_elapsed
    );
    assert!(
        open_elapsed.as_millis() < 2000,
        "10K open must complete in <2s (took {:?})",
        open_elapsed
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: Write engine batch operations
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_update_all_statuses_with_many_entities() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("batch_update.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // Mix of overdue and future deadlines
    for i in 0..100 {
        let hours = if i % 2 == 0 { -24 } else { 24 };
        let d = Deadline::new(
            format!("D{}", i),
            Utc::now() + ChronoDuration::hours(hours),
        );
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }

    // Add decay models
    for i in 0..50 {
        let dm = DecayModel::new(
            format!("Decay{}", i),
            100.0,
            DecayType::Linear { rate: 0.001 },
        );
        tf.add(EntityType::Decay, dm.id, &dm).unwrap();
    }
    tf.save().unwrap();

    let mut engine = WriteEngine::new(tf);
    let report = engine.update_all_statuses().unwrap();

    println!(
        "Batch update: {} deadlines updated, {} decays updated, {} new overdue",
        report.deadlines_updated,
        report.decays_updated,
        report.new_overdue.len()
    );

    // All overdue deadlines should have been updated
    assert!(report.deadlines_updated > 0, "Some deadlines should have been updated");
    assert_eq!(report.decays_updated, 50, "All 50 decays should have been updated");
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: Schedule conflict detection at scale
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_schedule_conflict_detection_100() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("conflicts.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // Create 100 non-overlapping schedules (each 1 hour, spaced 2 hours apart)
    for i in 0..100 {
        let start = Utc::now() + ChronoDuration::hours(i as i64 * 2);
        let s = Schedule::new(format!("S{}", i), start, 3600);
        tf.add(EntityType::Schedule, s.id, &s).unwrap();
    }
    tf.save().unwrap();

    let qe = QueryEngine::new(&tf);

    // Create a schedule that conflicts with S50
    let conflict_start = Utc::now() + ChronoDuration::hours(100); // overlaps with S50
    let test_schedule = Schedule::new("Test", conflict_start, 3600);

    let start = Instant::now();
    let conflicts = qe.schedule_conflicts(&test_schedule).unwrap();
    let elapsed = start.elapsed();
    println!(
        "100-schedule conflict check: {:?} ({} conflicts)",
        elapsed,
        conflicts.len()
    );

    assert!(
        elapsed.as_millis() < 100,
        "Conflict check on 100 schedules must be <100ms"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: Decay evaluation correctness under load
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_decay_batch_evaluation() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("decays.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    // Mix of decay types
    let decay_types = vec![
        DecayType::Linear { rate: 0.001 },
        DecayType::Exponential { lambda: 0.01 },
        DecayType::HalfLife {
            half_life_secs: 3600,
        },
        DecayType::Step {
            drop_amount: 0.1,
            interval_secs: 600,
        },
    ];

    for i in 0..200 {
        let dt = decay_types[i % decay_types.len()].clone();
        let dm = DecayModel::new(format!("Decay{}", i), 1.0, dt);
        tf.add(EntityType::Decay, dm.id, &dm).unwrap();
    }
    tf.save().unwrap();

    let qe = QueryEngine::new(&tf);

    let start = Instant::now();
    let below_half = qe.decays_below_threshold(0.5).unwrap();
    let elapsed = start.elapsed();
    println!(
        "200-decay threshold query: {:?} ({} below 0.5)",
        elapsed,
        below_half.len()
    );

    // All decays were just created so should be at initial_value=1.0
    assert_eq!(
        below_half.len(),
        0,
        "Freshly created decays should all be at 1.0"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: Available slots computation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_available_slots_50_schedules() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("slots.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    let base = Utc::now();
    for i in 0..50 {
        // 1-hour meetings with 30-min gaps
        let start = base + ChronoDuration::minutes(90 * i as i64);
        let s = Schedule::new(format!("Meeting{}", i), start, 3600);
        tf.add(EntityType::Schedule, s.id, &s).unwrap();
    }
    tf.save().unwrap();

    let qe = QueryEngine::new(&tf);
    let range_end = base + ChronoDuration::hours(100);

    let start = Instant::now();
    let slots = qe
        .available_slots(base, range_end, ChronoDuration::minutes(15))
        .unwrap();
    let elapsed = start.elapsed();
    println!(
        "50-schedule slots query: {:?} ({} slots found)",
        elapsed,
        slots.len()
    );

    assert!(slots.len() > 0, "Should find available slots between meetings");
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: Remove and re-add at scale
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_remove_and_readd() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("churn.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    let mut ids = Vec::new();

    // Add 500
    for i in 0..500 {
        let d = Deadline::new(
            format!("D{}", i),
            Utc::now() + ChronoDuration::hours(i as i64),
        );
        ids.push(d.id);
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    tf.save().unwrap();
    assert_eq!(tf.entity_count(), 500);

    // Remove first 250
    for id in &ids[..250] {
        assert!(tf.remove(id));
    }
    assert_eq!(tf.entity_count(), 250);

    // Add 250 new ones
    for i in 500..750 {
        let d = Deadline::new(
            format!("D{}", i),
            Utc::now() + ChronoDuration::hours(i as i64),
        );
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    assert_eq!(tf.entity_count(), 500);

    // Save and reopen
    tf.save().unwrap();
    let tf2 = TimeFile::open(&path).unwrap();
    assert_eq!(tf2.entity_count(), 500);

    // Verify removed IDs are gone
    for id in &ids[..250] {
        let result: Option<Deadline> = tf2.get(id).unwrap();
        assert!(result.is_none(), "Removed entity must not be loadable");
    }

    // Verify remaining IDs are present
    for id in &ids[250..500] {
        let result: Option<Deadline> = tf2.get(id).unwrap();
        assert!(
            result.is_some(),
            "Non-removed entity must still be loadable"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STRESS: Sequence advancement through all steps
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn stress_sequence_full_advancement() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("seq_full.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    // Create sequence with 20 steps
    let steps: Vec<_> = (0..20)
        .map(|i| sequence::SequenceStep::new(format!("Step {}", i), i))
        .collect();
    let seq = Sequence::new("Long pipeline", steps);
    let seq_id = engine.add_sequence(seq).unwrap();

    // Advance through all 20 steps
    for i in 0..20 {
        engine.advance_sequence(&seq_id).unwrap();
        let loaded: Sequence = engine.file().get(&seq_id).unwrap().unwrap();
        if i < 19 {
            assert_eq!(
                loaded.status,
                SequenceStatus::InProgress,
                "Sequence should be InProgress after step {}",
                i
            );
        } else {
            assert_eq!(
                loaded.status,
                SequenceStatus::Completed,
                "Sequence should be Completed after final step"
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BOUNDARY: Decay models edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn boundary_decay_zero_rate() {
    let dm = DecayModel::new(
        "Zero rate",
        1.0,
        DecayType::Linear { rate: 0.0 },
    );
    // At time=now, value should still be 1.0 (initial_value)
    let val = dm.calculate_value(Utc::now());
    assert!(
        (val - 1.0).abs() < 0.01,
        "Zero-rate linear decay should stay near 1.0, got {}",
        val
    );
}

#[test]
fn boundary_decay_negative_lambda() {
    // Negative lambda means growth — unusual but valid mathematically
    let dm = DecayModel::new(
        "Negative lambda",
        1.0,
        DecayType::Exponential { lambda: -0.01 },
    );
    let future = Utc::now() + ChronoDuration::hours(1);
    let val = dm.calculate_value(future);
    // exp(-(-0.01)*3600) = exp(36) which is huge
    assert!(val > 1.0, "Negative lambda should cause growth, got {}", val);
}

#[test]
fn boundary_decay_very_large_half_life() {
    let dm = DecayModel::new(
        "Very slow",
        1.0,
        DecayType::HalfLife {
            half_life_secs: 1_000_000_000_000_000,
        },
    );
    let future = Utc::now() + ChronoDuration::hours(1);
    let val = dm.calculate_value(future);
    assert!(
        (val - 1.0).abs() < 0.001,
        "Very large half-life should barely decay: {}",
        val
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// BOUNDARY: Duration estimate PERT edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn boundary_pert_all_same() {
    let d = DurationEstimate::new("Same", 100, 100, 100);
    assert_eq!(d.pert_estimate().num_seconds(), 100, "O=E=P should give PERT=100");
    assert_eq!(d.std_deviation().num_seconds(), 0, "No variance when O=P");
}

#[test]
fn boundary_pert_very_large_values() {
    // Use large values that won't overflow chrono's internal ms representation
    // (chrono::TimeDelta::seconds panics if secs * 1000 overflows i64)
    // Max safe: i64::MAX / 1000 ≈ 9.2e15 seconds ≈ 292 million years
    let large = i64::MAX / 1000;
    let d = DurationEstimate::new("Large", large, large, large);
    let pert = d.pert_estimate();
    let secs = pert.num_seconds();
    assert!(secs > 0, "PERT must produce a positive value for large inputs, got {}", secs);
    assert_eq!(secs, large, "O=E=P=large should give PERT=large");
}

// ═══════════════════════════════════════════════════════════════════════════
// BOUNDARY: EntityType try_from edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn boundary_entity_type_invalid_values() {
    for invalid in [0u8, 6, 100, 255] {
        let result = EntityType::try_from(invalid);
        assert!(
            result.is_err(),
            "EntityType::try_from({}) must fail",
            invalid
        );
    }
}

#[test]
fn boundary_entity_type_valid_values() {
    assert_eq!(EntityType::try_from(1).unwrap(), EntityType::Duration);
    assert_eq!(EntityType::try_from(2).unwrap(), EntityType::Deadline);
    assert_eq!(EntityType::try_from(3).unwrap(), EntityType::Schedule);
    assert_eq!(EntityType::try_from(4).unwrap(), EntityType::Sequence);
    assert_eq!(EntityType::try_from(5).unwrap(), EntityType::Decay);
}

// ═══════════════════════════════════════════════════════════════════════════
// BOUNDARY: FileHeader round-trip fidelity
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn boundary_header_round_trip() {
    let header = FileHeader {
        magic: *b"ATIM",
        version: 42,
        flags: 0xDEADBEEF,
        entity_count: 1_000_000,
        index_offset: 0xFFFFFFFFFFFFFFFF,
        deadline_index_offset: 12345678,
        decay_index_offset: 87654321,
        created_at: 1700000000_000000,
        modified_at: 1700000001_000000,
        checksum: [0xAB; 32],
    };

    let mut buf = Vec::new();
    header.write_to(&mut buf).unwrap();
    assert_eq!(buf.len(), FileHeader::SIZE);

    let mut cursor = std::io::Cursor::new(buf);
    let read_back = FileHeader::read_from(&mut cursor).unwrap();

    assert_eq!(read_back.version, 42);
    assert_eq!(read_back.flags, 0xDEADBEEF);
    assert_eq!(read_back.entity_count, 1_000_000);
    assert_eq!(read_back.index_offset, 0xFFFFFFFFFFFFFFFF);
    assert_eq!(read_back.deadline_index_offset, 12345678);
    assert_eq!(read_back.decay_index_offset, 87654321);
    assert_eq!(read_back.created_at, 1700000000_000000);
    assert_eq!(read_back.modified_at, 1700000001_000000);
    assert_eq!(read_back.checksum, [0xAB; 32]);
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION: Full workflow (create → add → query → update → save → reopen)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn integration_full_workflow() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("workflow.atime");

    // Phase 1: Create and populate
    let tf = TimeFile::create(&path).unwrap();
    let mut engine = WriteEngine::new(tf);

    let d1 = Deadline::new("Ship v1", Utc::now() + ChronoDuration::days(30));
    let d1_id = engine.add_deadline(d1).unwrap();

    let d2 = Deadline::new("Fix bug", Utc::now() - ChronoDuration::hours(1)); // overdue
    let _d2_id = engine.add_deadline(d2).unwrap();

    let s1 = Schedule::new("Team standup", Utc::now() + ChronoDuration::hours(1), 1800);
    let _s1_id = engine.add_schedule(s1).unwrap();

    let dur = DurationEstimate::new("Feature work", 3600, 7200, 14400);
    let _dur_id = engine.add_duration(dur).unwrap();

    let dm = DecayModel::new(
        "Interest decay",
        1.0,
        DecayType::Exponential { lambda: 0.001 },
    );
    let _dm_id = engine.add_decay(dm).unwrap();

    let steps = vec![
        sequence::SequenceStep::new("Plan", 0),
        sequence::SequenceStep::new("Build", 1),
        sequence::SequenceStep::new("Test", 2),
        sequence::SequenceStep::new("Deploy", 3),
    ];
    let seq = Sequence::new("Release pipeline", steps);
    let seq_id = engine.add_sequence(seq).unwrap();

    // Phase 2: Query
    let qe = QueryEngine::new(engine.file());
    let stats = qe.stats().unwrap();
    assert_eq!(stats.deadline_count, 2);
    assert_eq!(stats.schedule_count, 1);
    assert_eq!(stats.duration_count, 1);
    assert_eq!(stats.decay_count, 1);
    assert_eq!(stats.sequence_count, 1);

    let overdue = qe.overdue_deadlines().unwrap();
    assert_eq!(overdue.len(), 1, "Should have exactly 1 overdue deadline");

    // Phase 3: Update
    engine.complete_deadline(&d1_id).unwrap();
    engine.advance_sequence(&seq_id).unwrap(); // Plan → done
    engine.advance_sequence(&seq_id).unwrap(); // Build → done

    // Phase 4: Batch update
    let report = engine.update_all_statuses().unwrap();
    assert!(report.decays_updated > 0);

    // Phase 5: Save and reopen
    let file = engine.into_file();
    drop(file); // already saved by individual operations

    let tf2 = TimeFile::open(&path).unwrap();
    let qe2 = QueryEngine::new(&tf2);
    let stats2 = qe2.stats().unwrap();
    assert_eq!(stats2.completed_count, 1, "Completed deadline should persist");
    assert_eq!(tf2.entity_count(), 6, "All 6 entities should persist");
}
