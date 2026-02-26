# AgenticTime — Complete Specification (25 Parts)

> **The 5th Sister. Temporal reasoning for AI agents.**
> **"When should this happen?"**

---

# PART 1: CLAUDE-CODE-INSTRUCTIONS

## What You Are Building

AgenticTime is a temporal reasoning system for AI agents. It models:
- **Durations** — How long things take
- **Deadlines** — When things must happen by
- **Schedules** — When things should happen
- **Sequences** — What must happen in order
- **Decay** — What loses value over time

The entire temporal model lives in a single `.atime` file.

## Language & Toolchain

- **Language:** Rust (latest stable)
- **Build system:** Cargo workspace
- **Edition:** 2021
- **MSRV:** 1.75+

## Build Phases

### Phase 1: Foundation
1. Create project structure (Part 2)
2. Implement data structures (Part 4)
3. Implement file format (Part 5)
4. Run Phase 1 tests

### Phase 2: Core Engine
5. Implement write engine (Part 6)
6. Implement query engine (Part 7)
7. Implement indexes (Part 8)
8. Run Phase 2 tests

### Phase 3: MCP Layer
9. Implement MCP tools (Part 9)
10. Implement MCP resources (Part 10)
11. Implement MCP hardening (Part 24)
12. Run Phase 3 tests

### Phase 4: Integration
13. Implement CLI (Part 13)
14. Implement FFI (Part 14)
15. Implement Ghost Writer (Part 23)
16. Run Phase 4 tests

### Phase 5: Documentation
17. Create docs/public/ pages (Part 18)
18. Create assets (Part 19)
19. Write README (Part 20)
20. Set up CI (Part 22)

### Phase 6: Validation
21. Run edge case tests (Part 12)
22. Run guardrails
23. Generate research paper (Part 16)

## Rules

1. **No dependencies not in Part 3** — implement it yourself or find another way
2. **No skipping tests** — every phase has tests, run them
3. **No unsafe Rust** — except where explicitly required
4. **Every public function has doc comments**
5. **No unwrap() in library code** — proper error handling
6. **No println!() in library code** — use tracing
7. **Binary format is little-endian**
8. **All timestamps are Unix epoch microseconds (u64)**
9. **Copy patterns from sisters** — don't reinvent

## Success Criteria

- [ ] `cargo test` — zero failures
- [ ] `cargo clippy` — zero warnings  
- [ ] `cargo build --release` — compiles
- [ ] MCP server works with Claude Desktop
- [ ] Ghost Writer syncs to all clients
- [ ] Guardrails pass (23 + 27 sections)
- [ ] Research paper generated

---

# PART 2: PROJECT-STRUCTURE

```
agentic-time/
├── Cargo.toml                    # Workspace root
├── README.md
├── LICENSE
├── CANONICAL_SISTER_KIT.md
├── sister.manifest.json
│
├── crates/
│   ├── agentic-time/             # Core library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── duration.rs       # Duration modeling
│   │       ├── deadline.rs       # Deadline management
│   │       ├── schedule.rs       # Scheduling
│   │       ├── sequence.rs       # Sequence constraints
│   │       ├── decay.rs          # Decay modeling
│   │       ├── file_format.rs    # .atime format
│   │       ├── write_engine.rs   # Write operations
│   │       ├── query_engine.rs   # Query operations
│   │       ├── indexes/
│   │       │   ├── mod.rs
│   │       │   ├── deadline_index.rs
│   │       │   ├── sequence_index.rs
│   │       │   └── decay_index.rs
│   │       └── error.rs
│   │
│   ├── agentic-time-mcp/         # MCP server
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       ├── server.rs
│   │       ├── tools.rs          # MCP tools
│   │       ├── resources.rs      # MCP resources
│   │       ├── prompts.rs        # MCP prompts
│   │       ├── ghost_bridge.rs   # Ghost Writer
│   │       ├── stdio.rs          # Hardened stdio transport
│   │       └── greeting.rs       # Server greeting
│   │
│   ├── agentic-time-cli/         # CLI binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       └── commands/
│   │           ├── mod.rs
│   │           ├── deadline.rs
│   │           ├── schedule.rs
│   │           ├── sequence.rs
│   │           ├── decay.rs
│   │           ├── info.rs
│   │           └── install.rs
│   │
│   └── agentic-time-ffi/         # FFI bindings
│       ├── Cargo.toml
│       ├── src/
│       │   └── lib.rs
│       └── include/
│           └── agentic_time.h
│
├── docs/
│   └── public/                   # 13 required pages
│       ├── overview.md
│       ├── quickstart.md
│       ├── installation.md
│       ├── command-surface.md
│       ├── experience-with-vs-without.md
│       ├── runtime-install-sync.md
│       ├── integration-guide.md
│       ├── concepts.md
│       ├── api-reference.md
│       ├── file-format.md
│       ├── benchmarks.md
│       ├── faq.md
│       └── playbooks-agent-integration.md
│
├── assets/
│   ├── github-hero-pane.svg
│   ├── architecture.svg
│   └── benchmarks.svg
│
├── scripts/
│   ├── install.sh
│   ├── check-canonical-sister.sh
│   └── check-install-commands.sh
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       ├── canonical-sister-guardrails.yml
│       └── install-command-guardrails.yml
│
├── python/                       # Python bindings
│   ├── pyproject.toml
│   └── src/
│       └── agentic_time/
│           └── __init__.py
│
├── wasm/                         # WASM bindings
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
│
└── tests/
    ├── integration/
    └── edge_cases/
```

---

# PART 3: DEPENDENCIES

## Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/agentic-time",
    "crates/agentic-time-mcp",
    "crates/agentic-time-cli",
    "crates/agentic-time-ffi",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"
repository = "https://github.com/agentralabs/agentic-time"
authors = ["Agentralabs"]

[workspace.dependencies]
# Core
chrono = { version = "0.4", features = ["serde"] }
blake3 = "1.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"

# Async
tokio = { version = "1.36", features = ["full"] }

# MCP
uuid = { version = "1.7", features = ["v4", "serde"] }

# CLI
clap = { version = "4.5", features = ["derive"] }

# Internal
agentic-time = { path = "crates/agentic-time" }
```

## agentic-time/Cargo.toml

```toml
[package]
name = "agentic-time"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "Temporal reasoning for AI agents"
keywords = ["ai", "agents", "time", "temporal", "scheduling"]
categories = ["data-structures", "date-and-time"]

[dependencies]
chrono = { workspace = true }
blake3 = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
uuid = { workspace = true }

[dev-dependencies]
tempfile = "3.10"
```

## agentic-time-mcp/Cargo.toml

```toml
[package]
name = "agentic-time-mcp"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "MCP server for AgenticTime"

[[bin]]
name = "agentic-time-mcp"
path = "src/main.rs"

[dependencies]
agentic-time = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { workspace = true }
chrono = { workspace = true }

[dev-dependencies]
tempfile = "3.10"
```

## agentic-time-cli/Cargo.toml

```toml
[package]
name = "agentic-time-cli"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "CLI for AgenticTime"

[[bin]]
name = "atime"
path = "src/main.rs"

[dependencies]
agentic-time = { workspace = true }
clap = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { workspace = true }
```

## agentic-time-ffi/Cargo.toml

```toml
[package]
name = "agentic-time-ffi"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "FFI bindings for AgenticTime"

[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
agentic-time = { workspace = true }
libc = "0.2"
```

---

# PART 4: DATA-STRUCTURES

## Core Types

```rust
// crates/agentic-time/src/lib.rs

use chrono::{DateTime, Utc, Duration as ChronoDuration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for temporal entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemporalId(pub Uuid);

impl TemporalId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// A duration estimate with uncertainty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationEstimate {
    /// Unique identifier
    pub id: TemporalId,
    
    /// What this duration is for
    pub label: String,
    
    /// Optimistic estimate (best case)
    pub optimistic: ChronoDuration,
    
    /// Most likely estimate
    pub expected: ChronoDuration,
    
    /// Pessimistic estimate (worst case)
    pub pessimistic: ChronoDuration,
    
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    
    /// Source of this estimate
    pub source: DurationSource,
    
    /// When this estimate was created
    pub created_at: DateTime<Utc>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurationSource {
    /// User provided the estimate
    UserEstimate,
    /// Derived from historical data
    Historical { sample_count: u32 },
    /// AI predicted
    Predicted { model: String, confidence: f64 },
    /// Default/fallback
    Default,
}

impl DurationEstimate {
    /// Calculate PERT estimate: (O + 4M + P) / 6
    pub fn pert_estimate(&self) -> ChronoDuration {
        let o = self.optimistic.num_seconds();
        let m = self.expected.num_seconds();
        let p = self.pessimistic.num_seconds();
        ChronoDuration::seconds((o + 4 * m + p) / 6)
    }
    
    /// Calculate standard deviation: (P - O) / 6
    pub fn std_deviation(&self) -> ChronoDuration {
        let o = self.optimistic.num_seconds();
        let p = self.pessimistic.num_seconds();
        ChronoDuration::seconds((p - o) / 6)
    }
}
```

## Deadline

```rust
// crates/agentic-time/src/deadline.rs

use super::*;

/// A deadline with urgency and consequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    /// Unique identifier
    pub id: TemporalId,
    
    /// What this deadline is for
    pub label: String,
    
    /// The deadline timestamp
    pub due_at: DateTime<Utc>,
    
    /// Soft deadline (warning threshold)
    pub warn_at: Option<DateTime<Utc>>,
    
    /// Type of deadline
    pub deadline_type: DeadlineType,
    
    /// What happens if missed
    pub consequence: Consequence,
    
    /// Current status
    pub status: DeadlineStatus,
    
    /// Related temporal entities
    pub dependencies: Vec<TemporalId>,
    
    /// When this deadline was created
    pub created_at: DateTime<Utc>,
    
    /// When this deadline was completed (if completed)
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Additional context
    pub context: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlineType {
    /// Hard deadline - cannot be moved
    Hard,
    /// Soft deadline - can be negotiated
    Soft,
    /// Target - aspirational, not committed
    Target,
    /// Recurring - repeats on schedule
    Recurring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Consequence {
    /// No significant consequence
    None,
    /// Minor inconvenience
    Minor { description: String },
    /// Moderate impact
    Moderate { description: String },
    /// Severe impact
    Severe { description: String },
    /// Critical failure
    Critical { description: String },
    /// Quantified cost
    Quantified { 
        description: String,
        cost_estimate: f64,
        cost_unit: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlineStatus {
    /// Not yet due
    Pending,
    /// Warning threshold reached
    Warning,
    /// Due now or overdue
    Overdue,
    /// Completed on time
    Completed,
    /// Completed late
    CompletedLate,
    /// Cancelled
    Cancelled,
}

impl Deadline {
    /// Calculate time remaining
    pub fn time_remaining(&self) -> Option<ChronoDuration> {
        if self.status == DeadlineStatus::Completed || self.status == DeadlineStatus::Cancelled {
            return None;
        }
        let now = Utc::now();
        if now < self.due_at {
            Some(self.due_at - now)
        } else {
            None // Overdue
        }
    }
    
    /// Calculate overdue time
    pub fn overdue_by(&self) -> Option<ChronoDuration> {
        let now = Utc::now();
        if now > self.due_at && self.status != DeadlineStatus::Completed {
            Some(now - self.due_at)
        } else {
            None
        }
    }
    
    /// Update status based on current time
    pub fn update_status(&mut self) {
        if self.status == DeadlineStatus::Completed || 
           self.status == DeadlineStatus::CompletedLate ||
           self.status == DeadlineStatus::Cancelled {
            return;
        }
        
        let now = Utc::now();
        
        if now > self.due_at {
            self.status = DeadlineStatus::Overdue;
        } else if let Some(warn_at) = self.warn_at {
            if now > warn_at {
                self.status = DeadlineStatus::Warning;
            }
        }
    }
    
    /// Mark as completed
    pub fn complete(&mut self) {
        let now = Utc::now();
        self.completed_at = Some(now);
        
        if now > self.due_at {
            self.status = DeadlineStatus::CompletedLate;
        } else {
            self.status = DeadlineStatus::Completed;
        }
    }
}
```

## Schedule

```rust
// crates/agentic-time/src/schedule.rs

use super::*;

/// A scheduled event or task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// Unique identifier
    pub id: TemporalId,
    
    /// What is scheduled
    pub label: String,
    
    /// When it starts
    pub start_at: DateTime<Utc>,
    
    /// Expected duration
    pub duration: DurationEstimate,
    
    /// Recurrence pattern (if any)
    pub recurrence: Option<Recurrence>,
    
    /// Priority level
    pub priority: Priority,
    
    /// Can this be moved?
    pub flexible: bool,
    
    /// Buffer time before/after
    pub buffer: ScheduleBuffer,
    
    /// Dependencies (must complete before this)
    pub dependencies: Vec<TemporalId>,
    
    /// Blocked by (cannot start until these complete)
    pub blocked_by: Vec<TemporalId>,
    
    /// Status
    pub status: ScheduleStatus,
    
    /// When created
    pub created_at: DateTime<Utc>,
    
    /// Tags
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurrence {
    /// Pattern type
    pub pattern: RecurrencePattern,
    
    /// End condition
    pub ends: RecurrenceEnd,
    
    /// Exceptions (dates to skip)
    pub exceptions: Vec<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrencePattern {
    /// Every N days
    Daily { interval: u32 },
    
    /// Every N weeks on specific days
    Weekly { interval: u32, days: Vec<chrono::Weekday> },
    
    /// Every N months on specific day
    Monthly { interval: u32, day_of_month: u32 },
    
    /// Custom cron expression
    Cron { expression: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrenceEnd {
    /// Never ends
    Never,
    /// Ends after N occurrences
    After { count: u32 },
    /// Ends on specific date
    On { date: DateTime<Utc> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
    Optional,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduleBuffer {
    /// Buffer before event
    pub before: Option<ChronoDuration>,
    /// Buffer after event
    pub after: Option<ChronoDuration>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Rescheduled,
}

impl Schedule {
    /// Calculate end time
    pub fn end_at(&self) -> DateTime<Utc> {
        self.start_at + self.duration.pert_estimate()
    }
    
    /// Check if conflicts with another schedule
    pub fn conflicts_with(&self, other: &Schedule) -> bool {
        let self_end = self.end_at();
        let other_end = other.end_at();
        
        // Check overlap
        self.start_at < other_end && self_end > other.start_at
    }
    
    /// Get next occurrence (for recurring schedules)
    pub fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        // Implementation depends on recurrence pattern
        self.recurrence.as_ref().map(|r| {
            // Calculate next based on pattern
            match &r.pattern {
                RecurrencePattern::Daily { interval } => {
                    let days_since = (after - self.start_at).num_days();
                    let next_interval = ((days_since / *interval as i64) + 1) * *interval as i64;
                    self.start_at + ChronoDuration::days(next_interval)
                }
                _ => after // Placeholder - implement other patterns
            }
        })
    }
}
```

## Sequence

```rust
// crates/agentic-time/src/sequence.rs

use super::*;

/// A sequence of ordered temporal events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    /// Unique identifier
    pub id: TemporalId,
    
    /// Sequence name
    pub label: String,
    
    /// Ordered steps
    pub steps: Vec<SequenceStep>,
    
    /// Current step index
    pub current_step: usize,
    
    /// Overall status
    pub status: SequenceStatus,
    
    /// When sequence started
    pub started_at: Option<DateTime<Utc>>,
    
    /// When sequence completed
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Allow parallel execution?
    pub allow_parallel: bool,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Tags
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStep {
    /// Step identifier
    pub id: TemporalId,
    
    /// Step label
    pub label: String,
    
    /// Order in sequence (0-indexed)
    pub order: u32,
    
    /// Expected duration
    pub duration: Option<DurationEstimate>,
    
    /// Step status
    pub status: StepStatus,
    
    /// Can be done in parallel with next step?
    pub parallel_with_next: bool,
    
    /// Dependencies within sequence
    pub depends_on: Vec<u32>, // Order indices
    
    /// When step started
    pub started_at: Option<DateTime<Utc>>,
    
    /// When step completed
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SequenceStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Ready,      // Dependencies met, can start
    InProgress,
    Completed,
    Skipped,
    Failed,
}

impl Sequence {
    /// Get ready steps (dependencies met, not started)
    pub fn ready_steps(&self) -> Vec<&SequenceStep> {
        self.steps.iter()
            .filter(|s| s.status == StepStatus::Ready || 
                       (s.status == StepStatus::Pending && self.dependencies_met(s)))
            .collect()
    }
    
    /// Check if step dependencies are met
    fn dependencies_met(&self, step: &SequenceStep) -> bool {
        step.depends_on.iter().all(|dep_order| {
            self.steps.iter()
                .find(|s| s.order == *dep_order)
                .map(|s| s.status == StepStatus::Completed)
                .unwrap_or(true)
        })
    }
    
    /// Calculate total expected duration
    pub fn total_duration(&self) -> ChronoDuration {
        if self.allow_parallel {
            // Critical path calculation would go here
            self.steps.iter()
                .filter_map(|s| s.duration.as_ref())
                .map(|d| d.pert_estimate())
                .max()
                .unwrap_or(ChronoDuration::zero())
        } else {
            // Sequential sum
            self.steps.iter()
                .filter_map(|s| s.duration.as_ref())
                .map(|d| d.pert_estimate())
                .fold(ChronoDuration::zero(), |acc, d| acc + d)
        }
    }
    
    /// Advance to next step
    pub fn complete_current_step(&mut self) {
        if let Some(step) = self.steps.get_mut(self.current_step) {
            step.status = StepStatus::Completed;
            step.completed_at = Some(Utc::now());
        }
        
        self.current_step += 1;
        
        if self.current_step >= self.steps.len() {
            self.status = SequenceStatus::Completed;
            self.completed_at = Some(Utc::now());
        }
        
        // Mark next ready steps
        self.update_ready_steps();
    }
    
    fn update_ready_steps(&mut self) {
        for step in &mut self.steps {
            if step.status == StepStatus::Pending && self.dependencies_met_mut(step) {
                step.status = StepStatus::Ready;
            }
        }
    }
    
    fn dependencies_met_mut(&self, step: &SequenceStep) -> bool {
        step.depends_on.iter().all(|dep_order| {
            self.steps.iter()
                .find(|s| s.order == *dep_order)
                .map(|s| s.status == StepStatus::Completed)
                .unwrap_or(true)
        })
    }
}
```

## Decay

```rust
// crates/agentic-time/src/decay.rs

use super::*;

/// Models how value/relevance decays over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayModel {
    /// Unique identifier
    pub id: TemporalId,
    
    /// What this decay applies to
    pub label: String,
    
    /// Initial value (at t=0)
    pub initial_value: f64,
    
    /// Current calculated value
    pub current_value: f64,
    
    /// Reference time (when value was initial_value)
    pub reference_time: DateTime<Utc>,
    
    /// Decay function
    pub decay_type: DecayType,
    
    /// Minimum value (floor)
    pub floor: f64,
    
    /// When last calculated
    pub last_calculated: DateTime<Utc>,
    
    /// Tags
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecayType {
    /// Linear decay: value = initial - (rate * time)
    Linear { 
        /// Units lost per second
        rate: f64 
    },
    
    /// Exponential decay: value = initial * e^(-λt)
    Exponential { 
        /// Decay constant (λ)
        lambda: f64 
    },
    
    /// Half-life decay: value = initial * (0.5)^(t/half_life)
    HalfLife { 
        /// Half-life duration
        half_life: ChronoDuration 
    },
    
    /// Step decay: drops at specific intervals
    Step { 
        /// Drop amount at each interval
        drop_amount: f64,
        /// Interval between drops
        interval: ChronoDuration,
    },
    
    /// Custom decay curve
    Custom {
        /// Points defining the curve (time_offset_secs, value)
        points: Vec<(i64, f64)>,
    },
}

impl DecayModel {
    /// Calculate current value based on elapsed time
    pub fn calculate_value(&self, at: DateTime<Utc>) -> f64 {
        let elapsed_secs = (at - self.reference_time).num_seconds() as f64;
        
        if elapsed_secs < 0.0 {
            return self.initial_value;
        }
        
        let decayed = match &self.decay_type {
            DecayType::Linear { rate } => {
                self.initial_value - (rate * elapsed_secs)
            }
            DecayType::Exponential { lambda } => {
                self.initial_value * (-lambda * elapsed_secs).exp()
            }
            DecayType::HalfLife { half_life } => {
                let half_life_secs = half_life.num_seconds() as f64;
                self.initial_value * (0.5_f64).powf(elapsed_secs / half_life_secs)
            }
            DecayType::Step { drop_amount, interval } => {
                let interval_secs = interval.num_seconds() as f64;
                let intervals = (elapsed_secs / interval_secs).floor();
                self.initial_value - (drop_amount * intervals)
            }
            DecayType::Custom { points } => {
                // Linear interpolation between points
                self.interpolate_custom(elapsed_secs as i64, points)
            }
        };
        
        decayed.max(self.floor)
    }
    
    fn interpolate_custom(&self, elapsed: i64, points: &[(i64, f64)]) -> f64 {
        if points.is_empty() {
            return self.initial_value;
        }
        
        // Find surrounding points
        let mut prev = (0, self.initial_value);
        for &(t, v) in points {
            if t > elapsed {
                // Interpolate between prev and current
                let ratio = (elapsed - prev.0) as f64 / (t - prev.0) as f64;
                return prev.1 + ratio * (v - prev.1);
            }
            prev = (t, v);
        }
        
        // Past all points, return last value
        prev.1
    }
    
    /// Update current_value to now
    pub fn update(&mut self) {
        let now = Utc::now();
        self.current_value = self.calculate_value(now);
        self.last_calculated = now;
    }
    
    /// Time until value reaches threshold
    pub fn time_until(&self, threshold: f64) -> Option<ChronoDuration> {
        if threshold <= self.floor {
            return None; // Will never reach below floor
        }
        
        if self.current_value <= threshold {
            return Some(ChronoDuration::zero()); // Already there
        }
        
        let secs = match &self.decay_type {
            DecayType::Linear { rate } => {
                if *rate <= 0.0 { return None; }
                (self.initial_value - threshold) / rate
            }
            DecayType::Exponential { lambda } => {
                if *lambda <= 0.0 { return None; }
                -(threshold / self.initial_value).ln() / lambda
            }
            DecayType::HalfLife { half_life } => {
                let half_life_secs = half_life.num_seconds() as f64;
                half_life_secs * (threshold / self.initial_value).log2().abs()
            }
            _ => return None, // Complex calculation for Step/Custom
        };
        
        Some(ChronoDuration::seconds(secs as i64))
    }
}
```

## Error Types

```rust
// crates/agentic-time/src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeError {
    #[error("Temporal entity not found: {0}")]
    NotFound(String),
    
    #[error("Invalid time range: start {start} is after end {end}")]
    InvalidRange { start: String, end: String },
    
    #[error("Deadline already passed: {0}")]
    DeadlinePassed(String),
    
    #[error("Schedule conflict: {0} overlaps with {1}")]
    ScheduleConflict(String, String),
    
    #[error("Sequence dependency not met: step {step} depends on {dependency}")]
    DependencyNotMet { step: String, dependency: String },
    
    #[error("Invalid recurrence pattern: {0}")]
    InvalidRecurrence(String),
    
    #[error("File format error: {0}")]
    FileFormat(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type TimeResult<T> = Result<T, TimeError>;
```

---

# PART 5: FILE-FORMAT

## .atime Binary Format

```rust
// crates/agentic-time/src/file_format.rs

use super::*;
use std::io::{Read, Write, Seek, SeekFrom};

/// Magic bytes identifying .atime files
pub const MAGIC: [u8; 4] = *b"ATIM";

/// Current format version
pub const VERSION: u32 = 1;

/// File header (64 bytes)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FileHeader {
    /// Magic bytes "ATIM"
    pub magic: [u8; 4],
    
    /// Format version
    pub version: u32,
    
    /// Flags (reserved)
    pub flags: u32,
    
    /// Number of temporal entities
    pub entity_count: u64,
    
    /// Offset to entity index
    pub index_offset: u64,
    
    /// Offset to deadline index
    pub deadline_index_offset: u64,
    
    /// Offset to decay index
    pub decay_index_offset: u64,
    
    /// File creation timestamp (Unix micros)
    pub created_at: u64,
    
    /// Last modified timestamp (Unix micros)
    pub modified_at: u64,
    
    /// BLAKE3 checksum of content
    pub checksum: [u8; 32],
}

impl FileHeader {
    pub const SIZE: usize = 64;
    
    pub fn new() -> Self {
        let now = Utc::now().timestamp_micros() as u64;
        Self {
            magic: MAGIC,
            version: VERSION,
            flags: 0,
            entity_count: 0,
            index_offset: 0,
            deadline_index_offset: 0,
            decay_index_offset: 0,
            created_at: now,
            modified_at: now,
            checksum: [0; 32],
        }
    }
    
    pub fn write_to<W: Write>(&self, writer: &mut W) -> TimeResult<()> {
        writer.write_all(&self.magic)?;
        writer.write_all(&self.version.to_le_bytes())?;
        writer.write_all(&self.flags.to_le_bytes())?;
        writer.write_all(&self.entity_count.to_le_bytes())?;
        writer.write_all(&self.index_offset.to_le_bytes())?;
        writer.write_all(&self.deadline_index_offset.to_le_bytes())?;
        writer.write_all(&self.decay_index_offset.to_le_bytes())?;
        writer.write_all(&self.created_at.to_le_bytes())?;
        writer.write_all(&self.modified_at.to_le_bytes())?;
        writer.write_all(&self.checksum)?;
        Ok(())
    }
    
    pub fn read_from<R: Read>(reader: &mut R) -> TimeResult<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        
        if magic != MAGIC {
            return Err(TimeError::FileFormat(
                format!("Invalid magic bytes: {:?}", magic)
            ));
        }
        
        let mut buf4 = [0u8; 4];
        let mut buf8 = [0u8; 8];
        let mut checksum = [0u8; 32];
        
        reader.read_exact(&mut buf4)?;
        let version = u32::from_le_bytes(buf4);
        
        reader.read_exact(&mut buf4)?;
        let flags = u32::from_le_bytes(buf4);
        
        reader.read_exact(&mut buf8)?;
        let entity_count = u64::from_le_bytes(buf8);
        
        reader.read_exact(&mut buf8)?;
        let index_offset = u64::from_le_bytes(buf8);
        
        reader.read_exact(&mut buf8)?;
        let deadline_index_offset = u64::from_le_bytes(buf8);
        
        reader.read_exact(&mut buf8)?;
        let decay_index_offset = u64::from_le_bytes(buf8);
        
        reader.read_exact(&mut buf8)?;
        let created_at = u64::from_le_bytes(buf8);
        
        reader.read_exact(&mut buf8)?;
        let modified_at = u64::from_le_bytes(buf8);
        
        reader.read_exact(&mut checksum)?;
        
        Ok(Self {
            magic,
            version,
            flags,
            entity_count,
            index_offset,
            deadline_index_offset,
            decay_index_offset,
            created_at,
            modified_at,
            checksum,
        })
    }
}

/// Entity types stored in file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntityType {
    Duration = 1,
    Deadline = 2,
    Schedule = 3,
    Sequence = 4,
    Decay = 5,
}

impl TryFrom<u8> for EntityType {
    type Error = TimeError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EntityType::Duration),
            2 => Ok(EntityType::Deadline),
            3 => Ok(EntityType::Schedule),
            4 => Ok(EntityType::Sequence),
            5 => Ok(EntityType::Decay),
            _ => Err(TimeError::FileFormat(format!("Unknown entity type: {}", value))),
        }
    }
}

/// Entity block header
#[derive(Debug, Clone)]
pub struct EntityBlock {
    /// Entity type
    pub entity_type: EntityType,
    
    /// Entity ID
    pub id: TemporalId,
    
    /// Payload length
    pub payload_len: u32,
    
    /// Payload (JSON-serialized entity)
    pub payload: Vec<u8>,
}

impl EntityBlock {
    pub fn new<T: Serialize>(entity_type: EntityType, id: TemporalId, entity: &T) -> TimeResult<Self> {
        let payload = serde_json::to_vec(entity)?;
        Ok(Self {
            entity_type,
            id,
            payload_len: payload.len() as u32,
            payload,
        })
    }
    
    pub fn write_to<W: Write>(&self, writer: &mut W) -> TimeResult<()> {
        writer.write_all(&[self.entity_type as u8])?;
        writer.write_all(self.id.0.as_bytes())?;
        writer.write_all(&self.payload_len.to_le_bytes())?;
        writer.write_all(&self.payload)?;
        Ok(())
    }
    
    pub fn read_from<R: Read>(reader: &mut R) -> TimeResult<Self> {
        let mut type_buf = [0u8; 1];
        reader.read_exact(&mut type_buf)?;
        let entity_type = EntityType::try_from(type_buf[0])?;
        
        let mut id_buf = [0u8; 16];
        reader.read_exact(&mut id_buf)?;
        let id = TemporalId(uuid::Uuid::from_bytes(id_buf));
        
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let payload_len = u32::from_le_bytes(len_buf);
        
        let mut payload = vec![0u8; payload_len as usize];
        reader.read_exact(&mut payload)?;
        
        Ok(Self {
            entity_type,
            id,
            payload_len,
            payload,
        })
    }
    
    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> TimeResult<T> {
        Ok(serde_json::from_slice(&self.payload)?)
    }
}

/// The complete .atime file
pub struct TimeFile {
    /// File path
    pub path: std::path::PathBuf,
    
    /// Header
    pub header: FileHeader,
    
    /// All entities (loaded on demand)
    entities: std::collections::HashMap<TemporalId, EntityBlock>,
}

impl TimeFile {
    /// Create new empty file
    pub fn create(path: impl Into<std::path::PathBuf>) -> TimeResult<Self> {
        let path = path.into();
        let header = FileHeader::new();
        
        let file = Self {
            path,
            header,
            entities: std::collections::HashMap::new(),
        };
        
        file.save()?;
        Ok(file)
    }
    
    /// Open existing file
    pub fn open(path: impl Into<std::path::PathBuf>) -> TimeResult<Self> {
        let path = path.into();
        let mut file = std::fs::File::open(&path)?;
        
        let header = FileHeader::read_from(&mut file)?;
        
        // Read all entities
        let mut entities = std::collections::HashMap::new();
        for _ in 0..header.entity_count {
            let block = EntityBlock::read_from(&mut file)?;
            entities.insert(block.id, block);
        }
        
        Ok(Self {
            path,
            header,
            entities,
        })
    }
    
    /// Save to disk
    pub fn save(&self) -> TimeResult<()> {
        let tmp_path = self.path.with_extension("atime.tmp");
        let mut file = std::fs::File::create(&tmp_path)?;
        
        // Update header
        let mut header = self.header.clone();
        header.entity_count = self.entities.len() as u64;
        header.modified_at = Utc::now().timestamp_micros() as u64;
        
        // Write header
        header.write_to(&mut file)?;
        
        // Write entities
        for block in self.entities.values() {
            block.write_to(&mut file)?;
        }
        
        // Atomic rename
        std::fs::rename(&tmp_path, &self.path)?;
        
        Ok(())
    }
    
    /// Add entity
    pub fn add<T: Serialize>(&mut self, entity_type: EntityType, id: TemporalId, entity: &T) -> TimeResult<()> {
        let block = EntityBlock::new(entity_type, id, entity)?;
        self.entities.insert(id, block);
        Ok(())
    }
    
    /// Get entity
    pub fn get<T: for<'de> Deserialize<'de>>(&self, id: &TemporalId) -> TimeResult<Option<T>> {
        match self.entities.get(id) {
            Some(block) => Ok(Some(block.deserialize()?)),
            None => Ok(None),
        }
    }
    
    /// Remove entity
    pub fn remove(&mut self, id: &TemporalId) -> bool {
        self.entities.remove(id).is_some()
    }
    
    /// List all entities of type
    pub fn list_by_type(&self, entity_type: EntityType) -> Vec<&EntityBlock> {
        self.entities.values()
            .filter(|b| b.entity_type == entity_type)
            .collect()
    }
}
```

---

# PART 6: WRITE-ENGINE

```rust
// crates/agentic-time/src/write_engine.rs

use super::*;

/// Write engine for temporal operations
pub struct WriteEngine {
    file: TimeFile,
}

impl WriteEngine {
    pub fn new(file: TimeFile) -> Self {
        Self { file }
    }
    
    pub fn into_file(self) -> TimeFile {
        self.file
    }
    
    // --- Durations ---
    
    pub fn add_duration(&mut self, duration: DurationEstimate) -> TimeResult<TemporalId> {
        let id = duration.id;
        self.file.add(EntityType::Duration, id, &duration)?;
        self.file.save()?;
        Ok(id)
    }
    
    pub fn update_duration(&mut self, duration: DurationEstimate) -> TimeResult<()> {
        self.file.add(EntityType::Duration, duration.id, &duration)?;
        self.file.save()?;
        Ok(())
    }
    
    // --- Deadlines ---
    
    pub fn add_deadline(&mut self, deadline: Deadline) -> TimeResult<TemporalId> {
        let id = deadline.id;
        self.file.add(EntityType::Deadline, id, &deadline)?;
        self.file.save()?;
        Ok(id)
    }
    
    pub fn update_deadline(&mut self, deadline: Deadline) -> TimeResult<()> {
        self.file.add(EntityType::Deadline, deadline.id, &deadline)?;
        self.file.save()?;
        Ok(())
    }
    
    pub fn complete_deadline(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut deadline: Deadline = self.file.get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        deadline.complete();
        self.update_deadline(deadline)
    }
    
    pub fn cancel_deadline(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut deadline: Deadline = self.file.get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        deadline.status = DeadlineStatus::Cancelled;
        self.update_deadline(deadline)
    }
    
    // --- Schedules ---
    
    pub fn add_schedule(&mut self, schedule: Schedule) -> TimeResult<TemporalId> {
        // Check for conflicts
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
    
    pub fn update_schedule(&mut self, schedule: Schedule) -> TimeResult<()> {
        self.file.add(EntityType::Schedule, schedule.id, &schedule)?;
        self.file.save()?;
        Ok(())
    }
    
    pub fn reschedule(&mut self, id: &TemporalId, new_start: DateTime<Utc>) -> TimeResult<()> {
        let mut schedule: Schedule = self.file.get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        schedule.start_at = new_start;
        schedule.status = ScheduleStatus::Rescheduled;
        self.update_schedule(schedule)
    }
    
    fn check_schedule_conflicts(&self, schedule: &Schedule) -> TimeResult<Vec<Schedule>> {
        let mut conflicts = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Schedule) {
            let existing: Schedule = block.deserialize()?;
            if existing.id != schedule.id && 
               existing.status == ScheduleStatus::Scheduled &&
               schedule.conflicts_with(&existing) {
                conflicts.push(existing);
            }
        }
        
        Ok(conflicts)
    }
    
    // --- Sequences ---
    
    pub fn add_sequence(&mut self, sequence: Sequence) -> TimeResult<TemporalId> {
        let id = sequence.id;
        self.file.add(EntityType::Sequence, id, &sequence)?;
        self.file.save()?;
        Ok(id)
    }
    
    pub fn update_sequence(&mut self, sequence: Sequence) -> TimeResult<()> {
        self.file.add(EntityType::Sequence, sequence.id, &sequence)?;
        self.file.save()?;
        Ok(())
    }
    
    pub fn advance_sequence(&mut self, id: &TemporalId) -> TimeResult<()> {
        let mut sequence: Sequence = self.file.get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        sequence.complete_current_step();
        self.update_sequence(sequence)
    }
    
    // --- Decay ---
    
    pub fn add_decay(&mut self, decay: DecayModel) -> TimeResult<TemporalId> {
        let id = decay.id;
        self.file.add(EntityType::Decay, id, &decay)?;
        self.file.save()?;
        Ok(id)
    }
    
    pub fn update_decay(&mut self, decay: DecayModel) -> TimeResult<()> {
        self.file.add(EntityType::Decay, decay.id, &decay)?;
        self.file.save()?;
        Ok(())
    }
    
    pub fn refresh_decay(&mut self, id: &TemporalId) -> TimeResult<f64> {
        let mut decay: DecayModel = self.file.get(id)?
            .ok_or_else(|| TimeError::NotFound(id.0.to_string()))?;
        decay.update();
        let value = decay.current_value;
        self.update_decay(decay)?;
        Ok(value)
    }
    
    // --- Batch operations ---
    
    pub fn update_all_statuses(&mut self) -> TimeResult<UpdateReport> {
        let mut report = UpdateReport::default();
        
        // Update deadlines
        for block in self.file.list_by_type(EntityType::Deadline) {
            let mut deadline: Deadline = block.deserialize()?;
            let old_status = deadline.status;
            deadline.update_status();
            if deadline.status != old_status {
                self.file.add(EntityType::Deadline, deadline.id, &deadline)?;
                report.deadlines_updated += 1;
                if deadline.status == DeadlineStatus::Overdue {
                    report.new_overdue.push(deadline.id);
                }
            }
        }
        
        // Update decay models
        for block in self.file.list_by_type(EntityType::Decay) {
            let mut decay: DecayModel = block.deserialize()?;
            decay.update();
            self.file.add(EntityType::Decay, decay.id, &decay)?;
            report.decays_updated += 1;
        }
        
        self.file.save()?;
        Ok(report)
    }
}

#[derive(Debug, Default)]
pub struct UpdateReport {
    pub deadlines_updated: usize,
    pub decays_updated: usize,
    pub new_overdue: Vec<TemporalId>,
}
```

---

# PART 7: QUERY-ENGINE

```rust
// crates/agentic-time/src/query_engine.rs

use super::*;

/// Query engine for temporal data
pub struct QueryEngine<'a> {
    file: &'a TimeFile,
}

impl<'a> QueryEngine<'a> {
    pub fn new(file: &'a TimeFile) -> Self {
        Self { file }
    }
    
    // --- Deadline queries ---
    
    /// Get all overdue deadlines
    pub fn overdue_deadlines(&self) -> TimeResult<Vec<Deadline>> {
        let now = Utc::now();
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            if deadline.due_at < now && 
               deadline.status != DeadlineStatus::Completed &&
               deadline.status != DeadlineStatus::Cancelled {
                results.push(deadline);
            }
        }
        
        results.sort_by_key(|d| d.due_at);
        Ok(results)
    }
    
    /// Get deadlines due within duration
    pub fn deadlines_due_within(&self, duration: ChronoDuration) -> TimeResult<Vec<Deadline>> {
        let now = Utc::now();
        let cutoff = now + duration;
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            if deadline.due_at > now && 
               deadline.due_at <= cutoff &&
               deadline.status == DeadlineStatus::Pending {
                results.push(deadline);
            }
        }
        
        results.sort_by_key(|d| d.due_at);
        Ok(results)
    }
    
    /// Get deadlines by status
    pub fn deadlines_by_status(&self, status: DeadlineStatus) -> TimeResult<Vec<Deadline>> {
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            if deadline.status == status {
                results.push(deadline);
            }
        }
        
        Ok(results)
    }
    
    // --- Schedule queries ---
    
    /// Get schedule for time range
    pub fn schedules_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> TimeResult<Vec<Schedule>> {
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Schedule) {
            let schedule: Schedule = block.deserialize()?;
            let schedule_end = schedule.end_at();
            
            // Check if overlaps with range
            if schedule.start_at < end && schedule_end > start {
                results.push(schedule);
            }
        }
        
        results.sort_by_key(|s| s.start_at);
        Ok(results)
    }
    
    /// Get conflicting schedules
    pub fn schedule_conflicts(&self, schedule: &Schedule) -> TimeResult<Vec<Schedule>> {
        let mut conflicts = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Schedule) {
            let existing: Schedule = block.deserialize()?;
            if existing.id != schedule.id && schedule.conflicts_with(&existing) {
                conflicts.push(existing);
            }
        }
        
        Ok(conflicts)
    }
    
    /// Get available slots in time range
    pub fn available_slots(&self, start: DateTime<Utc>, end: DateTime<Utc>, min_duration: ChronoDuration) -> TimeResult<Vec<TimeSlot>> {
        let scheduled = self.schedules_in_range(start, end)?;
        let mut slots = Vec::new();
        let mut current = start;
        
        for schedule in scheduled {
            if schedule.start_at > current {
                let gap = schedule.start_at - current;
                if gap >= min_duration {
                    slots.push(TimeSlot {
                        start: current,
                        end: schedule.start_at,
                        duration: gap,
                    });
                }
            }
            current = schedule.end_at().max(current);
        }
        
        // Check remaining time after last schedule
        if current < end {
            let gap = end - current;
            if gap >= min_duration {
                slots.push(TimeSlot {
                    start: current,
                    end,
                    duration: gap,
                });
            }
        }
        
        Ok(slots)
    }
    
    // --- Sequence queries ---
    
    /// Get sequence by ID
    pub fn sequence(&self, id: &TemporalId) -> TimeResult<Option<Sequence>> {
        self.file.get(id)
    }
    
    /// Get active sequences
    pub fn active_sequences(&self) -> TimeResult<Vec<Sequence>> {
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Sequence) {
            let sequence: Sequence = block.deserialize()?;
            if sequence.status == SequenceStatus::InProgress {
                results.push(sequence);
            }
        }
        
        Ok(results)
    }
    
    /// Get blocked sequences (waiting on dependencies)
    pub fn blocked_sequences(&self) -> TimeResult<Vec<(Sequence, Vec<TemporalId>)>> {
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Sequence) {
            let sequence: Sequence = block.deserialize()?;
            let blocked: Vec<_> = sequence.steps.iter()
                .filter(|s| s.status == StepStatus::Pending)
                .flat_map(|s| &s.depends_on)
                .filter(|&dep_order| {
                    sequence.steps.iter()
                        .find(|s| s.order == *dep_order)
                        .map(|s| s.status != StepStatus::Completed)
                        .unwrap_or(false)
                })
                .map(|_| sequence.id)
                .collect();
            
            if !blocked.is_empty() {
                results.push((sequence, blocked));
            }
        }
        
        Ok(results)
    }
    
    // --- Decay queries ---
    
    /// Get decay models below threshold
    pub fn decays_below_threshold(&self, threshold: f64) -> TimeResult<Vec<DecayModel>> {
        let mut results = Vec::new();
        let now = Utc::now();
        
        for block in self.file.list_by_type(EntityType::Decay) {
            let decay: DecayModel = block.deserialize()?;
            let current_value = decay.calculate_value(now);
            if current_value < threshold {
                results.push(decay);
            }
        }
        
        Ok(results)
    }
    
    /// Get decays that will reach threshold within duration
    pub fn decays_reaching_threshold(&self, threshold: f64, within: ChronoDuration) -> TimeResult<Vec<(DecayModel, ChronoDuration)>> {
        let mut results = Vec::new();
        
        for block in self.file.list_by_type(EntityType::Decay) {
            let decay: DecayModel = block.deserialize()?;
            if let Some(time_until) = decay.time_until(threshold) {
                if time_until <= within {
                    results.push((decay, time_until));
                }
            }
        }
        
        results.sort_by_key(|(_, d)| *d);
        Ok(results)
    }
    
    // --- Duration queries ---
    
    /// Estimate total duration for tasks
    pub fn estimate_total(&self, ids: &[TemporalId]) -> TimeResult<DurationEstimate> {
        let mut optimistic = ChronoDuration::zero();
        let mut expected = ChronoDuration::zero();
        let mut pessimistic = ChronoDuration::zero();
        
        for id in ids {
            if let Some(duration) = self.file.get::<DurationEstimate>(id)? {
                optimistic = optimistic + duration.optimistic;
                expected = expected + duration.expected;
                pessimistic = pessimistic + duration.pessimistic;
            }
        }
        
        Ok(DurationEstimate {
            id: TemporalId::new(),
            label: "Combined estimate".to_string(),
            optimistic,
            expected,
            pessimistic,
            confidence: 0.5, // Combined estimates have lower confidence
            source: DurationSource::Predicted {
                model: "aggregation".to_string(),
                confidence: 0.5,
            },
            created_at: Utc::now(),
            tags: vec![],
        })
    }
    
    // --- Stats ---
    
    pub fn stats(&self) -> TimeResult<TimeStats> {
        let mut stats = TimeStats::default();
        
        for block in self.file.list_by_type(EntityType::Duration) {
            stats.duration_count += 1;
        }
        
        for block in self.file.list_by_type(EntityType::Deadline) {
            let deadline: Deadline = block.deserialize()?;
            stats.deadline_count += 1;
            match deadline.status {
                DeadlineStatus::Overdue => stats.overdue_count += 1,
                DeadlineStatus::Completed => stats.completed_count += 1,
                DeadlineStatus::Warning => stats.warning_count += 1,
                _ => stats.pending_count += 1,
            }
        }
        
        for block in self.file.list_by_type(EntityType::Schedule) {
            stats.schedule_count += 1;
        }
        
        for block in self.file.list_by_type(EntityType::Sequence) {
            stats.sequence_count += 1;
        }
        
        for block in self.file.list_by_type(EntityType::Decay) {
            stats.decay_count += 1;
        }
        
        Ok(stats)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub duration: ChronoDuration,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeStats {
    pub duration_count: usize,
    pub deadline_count: usize,
    pub schedule_count: usize,
    pub sequence_count: usize,
    pub decay_count: usize,
    pub overdue_count: usize,
    pub warning_count: usize,
    pub pending_count: usize,
    pub completed_count: usize,
}
```

---

# PART 8: INDEXES

```rust
// crates/agentic-time/src/indexes/mod.rs

mod deadline_index;
mod sequence_index;
mod decay_index;

pub use deadline_index::DeadlineIndex;
pub use sequence_index::SequenceIndex;
pub use decay_index::DecayIndex;

use super::*;
use std::collections::BTreeMap;

/// Deadline index for fast temporal queries
// crates/agentic-time/src/indexes/deadline_index.rs

pub struct DeadlineIndex {
    /// Deadlines sorted by due date
    by_due_date: BTreeMap<i64, Vec<TemporalId>>,
    
    /// Deadlines by status
    by_status: std::collections::HashMap<DeadlineStatus, Vec<TemporalId>>,
}

impl DeadlineIndex {
    pub fn new() -> Self {
        Self {
            by_due_date: BTreeMap::new(),
            by_status: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, deadline: &Deadline) {
        // Index by due date (Unix timestamp)
        let timestamp = deadline.due_at.timestamp();
        self.by_due_date
            .entry(timestamp)
            .or_default()
            .push(deadline.id);
        
        // Index by status
        self.by_status
            .entry(deadline.status)
            .or_default()
            .push(deadline.id);
    }
    
    pub fn remove(&mut self, deadline: &Deadline) {
        let timestamp = deadline.due_at.timestamp();
        if let Some(ids) = self.by_due_date.get_mut(&timestamp) {
            ids.retain(|id| *id != deadline.id);
        }
        
        if let Some(ids) = self.by_status.get_mut(&deadline.status) {
            ids.retain(|id| *id != deadline.id);
        }
    }
    
    /// Get deadlines due before timestamp
    pub fn due_before(&self, before: i64) -> Vec<TemporalId> {
        self.by_due_date
            .range(..before)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }
    
    /// Get deadlines due in range
    pub fn due_in_range(&self, start: i64, end: i64) -> Vec<TemporalId> {
        self.by_due_date
            .range(start..end)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }
    
    /// Get next deadline
    pub fn next_deadline(&self, after: i64) -> Option<TemporalId> {
        self.by_due_date
            .range(after..)
            .next()
            .and_then(|(_, ids)| ids.first().copied())
    }
}

// crates/agentic-time/src/indexes/sequence_index.rs

pub struct SequenceIndex {
    /// Sequences by status
    by_status: std::collections::HashMap<SequenceStatus, Vec<TemporalId>>,
    
    /// Active step lookup
    active_steps: std::collections::HashMap<TemporalId, usize>,
}

impl SequenceIndex {
    pub fn new() -> Self {
        Self {
            by_status: std::collections::HashMap::new(),
            active_steps: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, sequence: &Sequence) {
        self.by_status
            .entry(sequence.status)
            .or_default()
            .push(sequence.id);
        
        if sequence.status == SequenceStatus::InProgress {
            self.active_steps.insert(sequence.id, sequence.current_step);
        }
    }
    
    pub fn update(&mut self, sequence: &Sequence) {
        // Remove from old status buckets (simplified - full impl would track old status)
        for ids in self.by_status.values_mut() {
            ids.retain(|id| *id != sequence.id);
        }
        
        // Re-insert
        self.insert(sequence);
    }
    
    pub fn in_progress(&self) -> &[TemporalId] {
        self.by_status
            .get(&SequenceStatus::InProgress)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

// crates/agentic-time/src/indexes/decay_index.rs

pub struct DecayIndex {
    /// Decays sorted by current value (ascending)
    by_value: BTreeMap<i64, Vec<TemporalId>>, // value * 1000 as i64 for ordering
    
    /// Decays by tag
    by_tag: std::collections::HashMap<String, Vec<TemporalId>>,
}

impl DecayIndex {
    pub fn new() -> Self {
        Self {
            by_value: BTreeMap::new(),
            by_tag: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, decay: &DecayModel) {
        let value_key = (decay.current_value * 1000.0) as i64;
        self.by_value
            .entry(value_key)
            .or_default()
            .push(decay.id);
        
        for tag in &decay.tags {
            self.by_tag
                .entry(tag.clone())
                .or_default()
                .push(decay.id);
        }
    }
    
    /// Get decays below threshold
    pub fn below_threshold(&self, threshold: f64) -> Vec<TemporalId> {
        let threshold_key = (threshold * 1000.0) as i64;
        self.by_value
            .range(..threshold_key)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }
    
    /// Get lowest value decays
    pub fn lowest(&self, n: usize) -> Vec<TemporalId> {
        self.by_value
            .iter()
            .flat_map(|(_, ids)| ids.iter().copied())
            .take(n)
            .collect()
    }
}
```

---

# PART 9: MCP-TOOLS

```rust
// crates/agentic-time-mcp/src/tools.rs

use agentic_time::*;
use serde_json::{json, Value};

/// MCP Tool definitions for AgenticTime
pub const TOOLS: &[ToolDefinition] = &[
    // --- Deadline tools ---
    ToolDefinition {
        name: "time_deadline_add",
        description: "Add a new deadline",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "label": { "type": "string", "description": "What this deadline is for" },
                "due_at": { "type": "string", "format": "date-time", "description": "When the deadline is due (ISO 8601)" },
                "warn_at": { "type": "string", "format": "date-time", "description": "Optional warning threshold" },
                "deadline_type": { "type": "string", "enum": ["hard", "soft", "target", "recurring"], "default": "soft" },
                "consequence": { "type": "string", "description": "What happens if missed" },
                "tags": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["label", "due_at"]
        }"#,
    },
    ToolDefinition {
        name: "time_deadline_complete",
        description: "Mark a deadline as completed",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Deadline ID" }
            },
            "required": ["id"]
        }"#,
    },
    ToolDefinition {
        name: "time_deadline_list",
        description: "List deadlines with optional filters",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "status": { "type": "string", "enum": ["pending", "warning", "overdue", "completed", "cancelled"] },
                "due_within": { "type": "string", "description": "ISO 8601 duration (e.g., P7D for 7 days)" },
                "tags": { "type": "array", "items": { "type": "string" } }
            }
        }"#,
    },
    ToolDefinition {
        name: "time_deadline_overdue",
        description: "Get all overdue deadlines",
        input_schema: r#"{ "type": "object", "properties": {} }"#,
    },
    
    // --- Schedule tools ---
    ToolDefinition {
        name: "time_schedule_create",
        description: "Create a new scheduled event",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "label": { "type": "string", "description": "Event name" },
                "start_at": { "type": "string", "format": "date-time", "description": "Start time (ISO 8601)" },
                "duration_minutes": { "type": "integer", "description": "Expected duration in minutes" },
                "priority": { "type": "string", "enum": ["critical", "high", "medium", "low", "optional"], "default": "medium" },
                "flexible": { "type": "boolean", "default": true, "description": "Can this be moved?" },
                "recurrence": { "type": "string", "description": "Recurrence pattern (daily, weekly, monthly, or cron)" },
                "tags": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["label", "start_at", "duration_minutes"]
        }"#,
    },
    ToolDefinition {
        name: "time_schedule_reschedule",
        description: "Move a scheduled event to a new time",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Schedule ID" },
                "new_start_at": { "type": "string", "format": "date-time", "description": "New start time" }
            },
            "required": ["id", "new_start_at"]
        }"#,
    },
    ToolDefinition {
        name: "time_schedule_range",
        description: "Get schedule for a time range",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "start": { "type": "string", "format": "date-time" },
                "end": { "type": "string", "format": "date-time" }
            },
            "required": ["start", "end"]
        }"#,
    },
    ToolDefinition {
        name: "time_schedule_available",
        description: "Find available time slots",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "start": { "type": "string", "format": "date-time" },
                "end": { "type": "string", "format": "date-time" },
                "min_duration_minutes": { "type": "integer", "default": 30 }
            },
            "required": ["start", "end"]
        }"#,
    },
    ToolDefinition {
        name: "time_schedule_conflicts",
        description: "Check if a proposed time conflicts with existing schedule",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "start_at": { "type": "string", "format": "date-time" },
                "duration_minutes": { "type": "integer" }
            },
            "required": ["start_at", "duration_minutes"]
        }"#,
    },
    
    // --- Sequence tools ---
    ToolDefinition {
        name: "time_sequence_create",
        description: "Create a new sequence of ordered steps",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "label": { "type": "string", "description": "Sequence name" },
                "steps": { 
                    "type": "array", 
                    "items": {
                        "type": "object",
                        "properties": {
                            "label": { "type": "string" },
                            "duration_minutes": { "type": "integer" },
                            "depends_on": { "type": "array", "items": { "type": "integer" } }
                        },
                        "required": ["label"]
                    }
                },
                "allow_parallel": { "type": "boolean", "default": false }
            },
            "required": ["label", "steps"]
        }"#,
    },
    ToolDefinition {
        name: "time_sequence_advance",
        description: "Complete current step and advance sequence",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Sequence ID" }
            },
            "required": ["id"]
        }"#,
    },
    ToolDefinition {
        name: "time_sequence_status",
        description: "Get status of a sequence",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Sequence ID" }
            },
            "required": ["id"]
        }"#,
    },
    
    // --- Decay tools ---
    ToolDefinition {
        name: "time_decay_create",
        description: "Create a decay model for tracking time-sensitive value",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "label": { "type": "string", "description": "What is decaying" },
                "initial_value": { "type": "number", "description": "Starting value" },
                "decay_type": { "type": "string", "enum": ["linear", "exponential", "half_life", "step"], "default": "exponential" },
                "rate": { "type": "number", "description": "Decay rate (interpretation depends on decay_type)" },
                "half_life_hours": { "type": "number", "description": "For half_life type: hours until half value" },
                "floor": { "type": "number", "default": 0, "description": "Minimum value" },
                "tags": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["label", "initial_value"]
        }"#,
    },
    ToolDefinition {
        name: "time_decay_value",
        description: "Get current value of a decay model",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Decay ID" }
            },
            "required": ["id"]
        }"#,
    },
    ToolDefinition {
        name: "time_decay_alert",
        description: "Get decays that will fall below threshold within duration",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "threshold": { "type": "number", "description": "Value threshold" },
                "within_hours": { "type": "number", "description": "Time window in hours" }
            },
            "required": ["threshold", "within_hours"]
        }"#,
    },
    
    // --- Duration tools ---
    ToolDefinition {
        name: "time_duration_estimate",
        description: "Create a duration estimate with uncertainty",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "label": { "type": "string", "description": "What this duration is for" },
                "optimistic_minutes": { "type": "integer", "description": "Best case duration" },
                "expected_minutes": { "type": "integer", "description": "Most likely duration" },
                "pessimistic_minutes": { "type": "integer", "description": "Worst case duration" },
                "confidence": { "type": "number", "minimum": 0, "maximum": 1, "default": 0.7 },
                "tags": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["label", "expected_minutes"]
        }"#,
    },
    ToolDefinition {
        name: "time_duration_aggregate",
        description: "Aggregate multiple duration estimates",
        input_schema: r#"{
            "type": "object",
            "properties": {
                "ids": { "type": "array", "items": { "type": "string" }, "description": "Duration IDs to aggregate" }
            },
            "required": ["ids"]
        }"#,
    },
    
    // --- General tools ---
    ToolDefinition {
        name: "time_stats",
        description: "Get temporal statistics",
        input_schema: r#"{ "type": "object", "properties": {} }"#,
    },
    ToolDefinition {
        name: "time_refresh",
        description: "Update all temporal statuses (deadlines, decays)",
        input_schema: r#"{ "type": "object", "properties": {} }"#,
    },
];

pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub input_schema: &'static str,
}
```

---

# PART 10: MCP-RESOURCES

```rust
// crates/agentic-time-mcp/src/resources.rs

/// MCP Resource URI patterns for AgenticTime
/// 
/// URI Scheme: atime://
/// 
/// Resources:
/// - atime://deadline/{id}      - Single deadline
/// - atime://deadline/          - List all deadlines
/// - atime://deadline/overdue   - Overdue deadlines
/// - atime://deadline/upcoming  - Upcoming deadlines (next 7 days)
/// 
/// - atime://schedule/{id}      - Single schedule
/// - atime://schedule/          - List all schedules
/// - atime://schedule/today     - Today's schedule
/// - atime://schedule/week      - This week's schedule
/// 
/// - atime://sequence/{id}      - Single sequence
/// - atime://sequence/          - List all sequences
/// - atime://sequence/active    - Active sequences
/// 
/// - atime://decay/{id}         - Single decay model
/// - atime://decay/             - List all decay models
/// - atime://decay/critical     - Decays below 20% value
/// 
/// - atime://duration/{id}      - Single duration estimate
/// - atime://duration/          - List all estimates
/// 
/// - atime://stats              - Overall statistics

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

pub fn list_resources() -> Vec<ResourceDefinition> {
    vec![
        // Deadlines
        ResourceDefinition {
            uri: "atime://deadline/".to_string(),
            name: "All Deadlines".to_string(),
            description: "List of all deadlines".to_string(),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "atime://deadline/overdue".to_string(),
            name: "Overdue Deadlines".to_string(),
            description: "List of overdue deadlines".to_string(),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "atime://deadline/upcoming".to_string(),
            name: "Upcoming Deadlines".to_string(),
            description: "Deadlines due in next 7 days".to_string(),
            mime_type: "application/json".to_string(),
        },
        
        // Schedules
        ResourceDefinition {
            uri: "atime://schedule/".to_string(),
            name: "All Schedules".to_string(),
            description: "List of all scheduled events".to_string(),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "atime://schedule/today".to_string(),
            name: "Today's Schedule".to_string(),
            description: "Schedule for today".to_string(),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "atime://schedule/week".to_string(),
            name: "This Week's Schedule".to_string(),
            description: "Schedule for this week".to_string(),
            mime_type: "application/json".to_string(),
        },
        
        // Sequences
        ResourceDefinition {
            uri: "atime://sequence/".to_string(),
            name: "All Sequences".to_string(),
            description: "List of all sequences".to_string(),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "atime://sequence/active".to_string(),
            name: "Active Sequences".to_string(),
            description: "Sequences currently in progress".to_string(),
            mime_type: "application/json".to_string(),
        },
        
        // Decay
        ResourceDefinition {
            uri: "atime://decay/".to_string(),
            name: "All Decay Models".to_string(),
            description: "List of all decay models".to_string(),
            mime_type: "application/json".to_string(),
        },
        ResourceDefinition {
            uri: "atime://decay/critical".to_string(),
            name: "Critical Decays".to_string(),
            description: "Decay models below 20% value".to_string(),
            mime_type: "application/json".to_string(),
        },
        
        // Stats
        ResourceDefinition {
            uri: "atime://stats".to_string(),
            name: "Temporal Statistics".to_string(),
            description: "Overall temporal statistics".to_string(),
            mime_type: "application/json".to_string(),
        },
    ]
}
```

---

# PART 11: MCP-PROMPTS

```rust
// crates/agentic-time-mcp/src/prompts.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptDefinition {
    pub name: String,
    pub description: String,
    pub arguments: Vec<PromptArgument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

pub fn list_prompts() -> Vec<PromptDefinition> {
    vec![
        PromptDefinition {
            name: "time_plan".to_string(),
            description: "Plan a series of tasks with deadlines and dependencies".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "goal".to_string(),
                    description: "What you're trying to accomplish".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "deadline".to_string(),
                    description: "When it needs to be done".to_string(),
                    required: false,
                },
            ],
        },
        PromptDefinition {
            name: "time_review".to_string(),
            description: "Review current temporal state: deadlines, schedules, decays".to_string(),
            arguments: vec![],
        },
        PromptDefinition {
            name: "time_estimate".to_string(),
            description: "Estimate how long something will take".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "task".to_string(),
                    description: "What you're estimating".to_string(),
                    required: true,
                },
            ],
        },
        PromptDefinition {
            name: "time_schedule_day".to_string(),
            description: "Create a schedule for the day".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "tasks".to_string(),
                    description: "Tasks to schedule".to_string(),
                    required: true,
                },
            ],
        },
    ]
}

pub fn expand_prompt(name: &str, args: &std::collections::HashMap<String, String>) -> Option<String> {
    match name {
        "time_plan" => {
            let goal = args.get("goal")?;
            let deadline = args.get("deadline").map(|d| format!(" by {}", d)).unwrap_or_default();
            Some(format!(
                r#"I need to plan how to accomplish: {}{}

Please:
1. Break this down into sequential steps
2. Estimate duration for each step
3. Identify dependencies between steps
4. Create deadlines for milestones
5. Flag any scheduling conflicts

Use the time_sequence_create and time_deadline_add tools to record the plan."#,
                goal, deadline
            ))
        }
        "time_review" => {
            Some(r#"Please review my current temporal state:

1. Check for overdue deadlines (time_deadline_overdue)
2. Show today's schedule (atime://schedule/today resource)
3. List active sequences (time_sequence_status for each)
4. Check for critical decays (atime://decay/critical resource)
5. Show overall stats (time_stats)

Summarize what needs attention."#.to_string())
        }
        "time_estimate" => {
            let task = args.get("task")?;
            Some(format!(
                r#"I need to estimate how long this will take: {}

Please:
1. Consider optimistic, expected, and pessimistic scenarios
2. Factor in potential blockers or dependencies
3. Use time_duration_estimate to record the estimate
4. If this has subtasks, estimate each and aggregate"#,
                task
            ))
        }
        "time_schedule_day" => {
            let tasks = args.get("tasks")?;
            Some(format!(
                r#"I need to schedule these tasks for today: {}

Please:
1. Check for existing commitments (atime://schedule/today)
2. Find available slots (time_schedule_available)
3. Prioritize tasks by urgency/importance
4. Account for context switching time
5. Leave buffer for unexpected items
6. Use time_schedule_create for each task"#,
                tasks
            ))
        }
        _ => None,
    }
}
```

---

# PART 12: EDGE-CASES

## 16 Mandatory Edge Case Categories

```
SECURITY (1-5):
───────────────
1. Invalid date-time formats → Reject with clear error
2. Malformed JSON input → JSON-RPC error -32700
3. Oversized input (>8 MiB) → Reject before parsing
4. Invalid temporal IDs (non-UUID) → 404 or validation error
5. Negative durations → Reject with validation error

USER EXPERIENCE (6-10):
───────────────────────
6. Empty file (no entities) → Return empty arrays, not errors
7. Corrupted .atime file → Return file format error with recovery hint
8. Missing file → Create new file or clear error message
9. Timezone handling → All times UTC internally, convert on display
10. Date arithmetic overflow → Handle dates far in future gracefully

CONCURRENCY (11-12):
────────────────────
11. Graceful shutdown (SIGTERM) → Save state, close cleanly
12. Concurrent file access → File locking, atomic writes

BOUNDARY VALUES (13-16):
────────────────────────
13. Zero duration → Valid (instant events)
14. Maximum dates (year 9999) → Handle without overflow
15. Empty strings for labels → Require non-empty
16. Maximum entities (100K+) → Maintain <100ms query performance
```

---

# PART 13: CLI

```
atime — Temporal reasoning CLI

USAGE:
    atime <COMMAND>

COMMANDS:
    deadline    Manage deadlines
    schedule    Manage schedules
    sequence    Manage sequences
    decay       Manage decay models
    duration    Manage duration estimates
    stats       Show temporal statistics
    refresh     Update all statuses
    info        Show .atime file info
    install     Configure MCP server
    serve       Start MCP server

DEADLINE COMMANDS:
    atime deadline add <LABEL> --due <DATETIME> [--warn <DATETIME>] [--type <TYPE>]
    atime deadline complete <ID>
    atime deadline cancel <ID>
    atime deadline list [--status <STATUS>] [--due-within <DURATION>]
    atime deadline overdue

SCHEDULE COMMANDS:
    atime schedule create <LABEL> --start <DATETIME> --duration <MINUTES> [--priority <PRIORITY>]
    atime schedule reschedule <ID> --start <DATETIME>
    atime schedule cancel <ID>
    atime schedule range --start <DATETIME> --end <DATETIME>
    atime schedule today
    atime schedule week
    atime schedule available --start <DATETIME> --end <DATETIME> [--min-duration <MINUTES>]

SEQUENCE COMMANDS:
    atime sequence create <LABEL> --steps <JSON>
    atime sequence advance <ID>
    atime sequence status <ID>
    atime sequence list [--status <STATUS>]

DECAY COMMANDS:
    atime decay create <LABEL> --initial <VALUE> [--type <TYPE>] [--rate <RATE>]
    atime decay value <ID>
    atime decay alert --threshold <VALUE> --within <HOURS>
    atime decay list

DURATION COMMANDS:
    atime duration estimate <LABEL> --expected <MINUTES> [--optimistic <MINUTES>] [--pessimistic <MINUTES>]
    atime duration aggregate <ID1> <ID2> ...
    atime duration list

GENERAL:
    atime stats
    atime refresh
    atime info [PATH]
    atime install [--profile <desktop|terminal|server>]
    atime serve [--stdio | --sse]
```

---

# PART 14: FFI

```c
// include/agentic_time.h

#ifndef AGENTIC_TIME_H
#define AGENTIC_TIME_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle
typedef struct TimeFile TimeFile;

// Error codes
typedef enum {
    ATIME_OK = 0,
    ATIME_ERR_NOT_FOUND = 1,
    ATIME_ERR_INVALID_RANGE = 2,
    ATIME_ERR_DEADLINE_PASSED = 3,
    ATIME_ERR_SCHEDULE_CONFLICT = 4,
    ATIME_ERR_DEPENDENCY_NOT_MET = 5,
    ATIME_ERR_FILE_FORMAT = 6,
    ATIME_ERR_IO = 7,
} AtimeError;

// File operations
TimeFile* atime_open(const char* path, AtimeError* err);
TimeFile* atime_create(const char* path, AtimeError* err);
void atime_close(TimeFile* file);
AtimeError atime_save(TimeFile* file);

// Deadline operations
AtimeError atime_deadline_add(
    TimeFile* file,
    const char* label,
    int64_t due_at_micros,
    char* id_out,
    size_t id_out_len
);

AtimeError atime_deadline_complete(TimeFile* file, const char* id);

// Query operations
AtimeError atime_deadlines_overdue(
    TimeFile* file,
    char* json_out,
    size_t json_out_len,
    size_t* written
);

// Stats
AtimeError atime_stats(
    TimeFile* file,
    char* json_out,
    size_t json_out_len,
    size_t* written
);

#ifdef __cplusplus
}
#endif

#endif // AGENTIC_TIME_H
```

---

# PART 15: TESTS

## Test Phases

### Phase 1: Data Structures
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn duration_pert_estimate() {
        let d = DurationEstimate {
            optimistic: ChronoDuration::hours(1),
            expected: ChronoDuration::hours(2),
            pessimistic: ChronoDuration::hours(5),
            ..Default::default()
        };
        // PERT: (1 + 4*2 + 5) / 6 = 14/6 ≈ 2.33 hours
        assert!(d.pert_estimate().num_minutes() >= 130);
        assert!(d.pert_estimate().num_minutes() <= 150);
    }
    
    #[test]
    fn deadline_overdue_detection() {
        let mut d = Deadline {
            due_at: Utc::now() - ChronoDuration::hours(1),
            status: DeadlineStatus::Pending,
            ..Default::default()
        };
        d.update_status();
        assert_eq!(d.status, DeadlineStatus::Overdue);
    }
    
    #[test]
    fn schedule_conflict_detection() {
        let s1 = Schedule {
            start_at: Utc::now(),
            duration: DurationEstimate {
                expected: ChronoDuration::hours(2),
                ..Default::default()
            },
            ..Default::default()
        };
        let s2 = Schedule {
            start_at: Utc::now() + ChronoDuration::hours(1),
            duration: DurationEstimate {
                expected: ChronoDuration::hours(2),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(s1.conflicts_with(&s2));
    }
    
    #[test]
    fn decay_exponential() {
        let d = DecayModel {
            initial_value: 100.0,
            decay_type: DecayType::HalfLife {
                half_life: ChronoDuration::hours(1),
            },
            floor: 0.0,
            reference_time: Utc::now() - ChronoDuration::hours(1),
            ..Default::default()
        };
        let value = d.calculate_value(Utc::now());
        assert!(value >= 45.0 && value <= 55.0); // ~50 after one half-life
    }
}
```

### Phase 2: File Format
```rust
#[test]
fn file_create_and_reopen() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("test.atime");
    
    // Create and add entity
    let mut file = TimeFile::create(&path).unwrap();
    let deadline = Deadline {
        id: TemporalId::new(),
        label: "Test".to_string(),
        due_at: Utc::now() + ChronoDuration::days(1),
        ..Default::default()
    };
    file.add(EntityType::Deadline, deadline.id, &deadline).unwrap();
    file.save().unwrap();
    
    // Reopen and verify
    let file2 = TimeFile::open(&path).unwrap();
    let loaded: Deadline = file2.get(&deadline.id).unwrap().unwrap();
    assert_eq!(loaded.label, "Test");
}
```

### Phase 3: Query Engine
```rust
#[test]
fn query_deadlines_due_within() {
    // Setup file with deadlines at various times
    // Query for next 24 hours
    // Verify correct deadlines returned
}

#[test]
fn query_available_slots() {
    // Setup file with schedules
    // Query for available slots
    // Verify gaps are correctly identified
}
```

### Phase 4: MCP Integration
```rust
#[test]
fn mcp_deadline_add_tool() {
    // Send JSON-RPC request
    // Verify deadline created
}

#[test]
fn mcp_resource_deadline_overdue() {
    // Request atime://deadline/overdue
    // Verify correct JSON response
}
```

### Benchmarks
```rust
#[bench]
fn bench_100k_deadlines_query(b: &mut Bencher) {
    // Create file with 100K deadlines
    // Benchmark query performance
    // Target: <100ms
}
```

---

# PART 16: RESEARCH-PAPER

```latex
\documentclass[11pt,a4paper]{article}

\title{AgenticTime: Temporal Reasoning for Autonomous AI Agents}
\author{Agentralabs Research}
\date{\today}

\begin{document}

\maketitle

\begin{abstract}
We present AgenticTime, a temporal reasoning system that provides AI agents 
with the ability to model and reason about time. Unlike simple timestamp 
storage, AgenticTime provides duration modeling with uncertainty, deadline 
management with consequence awareness, schedule coordination with conflict 
detection, sequence dependencies, and value decay curves. Integrated via 
the Model Context Protocol (MCP), AgenticTime enables any LLM to answer 
temporal questions such as ``when should this happen?'', ``what's overdue?'', 
and ``what loses value if ignored?''

We demonstrate sub-millisecond query performance on graphs with 100,000+ 
temporal entities, with a binary file format that achieves 5x compression 
compared to JSON storage.
\end{abstract}

\section{Introduction}
% Problem: AI agents have no concept of time
% Solution: Structured temporal reasoning
% Contribution: Novel data structures + MCP integration

\section{Related Work}
% Calendar APIs
% Temporal databases
% Planning systems

\section{Architecture}
% Data structures
% File format
% Indexes

\section{Temporal Primitives}
% Durations with uncertainty
% Deadlines with consequences
% Schedules with conflicts
% Sequences with dependencies
% Decay models

\section{MCP Integration}
% Tools
% Resources
% Prompts

\section{Evaluation}
% Benchmarks
% Real benchmark data here

\section{Conclusion}
% Summary
% Future work

\end{document}
```

---

# PART 17: CANONICAL-KIT

Create `CANONICAL_SISTER_KIT.md` with all sections from the family template.
Copy from Memory's version and adapt for Time.

---

# PART 18: DOCS-PUBLIC

## Required Pages (13)

1. **overview.md** — What is AgenticTime, core concepts
2. **quickstart.md** — 5-minute getting started
3. **installation.md** — All install methods
4. **command-surface.md** — MCP tools reference
5. **experience-with-vs-without.md** — Before/after comparison
6. **runtime-install-sync.md** — How Ghost Writer syncs
7. **integration-guide.md** — Integrating with other systems
8. **concepts.md** — Deep dive into temporal primitives
9. **api-reference.md** — Full API documentation
10. **file-format.md** — .atime format specification
11. **benchmarks.md** — Performance benchmarks
12. **faq.md** — Frequently asked questions
13. **playbooks-agent-integration.md** — Agent integration patterns

Each must have frontmatter:
```yaml
---
status: published
---
```

---

# PART 19: ASSETS

## Required SVGs

1. **github-hero-pane.svg** — Hero image for README
2. **architecture.svg** — System architecture diagram
3. **benchmarks.svg** — Performance benchmark visualization

Style: Agentra design system (neutral substrate, mono typography, orange accent)

---

# PART 20: README-LAYOUT

```markdown
<!-- Hero image -->
<img src="assets/github-hero-pane.svg" />

<!-- Badges -->
[![Crates.io](https://img.shields.io/crates/v/agentic-time.svg)](https://crates.io/crates/agentic-time)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()

<!-- Navigation -->
[Install](#install) • [Quickstart](#quickstart) • [MCP Tools](#mcp-tools) • [Docs](docs/)

# AgenticTime

**Temporal reasoning for AI agents.**

> "When should this happen?"

## The Problem

AI agents have no concept of time...

## Install

```bash
cargo install agentic-time-cli
atime install
```

## Quickstart

...

## How It Works

...

## MCP Tools

| Tool | Description |
|------|-------------|
| time_deadline_add | Add deadline |
| ... | ... |

## Benchmarks

...
```

---

# PART 21: INSTALL-SCRIPT

```bash
#!/usr/bin/env bash
# scripts/install.sh

set -euo pipefail

# ... (copy pattern from agentic-memory/scripts/install.sh)
# Profile-based install
# Merge-only MCP config
# Post-install restart instruction
# Feedback prompt
```

---

# PART 22: CI-WORKFLOWS

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test

  guardrails:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/check-canonical-sister.sh
```

---

# PART 23: GHOST-WRITER

```rust
// crates/agentic-time-mcp/src/ghost_bridge.rs

// Copy exact pattern from agentic-memory-mcp/src/ghost_bridge.rs
// 
// Key points:
// - Spawn tokio background task
// - 5-second sync interval
// - Detect client dirs: Claude, Cursor, Windsurf, Cody
// - Atomic writes (temp file + rename)
// - Format as TIME_CONTEXT.md
// - Include: upcoming deadlines, today's schedule, active sequences, critical decays

use std::path::PathBuf;
use tokio::time::{interval, Duration};

pub fn detect_all_memory_dirs() -> Vec<(String, PathBuf)> {
    let mut dirs = Vec::new();
    
    if let Some(home) = dirs::home_dir() {
        // Claude
        let claude_dir = home.join(".claude").join("memory");
        if claude_dir.exists() || std::fs::create_dir_all(&claude_dir).is_ok() {
            dirs.push(("Claude".to_string(), claude_dir));
        }
        
        // Cursor
        let cursor_dir = home.join(".cursor").join("memory");
        if cursor_dir.exists() || std::fs::create_dir_all(&cursor_dir).is_ok() {
            dirs.push(("Cursor".to_string(), cursor_dir));
        }
        
        // Windsurf
        let windsurf_dir = home.join(".windsurf").join("memory");
        if windsurf_dir.exists() || std::fs::create_dir_all(&windsurf_dir).is_ok() {
            dirs.push(("Windsurf".to_string(), windsurf_dir));
        }
        
        // Cody
        let cody_dir = home.join(".sourcegraph").join("cody").join("memory");
        if cody_dir.exists() || std::fs::create_dir_all(&cody_dir).is_ok() {
            dirs.push(("Cody".to_string(), cody_dir));
        }
    }
    
    dirs
}

pub fn format_time_context(/* query engine */) -> String {
    // Format context as markdown
    let mut content = String::new();
    content.push_str("# AgenticTime Context\n\n");
    content.push_str(&format!("*Last sync: {}*\n\n", chrono::Utc::now()));
    
    // Overdue deadlines
    content.push_str("## ⚠️ Overdue\n");
    // ...
    
    // Today's schedule
    content.push_str("## 📅 Today\n");
    // ...
    
    // Active sequences
    content.push_str("## 🔄 In Progress\n");
    // ...
    
    // Critical decays
    content.push_str("## 📉 Attention Needed\n");
    // ...
    
    content
}

pub fn spawn_ghost_writer(/* engine */) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        
        loop {
            ticker.tick().await;
            
            let dirs = detect_all_memory_dirs();
            let content = format_time_context(/* ... */);
            
            for (client, dir) in &dirs {
                let filename = match client.as_str() {
                    "Claude" => "TIME_CONTEXT.md",
                    _ => "agentic-time.md",
                };
                let path = dir.join(filename);
                let tmp_path = dir.join(format!(".{}.tmp", filename));
                
                // Atomic write
                if let Ok(()) = std::fs::write(&tmp_path, &content) {
                    let _ = std::fs::rename(&tmp_path, &path);
                }
            }
        }
    });
}
```

---

# PART 24: MCP-HARDENING

```rust
// crates/agentic-time-mcp/src/stdio.rs

use std::io::{BufRead, Read, Write};

/// Maximum frame size (8 MiB)
pub const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024;

/// Hardened stdio transport with Content-Length framing
pub struct StdioTransport {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }
    
    /// Read a framed message
    pub fn read_message(&mut self) -> Result<String, TransportError> {
        let mut stdin = self.stdin.lock();
        let mut headers = String::new();
        
        // Read headers until empty line
        loop {
            let mut line = String::new();
            stdin.read_line(&mut line)?;
            
            if line == "\r\n" || line == "\n" {
                break;
            }
            headers.push_str(&line);
        }
        
        // Parse Content-Length
        let content_length = headers
            .lines()
            .find(|l| l.to_lowercase().starts_with("content-length:"))
            .and_then(|l| l.split(':').nth(1))
            .and_then(|v| v.trim().parse::<usize>().ok())
            .ok_or(TransportError::MissingContentLength)?;
        
        // Validate size
        if content_length > MAX_CONTENT_LENGTH_BYTES {
            return Err(TransportError::MessageTooLarge {
                size: content_length,
                max: MAX_CONTENT_LENGTH_BYTES,
            });
        }
        
        // Read body
        let mut body = vec![0u8; content_length];
        stdin.read_exact(&mut body)?;
        
        String::from_utf8(body).map_err(|_| TransportError::InvalidUtf8)
    }
    
    /// Write a framed message
    pub fn write_message(&mut self, content: &str) -> Result<(), TransportError> {
        let mut stdout = self.stdout.lock();
        
        write!(stdout, "Content-Length: {}\r\n\r\n{}", content.len(), content)?;
        stdout.flush()?;
        
        Ok(())
    }
}

/// Validate JSON-RPC 2.0 request
pub fn validate_jsonrpc(request: &serde_json::Value) -> Result<(), TransportError> {
    let version = request.get("jsonrpc").and_then(|v| v.as_str());
    
    if version != Some("2.0") {
        return Err(TransportError::InvalidJsonRpcVersion {
            expected: "2.0".to_string(),
            got: version.map(String::from),
        });
    }
    
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("Missing Content-Length header")]
    MissingContentLength,
    
    #[error("Message too large: {size} bytes (max: {max})")]
    MessageTooLarge { size: usize, max: usize },
    
    #[error("Invalid UTF-8 in message")]
    InvalidUtf8,
    
    #[error("Invalid JSON-RPC version: expected {expected}, got {got:?}")]
    InvalidJsonRpcVersion { expected: String, got: Option<String> },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

# PART 25: BINDINGS

## Python (PyO3)

```toml
# python/pyproject.toml
[project]
name = "agentic-time"
version = "0.1.0"
requires-python = ">=3.8"

[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"
```

## WASM

```toml
# wasm/Cargo.toml
[package]
name = "agentic-time-wasm"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
agentic-time = { path = "../crates/agentic-time" }
wasm-bindgen = "0.2"
```

## npm

```json
{
  "name": "@agentic/time",
  "version": "0.1.0",
  "description": "Temporal reasoning for AI agents",
  "main": "index.js",
  "scripts": {
    "build": "wasm-pack build ../wasm --target nodejs"
  }
}
```

---

# END OF SPECIFICATION

**AgenticTime is fully specified across all 25 parts.**

**Give her life.** 🕐
