---
status: stable
---

# API Reference

Complete reference for the AgenticTime Rust and Python APIs.

## Rust API (agentic-time-core)

### TimeGraph

The primary struct for interacting with an AgenticTime temporal graph. Each `TimeGraph` corresponds to a single `.atime` file.

```rust
use agentic_time_core::TimeGraph;

let mut graph = TimeGraph::open("project.atime")?;
```

#### `TimeGraph::open(path: &str) -> Result<TimeGraph>`

Opens an existing time graph or creates a new one at the given path.

#### `TimeGraph::create(path: &str) -> Result<TimeGraph>`

Creates a new empty time graph. Fails if the file already exists.

#### `TimeGraph::save(&self) -> Result<()>`

Persists all in-memory changes to disk.

#### `TimeGraph::stats(&self) -> GraphStats`

Returns statistics: deadline count, schedule count, sequence count, decay config count, total entities.

### Deadline

```rust
use agentic_time_core::{Deadline, Priority, DeadlineStatus};

let deadline = Deadline::new(
    "Ship v1.0",
    "2026-03-15T17:00:00Z".parse()?,
    Priority::High,
);
graph.add_deadline(deadline)?;
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-assigned unique identifier |
| `title` | `String` | Human-readable deadline title |
| `due_at` | `DateTime<Utc>` | UTC deadline timestamp |
| `priority` | `Priority` | `Low`, `Medium`, `High`, `Critical` |
| `status` | `DeadlineStatus` | `Pending`, `InProgress`, `Completed`, `Missed`, `Cancelled` |
| `tags` | `Vec<String>` | User-defined tags |
| `depends_on` | `Vec<u64>` | IDs of prerequisite deadlines |

### Duration

```rust
use agentic_time_core::{DurationEstimate, Confidence};

let estimate = DurationEstimate::new(
    "Auth refactor",
    Duration::from_secs(8 * 3600),
    Confidence::new(0.7),
);
graph.add_duration(estimate)?;
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-assigned unique identifier |
| `label` | `String` | What is being estimated |
| `estimate` | `Duration` | Best-estimate duration |
| `confidence` | `f64` | Confidence level (0.0 to 1.0) |
| `actual` | `Option<Duration>` | Actual elapsed time (if tracked) |
| `started_at` | `Option<DateTime<Utc>>` | When tracking started |
| `completed_at` | `Option<DateTime<Utc>>` | When tracking ended |

### Schedule

```rust
use agentic_time_core::{Schedule, Recurrence};

let schedule = Schedule::new(
    "Weekly code review",
    Recurrence::Weekly { day: Weekday::Tue, hour: 10, minute: 0 },
    Duration::from_secs(2 * 3600),
);
graph.add_schedule(schedule)?;
```

### Sequence

```rust
use agentic_time_core::Sequence;

let mut seq = Sequence::new("Deploy pipeline");
seq.add_step("Run tests", Duration::from_secs(600))?;
seq.add_step("Build artifacts", Duration::from_secs(300))?;
seq.add_step("Deploy to staging", Duration::from_secs(120))?;
seq.add_step("Smoke tests", Duration::from_secs(180))?;
seq.add_step("Deploy to production", Duration::from_secs(60))?;
graph.add_sequence(seq)?;
```

### DecayConfig

```rust
use agentic_time_core::{DecayConfig, DecayCurve};

let config = DecayConfig::new(
    "memory-freshness",
    DecayCurve::Exponential { halflife_hours: 168.0 },
);
graph.set_decay(config)?;
```

## Python API

```bash
pip install agentic-time
```

### TimeGraph

```python
from agentic_time import TimeGraph

tg = TimeGraph("project.atime")
```

#### `add_deadline(title, due_at, priority="medium", tags=None, depends_on=None) -> int`

Adds a deadline. Returns the deadline ID.

#### `add_duration_estimate(label, hours=None, minutes=None, confidence=0.8) -> int`

Adds a duration estimate. Returns the estimate ID.

#### `add_schedule(title, recurrence, duration_minutes, timezone="UTC") -> int`

Adds a schedule entry. Returns the schedule ID.

#### `create_sequence(title) -> int`

Creates a new empty sequence. Returns the sequence ID.

#### `add_sequence_step(sequence_id, label, duration_minutes) -> int`

Adds a step to an existing sequence.

#### `configure_decay(name, curve="exponential", halflife_hours=168) -> None`

Configures a named decay curve.

#### `query_decay(name, age_hours) -> float`

Returns the freshness value (0.0 to 1.0) for a given age.

#### `stats() -> dict`

Returns graph statistics.

#### `save() -> None`

Persists changes to disk.
