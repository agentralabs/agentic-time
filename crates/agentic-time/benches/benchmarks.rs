//! Criterion benchmarks for AgenticTime.
//!
//! Measures latency for all core operations: file I/O, entity CRUD,
//! query engine, write engine, decay calculation, and conflict detection.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use agentic_time::*;
use chrono::{Duration as ChronoDuration, Utc};
use rand::Rng;
use tempfile::tempdir;

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Build a TimeFile with `n` deadlines (no disk flush per entity — bulk add).
fn make_deadline_file(n: usize) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("bench.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    let now = Utc::now();
    for i in 0..n {
        let d = Deadline::new(
            format!("deadline-{}", i),
            now + ChronoDuration::hours(i as i64 + 1),
        );
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    tf.save().unwrap();
    (dir, path)
}

/// Build a TimeFile with mixed entity types.
fn make_mixed_file(n_per_type: usize) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("bench_mixed.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    let now = Utc::now();
    let mut rng = rand::thread_rng();

    // Deadlines
    for i in 0..n_per_type {
        let d = Deadline::new(
            format!("deadline-{}", i),
            now + ChronoDuration::hours(rng.gen_range(1..720)),
        );
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }

    // Schedules
    for i in 0..n_per_type {
        let offset = rng.gen_range(0..720);
        let s = Schedule::new(
            format!("schedule-{}", i),
            now + ChronoDuration::hours(offset),
            rng.gen_range(1800..7200),
        );
        tf.add(EntityType::Schedule, s.id, &s).unwrap();
    }

    // Decay models
    for i in 0..n_per_type {
        let decay_type = match i % 3 {
            0 => DecayType::Linear { rate: 0.001 },
            1 => DecayType::Exponential {
                lambda: 0.0001 + rng.gen::<f64>() * 0.001,
            },
            _ => DecayType::HalfLife {
                half_life_secs: 3600 * rng.gen_range(1..168),
            },
        };
        let d = DecayModel::new(format!("decay-{}", i), 1.0, decay_type);
        tf.add(EntityType::Decay, d.id, &d).unwrap();
    }

    // Duration estimates
    for i in 0..n_per_type {
        let opt = rng.gen_range(60..3600);
        let exp = opt + rng.gen_range(0..3600);
        let pess = exp + rng.gen_range(0..7200);
        let d = DurationEstimate::new(format!("duration-{}", i), opt, exp, pess);
        tf.add(EntityType::Duration, d.id, &d).unwrap();
    }

    // Sequences (5 steps each)
    for i in 0..n_per_type {
        let steps: Vec<SequenceStep> = (0..5)
            .map(|j| {
                let mut s = SequenceStep::new(format!("step-{}-{}", i, j), j as u32);
                s.duration_secs = Some(rng.gen_range(30..600));
                if j > 0 {
                    s.depends_on.push((j - 1) as u32);
                }
                s
            })
            .collect();
        let seq = Sequence::new(format!("sequence-{}", i), steps);
        tf.add(EntityType::Sequence, seq.id, &seq).unwrap();
    }

    tf.save().unwrap();
    (dir, path)
}

// ── File I/O Benchmarks ─────────────────────────────────────────────────────

fn bench_create_file(c: &mut Criterion) {
    c.bench_function("file_create_empty", |b| {
        let dir = tempdir().unwrap();
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let path = dir.path().join(format!("create_{}.atime", counter));
            let tf = TimeFile::create(&path).unwrap();
            black_box(tf);
        });
    });
}

fn bench_open_file_100(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(100);
    c.bench_function("file_open_100_entities", |b| {
        b.iter(|| {
            let tf = TimeFile::open(black_box(&path)).unwrap();
            black_box(tf);
        });
    });
}

fn bench_open_file_1k(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(1_000);
    c.bench_function("file_open_1k_entities", |b| {
        b.iter(|| {
            let tf = TimeFile::open(black_box(&path)).unwrap();
            black_box(tf);
        });
    });
}

fn bench_open_file_10k(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(10_000);
    c.bench_function("file_open_10k_entities", |b| {
        b.iter(|| {
            let tf = TimeFile::open(black_box(&path)).unwrap();
            black_box(tf);
        });
    });
}

fn bench_save_file_100(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(100);
    let tf = TimeFile::open(&path).unwrap();
    c.bench_function("file_save_100_entities", |b| {
        b.iter(|| {
            tf.save().unwrap();
        });
    });
}

fn bench_save_file_1k(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(1_000);
    let tf = TimeFile::open(&path).unwrap();
    c.bench_function("file_save_1k_entities", |b| {
        b.iter(|| {
            tf.save().unwrap();
        });
    });
}

fn bench_save_file_10k(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(10_000);
    let tf = TimeFile::open(&path).unwrap();
    c.bench_function("file_save_10k_entities", |b| {
        b.iter(|| {
            tf.save().unwrap();
        });
    });
}

// ── Entity Add Benchmarks ───────────────────────────────────────────────────

fn bench_add_deadline(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("add.atime");
    let mut tf = TimeFile::create(&path).unwrap();
    let now = Utc::now();

    c.bench_function("entity_add_deadline", |b| {
        b.iter(|| {
            let d = Deadline::new("bench", now + ChronoDuration::hours(24));
            tf.add(EntityType::Deadline, d.id, black_box(&d)).unwrap();
        });
    });
}

fn bench_add_schedule(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("add_sched.atime");
    let mut tf = TimeFile::create(&path).unwrap();
    let now = Utc::now();

    c.bench_function("entity_add_schedule", |b| {
        b.iter(|| {
            let s = Schedule::new("bench", now + ChronoDuration::hours(1), 3600);
            tf.add(EntityType::Schedule, s.id, black_box(&s)).unwrap();
        });
    });
}

fn bench_add_decay(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("add_decay.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    c.bench_function("entity_add_decay", |b| {
        b.iter(|| {
            let d = DecayModel::new("bench", 1.0, DecayType::Exponential { lambda: 0.001 });
            tf.add(EntityType::Decay, d.id, black_box(&d)).unwrap();
        });
    });
}

fn bench_add_duration(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("add_dur.atime");
    let mut tf = TimeFile::create(&path).unwrap();

    c.bench_function("entity_add_duration", |b| {
        b.iter(|| {
            let d = DurationEstimate::new("bench", 60, 120, 300);
            tf.add(EntityType::Duration, d.id, black_box(&d)).unwrap();
        });
    });
}

// ── Decay Calculation Benchmarks ────────────────────────────────────────────

fn bench_decay_exponential(c: &mut Criterion) {
    let d = DecayModel::new("bench", 1.0, DecayType::Exponential { lambda: 0.001 });
    let future = d.reference_time + ChronoDuration::hours(24);

    c.bench_function("decay_exponential_calculate", |b| {
        b.iter(|| {
            black_box(d.calculate_value(black_box(future)));
        });
    });
}

fn bench_decay_linear(c: &mut Criterion) {
    let d = DecayModel::new("bench", 100.0, DecayType::Linear { rate: 0.01 });
    let future = d.reference_time + ChronoDuration::hours(24);

    c.bench_function("decay_linear_calculate", |b| {
        b.iter(|| {
            black_box(d.calculate_value(black_box(future)));
        });
    });
}

fn bench_decay_half_life(c: &mut Criterion) {
    let d = DecayModel::new(
        "bench",
        1.0,
        DecayType::HalfLife {
            half_life_secs: 86400,
        },
    );
    let future = d.reference_time + ChronoDuration::hours(48);

    c.bench_function("decay_halflife_calculate", |b| {
        b.iter(|| {
            black_box(d.calculate_value(black_box(future)));
        });
    });
}

fn bench_decay_step(c: &mut Criterion) {
    let d = DecayModel::new(
        "bench",
        100.0,
        DecayType::Step {
            drop_amount: 10.0,
            interval_secs: 3600,
        },
    );
    let future = d.reference_time + ChronoDuration::hours(5);

    c.bench_function("decay_step_calculate", |b| {
        b.iter(|| {
            black_box(d.calculate_value(black_box(future)));
        });
    });
}

fn bench_decay_time_until(c: &mut Criterion) {
    let d = DecayModel::new("bench", 1.0, DecayType::Exponential { lambda: 0.001 });

    c.bench_function("decay_time_until_threshold", |b| {
        b.iter(|| {
            black_box(d.time_until(black_box(0.5)));
        });
    });
}

fn bench_decay_batch_1k(c: &mut Criterion) {
    let models: Vec<DecayModel> = (0..1_000)
        .map(|i| {
            DecayModel::new(
                format!("d-{}", i),
                1.0,
                DecayType::Exponential {
                    lambda: 0.0001 * (i as f64 + 1.0),
                },
            )
        })
        .collect();
    let future = Utc::now() + ChronoDuration::hours(24);

    c.bench_function("decay_batch_calculate_1k", |b| {
        b.iter(|| {
            for model in &models {
                black_box(model.calculate_value(future));
            }
        });
    });
}

// ── Query Engine Benchmarks ─────────────────────────────────────────────────

fn bench_query_overdue_100(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("query.atime");
    let mut tf = TimeFile::create(&path).unwrap();
    let now = Utc::now();

    // 50 future + 50 past (overdue)
    for i in 0..50 {
        let d = Deadline::new(format!("future-{}", i), now + ChronoDuration::hours(i + 1));
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    for i in 0..50 {
        let d = Deadline::new(format!("past-{}", i), now - ChronoDuration::hours(i + 1));
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    tf.save().unwrap();

    let tf = TimeFile::open(&path).unwrap();
    let qe = QueryEngine::new(&tf);

    c.bench_function("query_overdue_deadlines_100", |b| {
        b.iter(|| {
            let r = qe.overdue_deadlines().unwrap();
            black_box(r);
        });
    });
}

fn bench_query_due_within_1k(c: &mut Criterion) {
    let (_dir, path) = make_deadline_file(1_000);
    let tf = TimeFile::open(&path).unwrap();
    let qe = QueryEngine::new(&tf);

    c.bench_function("query_due_within_24h_1k", |b| {
        b.iter(|| {
            let r = qe
                .deadlines_due_within(black_box(ChronoDuration::hours(24)))
                .unwrap();
            black_box(r);
        });
    });
}

fn bench_query_schedule_conflicts_100(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("conflict.atime");
    let mut tf = TimeFile::create(&path).unwrap();
    let now = Utc::now();

    // 100 schedules, each 1 hour long, starting every 30 min (many conflicts)
    for i in 0..100 {
        let s = Schedule::new(
            format!("sched-{}", i),
            now + ChronoDuration::minutes(30 * i),
            3600,
        );
        tf.add(EntityType::Schedule, s.id, &s).unwrap();
    }
    tf.save().unwrap();

    let tf = TimeFile::open(&path).unwrap();
    let qe = QueryEngine::new(&tf);
    let probe = Schedule::new("probe", now + ChronoDuration::hours(2), 3600);

    c.bench_function("query_schedule_conflicts_100", |b| {
        b.iter(|| {
            let r = qe.schedule_conflicts(black_box(&probe)).unwrap();
            black_box(r);
        });
    });
}

fn bench_query_available_slots(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("slots.atime");
    let mut tf = TimeFile::create(&path).unwrap();
    let now = Utc::now();

    // 20 schedules spread over 48 hours
    for i in 0..20 {
        let s = Schedule::new(
            format!("sched-{}", i),
            now + ChronoDuration::hours(i * 2),
            3600,
        );
        tf.add(EntityType::Schedule, s.id, &s).unwrap();
    }
    tf.save().unwrap();

    let tf = TimeFile::open(&path).unwrap();
    let qe = QueryEngine::new(&tf);

    c.bench_function("query_available_slots_48h", |b| {
        b.iter(|| {
            let r = qe
                .available_slots(
                    now,
                    now + ChronoDuration::hours(48),
                    ChronoDuration::minutes(30),
                )
                .unwrap();
            black_box(r);
        });
    });
}

fn bench_query_decays_below_threshold(c: &mut Criterion) {
    let (_dir, path) = make_mixed_file(200);
    let tf = TimeFile::open(&path).unwrap();
    let qe = QueryEngine::new(&tf);

    c.bench_function("query_decays_below_threshold_200", |b| {
        b.iter(|| {
            let r = qe.decays_below_threshold(black_box(0.5)).unwrap();
            black_box(r);
        });
    });
}

fn bench_query_stats_1k(c: &mut Criterion) {
    let (_dir, path) = make_mixed_file(200);
    let tf = TimeFile::open(&path).unwrap();
    let qe = QueryEngine::new(&tf);

    c.bench_function("query_stats_1k_entities", |b| {
        b.iter(|| {
            let r = qe.stats().unwrap();
            black_box(r);
        });
    });
}

// ── Write Engine Benchmarks ─────────────────────────────────────────────────

fn bench_write_add_deadline_and_save(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("write.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut we = WriteEngine::new(tf);
    let now = Utc::now();

    c.bench_function("write_add_deadline_with_save", |b| {
        b.iter(|| {
            let d = Deadline::new("bench", now + ChronoDuration::hours(24));
            we.add_deadline(black_box(d)).unwrap();
        });
    });
}

fn bench_write_add_schedule_conflict_check(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("write_sched.atime");
    let tf = TimeFile::create(&path).unwrap();
    let mut we = WriteEngine::new(tf);
    let now = Utc::now();

    // Pre-populate with 50 schedules
    for i in 0..50 {
        let mut s = Schedule::new(
            format!("existing-{}", i),
            now + ChronoDuration::hours(i * 2),
            3600,
        );
        s.flexible = true;
        we.add_schedule(s).unwrap();
    }

    c.bench_function("write_add_schedule_with_conflict_check_50", |b| {
        let mut counter = 0i64;
        b.iter(|| {
            counter += 1;
            let mut s = Schedule::new(
                format!("new-{}", counter),
                now + ChronoDuration::hours(200 + counter),
                1800,
            );
            s.flexible = true;
            we.add_schedule(black_box(s)).unwrap();
        });
    });
}

fn bench_write_update_all_statuses(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("status.atime");
    let mut tf = TimeFile::create(&path).unwrap();
    let now = Utc::now();

    // 50 deadlines (mix of past and future) + 50 decays
    for i in 0..25 {
        let d = Deadline::new(format!("future-{}", i), now + ChronoDuration::hours(i + 1));
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    for i in 0..25 {
        let d = Deadline::new(format!("past-{}", i), now - ChronoDuration::hours(i + 1));
        tf.add(EntityType::Deadline, d.id, &d).unwrap();
    }
    for i in 0..50 {
        let decay = DecayModel::new(
            format!("decay-{}", i),
            1.0,
            DecayType::Exponential { lambda: 0.001 },
        );
        tf.add(EntityType::Decay, decay.id, &decay).unwrap();
    }
    tf.save().unwrap();

    c.bench_function("write_update_all_statuses_100", |b| {
        b.iter(|| {
            let tf = TimeFile::open(&path).unwrap();
            let mut we = WriteEngine::new(tf);
            let report = we.update_all_statuses().unwrap();
            black_box(report);
        });
    });
}

// ── PERT Calculation Benchmarks ─────────────────────────────────────────────

fn bench_pert_estimate(c: &mut Criterion) {
    let d = DurationEstimate::new("bench", 3600, 7200, 14400);

    c.bench_function("pert_estimate_calculate", |b| {
        b.iter(|| {
            black_box(d.pert_estimate());
            black_box(d.std_deviation());
        });
    });
}

// ── File Size Measurement ───────────────────────────────────────────────────

fn bench_file_size_measurement(c: &mut Criterion) {
    // This benchmark measures open + query + save round-trip at scale
    let (_dir, path) = make_mixed_file(200);

    c.bench_function("round_trip_open_query_save_1k", |b| {
        b.iter(|| {
            let tf = TimeFile::open(&path).unwrap();
            let qe = QueryEngine::new(&tf);
            let _stats = qe.stats().unwrap();
            let _overdue = qe.overdue_deadlines().unwrap();
            tf.save().unwrap();
            black_box(tf.entity_count());
        });
    });
}

// ── Group registration ──────────────────────────────────────────────────────

criterion_group!(
    file_io,
    bench_create_file,
    bench_open_file_100,
    bench_open_file_1k,
    bench_open_file_10k,
    bench_save_file_100,
    bench_save_file_1k,
    bench_save_file_10k,
);

criterion_group!(
    entity_ops,
    bench_add_deadline,
    bench_add_schedule,
    bench_add_decay,
    bench_add_duration,
);

criterion_group!(
    decay_calc,
    bench_decay_exponential,
    bench_decay_linear,
    bench_decay_half_life,
    bench_decay_step,
    bench_decay_time_until,
    bench_decay_batch_1k,
);

criterion_group!(
    queries,
    bench_query_overdue_100,
    bench_query_due_within_1k,
    bench_query_schedule_conflicts_100,
    bench_query_available_slots,
    bench_query_decays_below_threshold,
    bench_query_stats_1k,
);

criterion_group!(
    write_ops,
    bench_write_add_deadline_and_save,
    bench_write_add_schedule_conflict_check,
    bench_write_update_all_statuses,
    bench_pert_estimate,
    bench_file_size_measurement,
);

criterion_main!(file_io, entity_ops, decay_calc, queries, write_ops);
