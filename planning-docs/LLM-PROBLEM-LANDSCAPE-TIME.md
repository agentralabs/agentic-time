# LLM Problem Landscape and Distribution (Time Lens)

Status: Planning-only (gitignored local working document)
Scope: Global LLM coding/system problems, with AgenticTime ownership focus
Updated: 2026-02-26

## 1. Exhaustive working catalog (plain language)

| ID | Problem all LLMs face today | Plain-language explanation | Primary sister |
|---|---|---|---|
| P01 | Context-window limits | The model cannot reliably hold the full project/problem state at once. | Memory |
| P02 | Retrieval noise | Search often returns text that looks related but is not decision-useful. | Memory |
| P03 | Provenance gaps | Answers may miss exact source/trace, reducing trust and auditability. | Codebase + Memory |
| P04 | Temporal staleness | The model may use old context after code/config changed. | Time + Codebase + Memory |
| P05 | Cross-session amnesia | Important details are forgotten between sessions/restarts. | Memory |
| P06 | Contradiction persistence | Old wrong beliefs remain active and conflict with new facts. | Memory |
| P07 | Weak uncertainty calibration | Model sounds certain when confidence should be low. | Memory |
| P08 | Intent ambiguity | User requirement is under-specified; model fills gaps incorrectly. | External + Memory |
| P09 | Whole-repo topology blindness | Model understands files locally but misses global structure. | Codebase |
| P10 | Change-impact blindness | Hard to know what breaks when one unit changes. | Codebase |
| P11 | Hidden coupling | Fragile dependencies remain invisible until failure. | Codebase |
| P12 | Test-gap blindness | Missing or weak test coverage is not consistently surfaced. | Codebase |
| P13 | Refactor-safety uncertainty | Large edits are risky without structure-aware guardrails. | Codebase |
| P14 | Multi-language boundary gaps | Cross-language edges (Rust/Python/TS/etc.) are hard to reason about. | Codebase |
| P15 | Dependency/version drift | Upstream library changes silently invalidate assumptions. | Codebase |
| P16 | Build-system variance | Different build tools/profiles create inconsistent behavior. | Codebase |
| P17 | Config/env mismatch | Local/dev/prod differences cause hidden bugs. | Codebase + Memory |
| P18 | Migration risk | Schema/data migrations can cause irreversible damage if unsafe. | Codebase + External |
| P19 | Performance-regression risk | Code changes may degrade latency/cost without obvious signs. | Codebase + Vision |
| P20 | Security coverage gaps | Subtle auth/input/permission bugs remain underdetected. | Codebase + External |
| P21 | Reproducibility failure | Same prompt/code path yields non-repeatable outcomes. | Memory + Codebase |
| P22 | Spec-to-code drift | Implementation no longer matches design/intended contract. | Codebase |
| P23 | UI-state blindness | Text-only reasoning misses what users actually saw on screen. | Vision |
| P24 | Non-text signal blindness | Layout, color, interaction state are not captured by logs alone. | Vision |
| P25 | Observability gaps | Missing logs/metrics/traces block root-cause analysis. | Vision + Memory |
| P26 | Incident timeline reconstruction | Hard to rebuild exact sequence of events after failure. | Time + Memory + Vision |
| P27 | Artifact portability friction | Knowledge/state is tied to one runtime/client machine. | All sisters |
| P28 | Cloud-local divide | Cloud agents cannot directly read local artifacts without sync/auth. | All sisters |
| P29 | Auth integration friction | Secure remote execution is inconsistent across clients. | All sisters |
| P30 | Latency/cost optimization uncertainty | Hard to pick best runtime policy for quality vs speed vs cost. | Time + Memory + Codebase |
| P31 | Long-session reliability decay | Performance and quality degrade over long autonomous runs. | Time + Memory + Vision |
| P32 | Long-horizon storage governance | "Capture everything" can explode storage/cost if unmanaged. | Memory |
| P33 | Privacy/redaction control | Sensitive user/org data needs policy-aware capture. | Memory + Vision |
| P34 | Feedback incorporation lag | User corrections are not consistently merged into future behavior. | Memory |
| P35 | Multi-agent coordination drift | Multiple agents diverge on facts/tasks/contracts. | Memory + Codebase |
| P36 | Evaluation drift | Benchmarks stop reflecting real production workloads. | Time + All sisters |
| P37 | Requirement ambiguity | Stakeholders ask for outcomes without testable acceptance criteria. | External |
| P38 | Tacit business-rule knowledge | Critical rules live in people, not docs/code. | External + Memory |
| P39 | Priority conflict | Teams disagree on quality/speed/cost tradeoffs. | Time + External |
| P40 | Compliance interpretation uncertainty | Legal/policy language is hard to map safely to code behavior. | External + Codebase |
| P41 | Third-party API volatility | Vendor behavior changes without warning. | External + Memory |
| P42 | Handoff quality gaps | Context loss across teams/time zones causes repeated work. | Memory |
| P43 | Incentive misalignment | Metrics reward speed while quality risk accumulates. | Time + External |
| P44 | Explainability for non-technical stakeholders | Teams cannot explain risk/decision rationale clearly. | Codebase + Memory |

## 2. AgenticTime: primary ownership

### 2.1 Problems Time should solve directly
- P04, P26, P30, P31, P36, P39, P43
- Contributing: P05, P07, P19, P21, P25, P35, P41, P42, P44

### 2.2 Why Time is the right owner
- These are temporal reasoning/scheduling/freshness problems that require mathematical models (decay curves, PERT estimation, conflict detection) and persistent entity storage.
- They require typed temporal entities (deadlines, durations, schedules, sequences, decay) with domain-specific operations, not flat date metadata or calendar APIs.

### 2.3 Planned capability tracks (Time)
- Track T1: core temporal persistence (5 entity types in .atime binary, 19 MCP tools, BLAKE3 integrity)
- Track T2: temporal exploration (timeline forks, temporal replay, ripple-effect analysis)
- Track T3: temporal prediction (deadline prophecy, causal archaeology, future memory pre-loading)
- Track T4: temporal management (temporal debt with compounding interest, chrono-gravity priority warping, temporal entanglement)
- Track T5: temporal protection (ghost writer calendar sync, temporal anomaly detection, session time budgeting)
- Track T6: time travel (temporal jump, temporal anchors, time loop detection, temporal wormholes)

## 3. What Time cannot solve alone

- P01/P02/P05-P06/P34: requires persistent memory with typed events and confidence (Memory).
- P09-P16/P22: requires graph-grade source-code semantics (Codebase).
- P23/P24: requires direct visual signal ingestion (Vision).
- P37/P39/P43: organizational governance problems partially outside runtime temporal reasoning.

## 4. Integration contracts needed from other sisters

- From Memory:
  - decision node IDs to link temporal entities to the cognitive events that created them
  - session boundaries for temporal scope (auto-session triggers .atime context loading)
- From Codebase:
  - code unit IDs for sequence step targets (deploy pipeline steps map to code modules)
  - impact analysis results feeding into deadline prophecy risk factors
- From Vision:
  - scheduled visual captures triggered by Time's schedule entities
  - visual evidence timestamps for incident timeline reconstruction
- From Identity:
  - signed receipts for temporal operations (deadline creation, schedule changes)
  - trust grants gating temporal actions (only authorized agents can modify production deadlines)

## 5. Acceptance signal for this planning document

This document is complete when Time roadmap items map to catalog IDs and each ID has an owner (Time, sister, or external).
