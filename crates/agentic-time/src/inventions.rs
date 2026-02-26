//! The 16 Temporal Inventions — advanced temporal reasoning capabilities.
//!
//! Organized into five categories:
//!
//! - **Exploration** (1-3): Temporal Replay, Temporal Cloning, Temporal Echoes
//! - **Prediction** (4-6): Deadline Prophecy, Causal Archaeology, Future Memory
//! - **Management** (7-9): Temporal Debt, Chrono-Gravity, Temporal Entanglement
//! - **Protection** (10-12): Temporal Immune System, Decay Reversal, Time Dilation Awareness
//! - **Time Travel** (13-16): Temporal Jump, Temporal Anchors, Time Loops, Temporal Wormholes

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::TemporalId;

// ─── EXPLORATION (1-3) ─────────────────────────────────────────────────────────

// ─── 1. Temporal Replay ────────────────────────────────────────────────────────

/// A decision point that could have gone differently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchPoint {
    /// Unique identifier.
    pub id: TemporalId,
    /// When the decision was made.
    pub decided_at: DateTime<Utc>,
    /// What was decided.
    pub decision: String,
    /// All alternatives that were considered.
    pub alternatives: Vec<Alternative>,
    /// Snapshot of the context at decision time.
    pub context_snapshot: serde_json::Value,
    /// What actually happened after this decision.
    pub actual_outcome: Option<String>,
}

/// One possible alternative at a branch point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    /// Short label for this alternative.
    pub label: String,
    /// Detailed description.
    pub description: String,
    /// Whether this was the alternative that was actually chosen.
    pub was_chosen: bool,
}

/// A forked timeline exploring a different choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineFork {
    /// Unique identifier.
    pub id: TemporalId,
    /// The branch point this fork diverges from.
    pub branch_point: TemporalId,
    /// Index into the branch point's alternatives vec.
    pub alternative_index: usize,
    /// Simulated events that would have occurred.
    pub simulated_events: Vec<SimulatedEvent>,
    /// Projected outcome of this timeline.
    pub projected_outcome: String,
    /// Confidence in this projection (0.0-1.0).
    pub confidence: f64,
}

/// A simulated event in a forked timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedEvent {
    /// When this event would have occurred.
    pub timestamp: DateTime<Utc>,
    /// What would have happened.
    pub description: String,
    /// Impact magnitude (-1.0 to 1.0, negative = bad).
    pub impact: f64,
}

// ─── 2. Temporal Cloning ───────────────────────────────────────────────────────

/// A parallel exploration clone testing a hypothesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalClone {
    /// Unique identifier.
    pub id: TemporalId,
    /// Parent entity this clone was spawned from.
    pub parent: TemporalId,
    /// What this clone is testing.
    pub hypothesis: String,
    /// Current status.
    pub status: CloneStatus,
    /// Findings discovered during exploration.
    pub findings: Vec<Finding>,
    /// How long this clone has been exploring (seconds).
    pub exploration_time_secs: i64,
    /// When the clone was created.
    pub created_at: DateTime<Utc>,
    /// When the clone finished (if it has).
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status of a temporal clone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloneStatus {
    /// Still actively exploring.
    Exploring,
    /// Found a viable solution.
    FoundSolution,
    /// Ruled out the hypothesis.
    RuledOut,
    /// Merged back into the parent.
    Merged,
    /// Abandoned without conclusion.
    Abandoned,
}

/// A finding from a temporal clone's exploration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// What was discovered.
    pub description: String,
    /// Confidence in this finding (0.0-1.0).
    pub confidence: f64,
    /// Evidence supporting this finding.
    pub evidence: String,
}

/// A race between multiple clones to find the best approach.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneRace {
    /// Unique identifier.
    pub id: TemporalId,
    /// Clones participating in this race.
    pub clones: Vec<TemporalId>,
    /// The winning clone (if decided).
    pub winner: Option<TemporalId>,
    /// When the race began.
    pub started_at: DateTime<Utc>,
}

// ─── 3. Temporal Echoes ────────────────────────────────────────────────────────

/// Ripple effects from a change propagating through time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEcho {
    /// Unique identifier.
    pub id: TemporalId,
    /// The change that triggered the ripples.
    pub source_change: String,
    /// Ripples across different time horizons.
    pub ripples: Vec<Ripple>,
    /// Overall impact summary.
    pub impact_summary: String,
}

/// A single ripple at a particular time horizon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ripple {
    /// When this ripple manifests.
    pub time_horizon: TimeHorizon,
    /// Items affected by this ripple.
    pub affected_items: Vec<AffectedItem>,
    /// Severity (0.0-1.0).
    pub severity: f64,
    /// Whether the ripple's effects can be undone.
    pub reversible: bool,
}

/// How far into the future a ripple reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeHorizon {
    /// Minutes to hours.
    Immediate,
    /// Hours to days.
    ShortTerm,
    /// Days to weeks.
    MediumTerm,
    /// Weeks to months.
    LongTerm,
    /// Months to years.
    Extended,
}

/// An item affected by a temporal ripple.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedItem {
    /// ID of the affected entity.
    pub entity_id: TemporalId,
    /// Description of how it is affected.
    pub description: String,
    /// Impact magnitude (0.0-1.0).
    pub impact: f64,
}

// ─── PREDICTION (4-6) ──────────────────────────────────────────────────────────

// ─── 4. Deadline Prophecy ──────────────────────────────────────────────────────

/// A prediction about whether a deadline will be met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineProphecy {
    /// Unique identifier.
    pub id: TemporalId,
    /// The deadline being prophesied about.
    pub deadline_id: TemporalId,
    /// Probability of meeting the deadline (0.0-1.0).
    pub success_probability: f64,
    /// Factors influencing the prophecy.
    pub factors: Vec<ProphecyFactor>,
    /// Possible interventions to improve odds.
    pub interventions: Vec<Intervention>,
    /// When this prophecy was made.
    pub prophesied_at: DateTime<Utc>,
    /// How accurate past prophecies have been (0.0-1.0).
    pub historical_accuracy: f64,
}

/// A factor influencing a deadline prophecy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProphecyFactor {
    /// Name of the factor.
    pub name: String,
    /// How much this factor impacts the prophecy (-1.0 to 1.0).
    pub impact: f64,
    /// Confidence in this factor's assessment (0.0-1.0).
    pub confidence: f64,
    /// Evidence for this factor.
    pub evidence: String,
}

/// A possible intervention to improve deadline success.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    /// What action to take.
    pub action: String,
    /// Success probability after this intervention (0.0-1.0).
    pub success_probability_after: f64,
    /// Cost description.
    pub cost: String,
    /// Whether this intervention is recommended.
    pub recommended: bool,
}

// ─── 5. Causal Archaeology ─────────────────────────────────────────────────────

/// An excavation into the root cause of a problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalExcavation {
    /// Unique identifier.
    pub id: TemporalId,
    /// The problem being investigated.
    pub problem: String,
    /// The identified root cause.
    pub root_cause: CausalEvent,
    /// The chain of events from root cause to problem.
    pub causal_chain: Vec<CausalEvent>,
    /// Points where the chain could have been broken.
    pub inflection_points: Vec<InflectionPoint>,
    /// Recurring patterns found during excavation.
    pub patterns: Vec<CausalPattern>,
}

/// An event in a causal chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEvent {
    /// When the event occurred.
    pub timestamp: DateTime<Utc>,
    /// What happened.
    pub description: String,
    /// Type of causal event.
    pub event_type: CausalEventType,
    /// What this event led to.
    pub led_to: Vec<String>,
}

/// Classification of a causal event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CausalEventType {
    /// A deliberate choice.
    Decision,
    /// Something from outside the system.
    ExternalEvent,
    /// Failure to act.
    Inaction,
    /// Communication breakdown.
    Miscommunication,
    /// Something broke.
    TechnicalFailure,
}

/// A point where a different action could have changed the outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflectionPoint {
    /// The event at this inflection point.
    pub event: CausalEvent,
    /// What could have been done differently.
    pub alternative_action: String,
    /// Estimated impact of the alternative.
    pub estimated_impact: String,
}

/// A recurring causal pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPattern {
    /// Description of the pattern.
    pub description: String,
    /// How many times this pattern has occurred.
    pub occurrences: u32,
    /// Severity of this pattern (0.0-1.0).
    pub severity: f64,
}

// ─── 6. Future Memory ──────────────────────────────────────────────────────────

/// Pre-loaded context for an upcoming event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureMemory {
    /// Unique identifier.
    pub id: TemporalId,
    /// The event this memory is for.
    pub event_id: TemporalId,
    /// When the event occurs.
    pub event_time: DateTime<Utc>,
    /// Relevant context items.
    pub context_items: Vec<ContextItem>,
    /// Data precomputed for quick access.
    pub precomputed: Vec<PrecomputedData>,
    /// Suggested areas of focus.
    pub suggested_focus: Vec<String>,
    /// When this future memory was created.
    pub created_at: DateTime<Utc>,
}

/// A piece of context relevant to a future event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    /// Short label.
    pub label: String,
    /// Type of content.
    pub content_type: ContextType,
    /// The actual content.
    pub content: String,
    /// How relevant this is (0.0-1.0).
    pub relevance: f64,
}

/// Classification of context content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextType {
    /// A document.
    Document,
    /// Notes from a previous meeting.
    PreviousMeeting,
    /// A related task.
    RelatedTask,
    /// Information about a person.
    PersonInfo,
    /// A metric or data point.
    Metric,
}

/// Data precomputed for a future event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecomputedData {
    /// Label for this data.
    pub label: String,
    /// The precomputed data.
    pub data: serde_json::Value,
}

// ─── MANAGEMENT (7-9) ──────────────────────────────────────────────────────────

// ─── 7. Temporal Debt ──────────────────────────────────────────────────────────

/// A unit of temporal debt — postponed work that compounds over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDebt {
    /// Unique identifier.
    pub id: TemporalId,
    /// What was deferred.
    pub description: String,
    /// Original time cost in minutes.
    pub principal_minutes: u32,
    /// Interest rate (fraction per compound period).
    pub interest_rate: f64,
    /// How often interest compounds (seconds).
    pub compound_period_secs: i64,
    /// When the debt was incurred.
    pub incurred_at: DateTime<Utc>,
    /// Current total including interest (minutes).
    pub current_total_minutes: u32,
    /// What this debt is blocking.
    pub blocking: Vec<String>,
    /// Category of debt.
    pub category: DebtCategory,
    /// When the debt was paid off (if it has been).
    pub paid_at: Option<DateTime<Utc>>,
}

/// Category of temporal debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebtCategory {
    /// Deferred tests.
    Testing,
    /// Missing or outdated documentation.
    Documentation,
    /// Code that needs restructuring.
    Refactoring,
    /// Security improvements deferred.
    Security,
    /// Performance optimizations deferred.
    Performance,
}

/// A report summarizing all temporal debt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtReport {
    /// Total debt across all items (minutes).
    pub total_debt_minutes: u32,
    /// Individual debt items.
    pub items: Vec<TemporalDebt>,
    /// Interest accruing per week (minutes).
    pub interest_per_week_minutes: u32,
    /// Recommended payoff order.
    pub recommendations: Vec<PayoffRecommendation>,
}

/// A recommendation for paying off a specific debt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoffRecommendation {
    /// Which debt to pay off.
    pub debt_id: TemporalId,
    /// Current cost to pay it off (minutes).
    pub current_cost_minutes: u32,
    /// What it will cost if left unpaid (minutes).
    pub future_cost_if_unpaid: u32,
    /// Return on investment of paying now vs. later.
    pub roi_percentage: f64,
}

impl TemporalDebt {
    /// Create a new temporal debt with default 10% interest rate and 7-day compound period.
    pub fn new(description: &str, principal_minutes: u32, category: DebtCategory) -> Self {
        let now = Utc::now();
        Self {
            id: TemporalId::new(),
            description: description.to_string(),
            principal_minutes,
            interest_rate: 0.1,
            compound_period_secs: 7 * 24 * 3600, // 7 days
            incurred_at: now,
            current_total_minutes: principal_minutes,
            blocking: Vec::new(),
            category,
            paid_at: None,
        }
    }

    /// Calculate the current total debt including compound interest.
    ///
    /// Uses the formula: P * (1 + r)^n where:
    /// - P = principal_minutes
    /// - r = interest_rate per compound period
    /// - n = number of compound periods elapsed
    pub fn calculate_current_total(&self) -> u32 {
        if self.paid_at.is_some() {
            return self.current_total_minutes;
        }

        let elapsed_secs = (Utc::now() - self.incurred_at).num_seconds();
        if elapsed_secs <= 0 || self.compound_period_secs <= 0 {
            return self.principal_minutes;
        }

        let periods = elapsed_secs as f64 / self.compound_period_secs as f64;
        let total = self.principal_minutes as f64 * (1.0 + self.interest_rate).powf(periods);
        total.round() as u32
    }

    /// Mark this debt as paid at the current timestamp.
    pub fn pay(&mut self) {
        self.current_total_minutes = self.calculate_current_total();
        self.paid_at = Some(Utc::now());
    }
}

// ─── 8. Chrono-Gravity ─────────────────────────────────────────────────────────

/// A high-mass temporal event that pulls other items toward it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoGravity {
    /// Unique identifier.
    pub id: TemporalId,
    /// The event exerting gravitational pull.
    pub event_id: TemporalId,
    /// Gravitational mass (importance/urgency).
    pub mass: f64,
    /// Radius of influence in days.
    pub radius_days: u32,
    /// Point of no return — after this, everything gets pulled in.
    pub event_horizon: Option<DateTime<Utc>>,
    /// Items being attracted.
    pub attracted_items: Vec<AttractedItem>,
    /// Time dilation factor near this event.
    pub time_dilation_factor: f64,
}

/// An item being pulled by chrono-gravity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractedItem {
    /// The item being attracted.
    pub item_id: TemporalId,
    /// Type of the item.
    pub item_type: String,
    /// How strongly it is being pulled (0.0-1.0).
    pub pull_strength: f64,
    /// Whether this item will reach the event in time.
    pub will_reach_event: bool,
}

/// A conflict when two gravity wells pull the same item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityConflict {
    /// The item being pulled in two directions.
    pub item_id: TemporalId,
    /// Gravity wells pulling this item.
    pub pulled_by: Vec<TemporalId>,
    /// Whether this conflict needs resolution.
    pub resolution_needed: bool,
    /// Suggested resolution.
    pub suggested_resolution: String,
}

// ─── 9. Temporal Entanglement ──────────────────────────────────────────────────

/// A quantum-like link between temporal entities that must move together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEntanglement {
    /// Unique identifier.
    pub id: TemporalId,
    /// Entangled entities.
    pub entities: Vec<TemporalId>,
    /// How they are entangled.
    pub entanglement_type: EntanglementType,
    /// Whether this entanglement is still active.
    pub active: bool,
    /// When this entanglement was created.
    pub created_at: DateTime<Utc>,
    /// Notification channels for entanglement violations.
    pub notify: Vec<String>,
}

/// How two entities are temporally entangled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntanglementType {
    /// Move together — if one shifts, the other shifts identically.
    Sync,
    /// Must happen in sequence with a fixed gap.
    Sequence {
        /// Gap between events in seconds.
        gap_secs: i64,
    },
    /// Mirror — if one moves forward, the other moves backward.
    Mirror,
    /// Inverse — if one succeeds, the other fails.
    Inverse,
    /// If one fails, the failure propagates.
    FailurePropagation,
}

// ─── PROTECTION (10-12) ────────────────────────────────────────────────────────

// ─── 10. Temporal Immune System ────────────────────────────────────────────────

/// An anomaly detected in the temporal graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnomaly {
    /// Unique identifier.
    pub id: TemporalId,
    /// Type of anomaly.
    pub anomaly_type: AnomalyType,
    /// Severity (0.0-1.0).
    pub severity: f64,
    /// Entities involved in this anomaly.
    pub involved_entities: Vec<TemporalId>,
    /// Human-readable description.
    pub description: String,
    /// Possible resolutions.
    pub resolutions: Vec<Resolution>,
    /// When the anomaly was detected.
    pub detected_at: DateTime<Utc>,
}

/// Classification of temporal anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnomalyType {
    /// A circular dependency in the temporal graph.
    DependencyCycle,
    /// Too many items scheduled in the same period.
    CapacityOverflow,
    /// An effect precedes its cause.
    CausalityViolation,
    /// Conflicting entanglements.
    EntanglementConflict,
    /// Two gravity wells fighting over the same item.
    GravityConflict,
    /// A deadline that cannot possibly be met.
    ImpossibleDeadline,
}

/// A possible resolution for an anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    /// What action to take.
    pub action: String,
    /// Whether this action fully removes the anomaly.
    pub removes_anomaly: bool,
    /// Side effects of this resolution.
    pub side_effects: Vec<String>,
}

/// Overall immune system health status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneStatus {
    /// Number of active anomalies.
    pub anomaly_count: u32,
    /// All detected anomalies.
    pub anomalies: Vec<TemporalAnomaly>,
    /// When the last scan was performed.
    pub last_scan: DateTime<Utc>,
    /// Overall health score (0.0-1.0, higher is healthier).
    pub health_score: f64,
}

// ─── 11. Decay Reversal ────────────────────────────────────────────────────────

/// Options for reversing temporal decay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayReversal {
    /// Unique identifier.
    pub id: TemporalId,
    /// The decay model to reverse.
    pub decay_id: TemporalId,
    /// Current decayed value.
    pub current_value: f64,
    /// Target value to restore to.
    pub target_value: f64,
    /// Available reversal options.
    pub options: Vec<ReversalOption>,
    /// Index of the recommended option (if any).
    pub recommended: Option<usize>,
}

/// A specific option for reversing decay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalOption {
    /// Name of this reversal strategy.
    pub name: String,
    /// Time required to complete the reversal (seconds).
    pub time_required_secs: i64,
    /// Value after reversal is applied.
    pub restored_value: f64,
    /// Steps to complete this reversal.
    pub steps: Vec<String>,
    /// How long the reversal will last before decay resumes (seconds).
    pub durability_secs: i64,
}

// ─── 12. Time Dilation Awareness ───────────────────────────────────────────────

/// A model of how time perception varies by activity and energy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDilationModel {
    /// Unique identifier.
    pub id: TemporalId,
    /// How different activities dilate time.
    pub activity_factors: Vec<ActivityDilation>,
    /// Energy level curve over the day: (hour, energy_level).
    pub energy_curve: Vec<(u32, f64)>,
    /// Weekly productivity pattern: (day_name, productivity_factor).
    pub weekly_pattern: Vec<(String, f64)>,
}

/// How a specific activity dilates perceived time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDilation {
    /// Name of the activity.
    pub activity: String,
    /// Dilation factor (>1.0 = time flies, <1.0 = time drags).
    pub dilation_factor: f64,
    /// Energy cost (0.0-1.0).
    pub energy_cost: f64,
    /// Best hours of the day for this activity (0-23).
    pub optimal_hours: Vec<u32>,
}

/// A report on subjective time for a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectiveTimeReport {
    /// Start of the period.
    pub period_start: DateTime<Utc>,
    /// End of the period.
    pub period_end: DateTime<Utc>,
    /// Actual wall-clock hours.
    pub clock_hours: f64,
    /// Perceived productive hours.
    pub perceived_productive_hours: f64,
    /// Perceived wasted hours.
    pub perceived_wasted_hours: f64,
    /// Total perceived hours (productive + wasted, may differ from clock).
    pub total_perceived_hours: f64,
    /// Recommendations for improving time perception.
    pub recommendations: Vec<String>,
}

// ─── EntityType extensions needed ──────────────────────────────────────────────
//
// The following new EntityType variants will be needed in `file_format::EntityType`
// to store these inventions in `.atime` files:
//
//   BranchPoint      = 10,
//   TimelineFork     = 11,
//   TemporalClone    = 12,
//   CloneRace        = 13,
//   TemporalEcho     = 14,
//   DeadlineProphecy = 15,
//   CausalExcavation = 16,
//   FutureMemory     = 17,
//   TemporalDebt     = 18,
//   ChronoGravity    = 19,
//   TemporalEntanglement = 20,
//   TemporalAnomaly  = 21,
//   ImmuneStatus     = 22,
//   DecayReversal    = 23,
//   TimeDilationModel = 24,
//   SubjectiveTimeReport = 25,
//
//   TIME TRAVEL INVENTIONS (26-35):
//   TemporalJump     = 26,
//   JumpPoint        = 27,
//   TemporalAnchor   = 28,
//   StateSnapshot    = 29,
//   TimeLoop         = 30,
//   LoopPattern      = 31,
//   LoopBreaker      = 32,
//   TemporalWormhole = 33,
//   WormholeEndpoint = 34,
//   WormholeInsight  = 35,

// ─── TIME TRAVEL INVENTIONS (13-16) ─────────────────────────────────────────────

// ─── 13. Temporal Jump ──────────────────────────────────────────────────────────

/// A cognitive teleportation to any point in a project's timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalJump {
    /// Jump identifier.
    pub id: TemporalId,
    /// Current temporal position.
    pub current_position: DateTime<Utc>,
    /// Origin (where we jumped from).
    pub origin: DateTime<Utc>,
    /// What was known at this time.
    pub known_facts: Vec<KnownFact>,
    /// What was unknown (hindsight from future).
    pub unknown_facts: Vec<UnknownFact>,
    /// Breadcrumb trail (for returning).
    pub jump_history: Vec<JumpPoint>,
    /// Is this an active jump?
    pub active: bool,
    /// Created at.
    pub created_at: DateTime<Utc>,
}

/// A fact that was known at a given point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownFact {
    /// The fact.
    pub fact: String,
    /// Source of the fact.
    pub source: String,
    /// Confidence at the time.
    pub confidence_at_time: f64,
}

/// A fact that was NOT known at a given point (discovered later).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownFact {
    /// What wasn't known.
    pub fact: String,
    /// When it became known.
    pub discovered_at: DateTime<Utc>,
    /// Impact of not knowing.
    pub impact: String,
    /// Signals that could have revealed it.
    pub missed_signals: Vec<String>,
}

/// A point in the jump history breadcrumb trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpPoint {
    /// The timestamp jumped to.
    pub timestamp: DateTime<Utc>,
    /// When the jump was made.
    pub jumped_at: DateTime<Utc>,
    /// Why the jump was made.
    pub reason: String,
}

impl TemporalJump {
    /// Create a new temporal jump to a destination time.
    pub fn new(destination: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: TemporalId::new(),
            current_position: destination,
            origin: now,
            known_facts: Vec::new(),
            unknown_facts: Vec::new(),
            jump_history: vec![JumpPoint {
                timestamp: destination,
                jumped_at: now,
                reason: "Initial jump".to_string(),
            }],
            active: true,
            created_at: now,
        }
    }

    /// Return to origin time.
    pub fn return_to_origin(&mut self) {
        self.active = false;
        self.jump_history.push(JumpPoint {
            timestamp: self.origin,
            jumped_at: Utc::now(),
            reason: "Return to origin".to_string(),
        });
        self.current_position = self.origin;
    }
}

// ─── 14. Temporal Anchors ───────────────────────────────────────────────────────

/// A save point in time that captures full cognitive context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnchor {
    /// Anchor identifier.
    pub id: TemporalId,
    /// Human-readable name.
    pub name: String,
    /// When anchor was created.
    pub anchored_at: DateTime<Utc>,
    /// Full state snapshot.
    pub state_snapshot: StateSnapshot,
    /// Why this anchor was created.
    pub reason: String,
    /// Tags for organization.
    pub tags: Vec<String>,
    /// Expiration (optional).
    pub expires_at: Option<DateTime<Utc>>,
    /// Times this anchor has been jumped to.
    pub jump_count: u32,
    /// Last jumped to.
    pub last_jumped_at: Option<DateTime<Utc>>,
}

/// A captured state snapshot at an anchor point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// Codebase understanding at this point.
    pub codebase_state: Option<String>,
    /// Memory/decisions at this point.
    pub memory_state: Option<String>,
    /// Active temporal entity IDs.
    pub active_deadlines: Vec<TemporalId>,
    /// Active schedule IDs.
    pub active_schedules: Vec<TemporalId>,
    /// Active sequence IDs.
    pub active_sequences: Vec<TemporalId>,
    /// Decay model values at snapshot time.
    pub decay_values: Vec<(TemporalId, f64)>,
    /// User's mental model description.
    pub mental_model: Option<String>,
    /// Integrity checksum.
    pub checksum: String,
}

/// Result of jumping to an anchor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorJumpResult {
    /// The anchor we jumped to.
    pub anchor_id: TemporalId,
    /// Anchor name.
    pub anchor_name: String,
    /// When anchor was created.
    pub anchored_at: DateTime<Utc>,
    /// Knowledge preserved from the future (what we learned since anchor).
    pub future_knowledge: Vec<String>,
    /// What's different between anchor and now.
    pub divergence_summary: String,
}

impl TemporalAnchor {
    /// Create a new temporal anchor.
    pub fn new(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            id: TemporalId::new(),
            name: name.into(),
            anchored_at: Utc::now(),
            state_snapshot: StateSnapshot {
                codebase_state: None,
                memory_state: None,
                active_deadlines: Vec::new(),
                active_schedules: Vec::new(),
                active_sequences: Vec::new(),
                decay_values: Vec::new(),
                mental_model: None,
                checksum: String::new(),
            },
            reason: reason.into(),
            tags: Vec::new(),
            expires_at: None,
            jump_count: 0,
            last_jumped_at: None,
        }
    }

    /// Check if anchor has expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|exp| Utc::now() > exp).unwrap_or(false)
    }

    /// Record a jump to this anchor.
    pub fn record_jump(&mut self) {
        self.jump_count += 1;
        self.last_jumped_at = Some(Utc::now());
    }
}

// ─── 15. Time Loops ─────────────────────────────────────────────────────────────

/// A detected repetitive pattern that wastes time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLoop {
    /// Loop identifier.
    pub id: TemporalId,
    /// Pattern signature.
    pub pattern: LoopPattern,
    /// Detected iterations.
    pub iterations: Vec<LoopIteration>,
    /// Total time spent in this loop (seconds).
    pub total_time_spent_secs: i64,
    /// Is the loop currently active?
    pub active: bool,
    /// Root cause analysis.
    pub root_cause: Option<RootCauseAnalysis>,
    /// Suggested loop breaker.
    pub breaker: Option<LoopBreaker>,
    /// When this loop was first detected.
    pub detected_at: DateTime<Utc>,
}

/// The repeating pattern within a time loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopPattern {
    /// Actions that repeat.
    pub repeating_actions: Vec<String>,
    /// What triggers each iteration.
    pub trigger: String,
    /// Expected outcome (never achieved).
    pub expected_outcome: String,
    /// Actual outcome (repeated).
    pub actual_outcome: String,
    /// Similarity score for detection (0.0-1.0).
    pub similarity_score: f64,
}

/// A single iteration within a time loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopIteration {
    /// Iteration number.
    pub number: u32,
    /// When this iteration started.
    pub started_at: DateTime<Utc>,
    /// When this iteration ended.
    pub ended_at: Option<DateTime<Utc>>,
    /// What was tried.
    pub action_taken: String,
    /// What happened.
    pub result: String,
    /// Duration in seconds.
    pub duration_secs: i64,
}

/// Root cause analysis for why a loop persists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCauseAnalysis {
    /// The actual root cause.
    pub root_cause: String,
    /// Why the repeated fix doesn't work.
    pub why_fix_fails: String,
    /// Evidence for this analysis.
    pub evidence: Vec<String>,
    /// Confidence in analysis (0.0-1.0).
    pub confidence: f64,
}

/// A recommendation for breaking a time loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopBreaker {
    /// Recommended action to break the loop.
    pub action: String,
    /// Why this will break the loop.
    pub rationale: String,
    /// Estimated time to implement (seconds).
    pub estimated_time_secs: i64,
    /// Expected outcome after breaking.
    pub expected_outcome: String,
    /// Confidence this will work (0.0-1.0).
    pub confidence: f64,
}

impl TimeLoop {
    /// Create a new time loop detection.
    pub fn new(pattern: LoopPattern) -> Self {
        Self {
            id: TemporalId::new(),
            pattern,
            iterations: Vec::new(),
            total_time_spent_secs: 0,
            active: true,
            root_cause: None,
            breaker: None,
            detected_at: Utc::now(),
        }
    }

    /// Add an iteration to this loop.
    pub fn add_iteration(&mut self, iteration: LoopIteration) {
        self.total_time_spent_secs += iteration.duration_secs;
        self.iterations.push(iteration);
    }

    /// Get the number of iterations.
    pub fn iteration_count(&self) -> u32 {
        self.iterations.len() as u32
    }

    /// Mark loop as broken.
    pub fn mark_broken(&mut self) {
        self.active = false;
    }
}

// ─── 16. Temporal Wormholes ─────────────────────────────────────────────────────

/// A bidirectional connection between two distant points in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalWormhole {
    /// Wormhole identifier.
    pub id: TemporalId,
    /// First endpoint (usually earlier).
    pub endpoint_a: WormholeEndpoint,
    /// Second endpoint (usually later).
    pub endpoint_b: WormholeEndpoint,
    /// Causal relationship description.
    pub relationship: String,
    /// Insights transmitted through wormhole.
    pub transmitted_insights: Vec<WormholeInsight>,
    /// Is wormhole active?
    pub active: bool,
    /// Created at.
    pub created_at: DateTime<Utc>,
    /// Times queried.
    pub query_count: u32,
}

/// One end of a temporal wormhole.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormholeEndpoint {
    /// Point in time.
    pub timestamp: DateTime<Utc>,
    /// Label for this endpoint.
    pub label: String,
    /// What happened here.
    pub event_description: String,
    /// Related temporal entities.
    pub related_entities: Vec<TemporalId>,
    /// Context at this point.
    pub context_summary: String,
}

/// An insight transmitted through a wormhole.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormholeInsight {
    /// Direction of insight.
    pub direction: InsightDirection,
    /// The insight content.
    pub insight: String,
    /// When this insight was transmitted.
    pub transmitted_at: DateTime<Utc>,
    /// Impact of this insight.
    pub impact: String,
}

/// Direction of information flow through a wormhole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InsightDirection {
    /// Past informing future (understanding why).
    PastToFuture,
    /// Future informing past (hindsight annotation).
    FutureToPast,
    /// Bidirectional.
    Bidirectional,
}

impl TemporalWormhole {
    /// Create a new temporal wormhole connecting two time points.
    pub fn new(
        endpoint_a: WormholeEndpoint,
        endpoint_b: WormholeEndpoint,
        relationship: impl Into<String>,
    ) -> Self {
        Self {
            id: TemporalId::new(),
            endpoint_a,
            endpoint_b,
            relationship: relationship.into(),
            transmitted_insights: Vec::new(),
            active: true,
            created_at: Utc::now(),
            query_count: 0,
        }
    }

    /// Add an insight to the wormhole.
    pub fn transmit_insight(
        &mut self,
        direction: InsightDirection,
        insight: String,
        impact: String,
    ) {
        self.transmitted_insights.push(WormholeInsight {
            direction,
            insight,
            transmitted_at: Utc::now(),
            impact,
        });
    }

    /// Record a query through this wormhole.
    pub fn record_query(&mut self) {
        self.query_count += 1;
    }

    /// Close the wormhole.
    pub fn close(&mut self) {
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;

    #[test]
    fn test_temporal_debt_new_defaults() {
        let debt = TemporalDebt::new("Write unit tests", 60, DebtCategory::Testing);
        assert_eq!(debt.principal_minutes, 60);
        assert_eq!(debt.current_total_minutes, 60);
        assert!((debt.interest_rate - 0.1).abs() < f64::EPSILON);
        assert_eq!(debt.compound_period_secs, 7 * 24 * 3600);
        assert!(debt.paid_at.is_none());
        assert!(debt.blocking.is_empty());
        assert_eq!(debt.category, DebtCategory::Testing);
    }

    #[test]
    fn test_temporal_debt_no_interest_before_first_period() {
        let debt = TemporalDebt::new("Fresh debt", 100, DebtCategory::Documentation);
        // Just created, so elapsed time is ~0 — should return principal.
        let total = debt.calculate_current_total();
        assert_eq!(total, 100);
    }

    #[test]
    fn test_temporal_debt_compound_interest_one_period() {
        let mut debt = TemporalDebt::new("Old debt", 100, DebtCategory::Refactoring);
        // Backdate by exactly one compound period (7 days).
        debt.incurred_at = Utc::now() - ChronoDuration::seconds(debt.compound_period_secs);

        let total = debt.calculate_current_total();
        // After 1 period at 10%: 100 * 1.1^1 = 110
        assert_eq!(total, 110);
    }

    #[test]
    fn test_temporal_debt_compound_interest_two_periods() {
        let mut debt = TemporalDebt::new("Older debt", 100, DebtCategory::Security);
        // Backdate by exactly two compound periods (14 days).
        debt.incurred_at = Utc::now() - ChronoDuration::seconds(2 * debt.compound_period_secs);

        let total = debt.calculate_current_total();
        // After 2 periods at 10%: 100 * 1.1^2 = 121
        assert_eq!(total, 121);
    }

    #[test]
    fn test_temporal_debt_pay_freezes_total() {
        let mut debt = TemporalDebt::new("Pay me", 100, DebtCategory::Performance);
        // Backdate by one period.
        debt.incurred_at = Utc::now() - ChronoDuration::seconds(debt.compound_period_secs);

        debt.pay();
        assert!(debt.paid_at.is_some());
        assert_eq!(debt.current_total_minutes, 110);

        // After paying, calculate_current_total should return the frozen value.
        let total = debt.calculate_current_total();
        assert_eq!(total, 110);
    }

    #[test]
    fn test_temporal_debt_custom_rate() {
        let mut debt = TemporalDebt::new("Custom rate", 200, DebtCategory::Testing);
        debt.interest_rate = 0.2;
        debt.incurred_at = Utc::now() - ChronoDuration::seconds(debt.compound_period_secs);

        let total = debt.calculate_current_total();
        // 200 * 1.2^1 = 240
        assert_eq!(total, 240);
    }

    #[test]
    fn test_clone_status_derives() {
        let s1 = CloneStatus::Exploring;
        let s2 = CloneStatus::Exploring;
        assert_eq!(s1, s2);

        // Copy
        let s3 = s1;
        assert_eq!(s3, CloneStatus::Exploring);
    }

    #[test]
    fn test_entanglement_type_partial_eq() {
        let t1 = EntanglementType::Sequence { gap_secs: 3600 };
        let t2 = EntanglementType::Sequence { gap_secs: 3600 };
        let t3 = EntanglementType::Sync;
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_anomaly_type_copy() {
        let a = AnomalyType::CausalityViolation;
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn test_debt_category_copy() {
        let c = DebtCategory::Security;
        let d = c; // Copy
        assert_eq!(c, d);
    }

    // ── Time Travel Inventions (13-16) ────────────────────────────────────────

    #[test]
    fn test_temporal_jump_create_and_return() {
        let destination = Utc::now() - ChronoDuration::hours(24);
        let mut jump = TemporalJump::new(destination);
        assert!(jump.active);
        assert_eq!(jump.jump_history.len(), 1);

        jump.return_to_origin();
        assert!(!jump.active);
        assert_eq!(jump.jump_history.len(), 2);
        assert_eq!(jump.current_position, jump.origin);
    }

    #[test]
    fn test_temporal_anchor_create_and_expire() {
        let anchor = TemporalAnchor::new("pre-refactor", "Safety save before risky change");
        assert_eq!(anchor.name, "pre-refactor");
        assert_eq!(anchor.jump_count, 0);
        assert!(!anchor.is_expired());
    }

    #[test]
    fn test_temporal_anchor_record_jump() {
        let mut anchor = TemporalAnchor::new("checkpoint", "test");
        anchor.record_jump();
        anchor.record_jump();
        assert_eq!(anchor.jump_count, 2);
        assert!(anchor.last_jumped_at.is_some());
    }

    #[test]
    fn test_time_loop_add_iterations() {
        let pattern = LoopPattern {
            repeating_actions: vec!["fix null check".to_string()],
            trigger: "null pointer exception".to_string(),
            expected_outcome: "bug fixed".to_string(),
            actual_outcome: "bug returns".to_string(),
            similarity_score: 0.95,
        };
        let mut loop_det = TimeLoop::new(pattern);
        assert_eq!(loop_det.iteration_count(), 0);

        loop_det.add_iteration(LoopIteration {
            number: 1,
            started_at: Utc::now(),
            ended_at: Some(Utc::now()),
            action_taken: "Added null check".to_string(),
            result: "Bug returned after 2 hours".to_string(),
            duration_secs: 7200,
        });

        assert_eq!(loop_det.iteration_count(), 1);
        assert_eq!(loop_det.total_time_spent_secs, 7200);

        loop_det.mark_broken();
        assert!(!loop_det.active);
    }

    #[test]
    fn test_temporal_wormhole_create_and_transmit() {
        let ep_a = WormholeEndpoint {
            timestamp: Utc::now() - ChronoDuration::days(30),
            label: "Architecture decision".to_string(),
            event_description: "Chose sync calls".to_string(),
            related_entities: vec![],
            context_summary: "Early design phase".to_string(),
        };
        let ep_b = WormholeEndpoint {
            timestamp: Utc::now(),
            label: "Scaling problems".to_string(),
            event_description: "3x cost increase".to_string(),
            related_entities: vec![],
            context_summary: "Production scaling issues".to_string(),
        };
        let mut wormhole = TemporalWormhole::new(ep_a, ep_b, "Sync calls causing scale issues");
        assert!(wormhole.active);
        assert_eq!(wormhole.transmitted_insights.len(), 0);

        wormhole.transmit_insight(
            InsightDirection::FutureToPast,
            "This sync design causes 3x scaling cost".to_string(),
            "Would have saved $50k".to_string(),
        );
        assert_eq!(wormhole.transmitted_insights.len(), 1);

        wormhole.record_query();
        assert_eq!(wormhole.query_count, 1);

        wormhole.close();
        assert!(!wormhole.active);
    }

    #[test]
    fn test_insight_direction_copy() {
        let d = InsightDirection::PastToFuture;
        let e = d; // Copy
        assert_eq!(d, e);
    }
}
