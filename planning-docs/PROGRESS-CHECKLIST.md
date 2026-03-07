# AgenticTime Build Progress Checklist

> **Track your progress. Check items as you complete them.**

---

## PHASE 1: Foundation

### Directory Structure
- [ ] Created workspace `Cargo.toml`
- [ ] Created `crates/agentic-time/` directory
- [ ] Created `crates/agentic-time-mcp/` directory
- [ ] Created `crates/agentic-time-cli/` directory
- [ ] Created `crates/agentic-time-ffi/` directory
- [ ] Created `docs/public/` directory
- [ ] Created `assets/` directory
- [ ] Created `scripts/` directory
- [ ] Created `.github/workflows/` directory
- [ ] Created `tests/` directory
- [ ] Created `python/` directory
- [ ] Created `wasm/` directory

### Cargo.toml Files
- [ ] Workspace `Cargo.toml` with all members
- [ ] `agentic-time/Cargo.toml`
- [ ] `agentic-time-mcp/Cargo.toml`
- [ ] `agentic-time-cli/Cargo.toml`
- [ ] `agentic-time-ffi/Cargo.toml`

### Core Data Structures
- [ ] `TemporalId` (UUID wrapper)
- [ ] `DurationEstimate` (with PERT calculation)
- [ ] `Deadline` (with status, consequence, completion)
- [ ] `DeadlineType` enum
- [ ] `DeadlineStatus` enum
- [ ] `Consequence` enum
- [ ] `Schedule` (with recurrence, buffer, conflicts)
- [ ] `Recurrence` struct
- [ ] `RecurrencePattern` enum
- [ ] `Priority` enum
- [ ] `ScheduleStatus` enum
- [ ] `Sequence` (with ordered steps)
- [ ] `SequenceStep` struct
- [ ] `SequenceStatus` enum
- [ ] `StepStatus` enum
- [ ] `DecayModel` (with multiple decay types)
- [ ] `DecayType` enum
- [ ] `TimeError` enum
- [ ] `TimeResult<T>` type alias

### File Format
- [ ] `MAGIC` constant (`b"ATIM"`)
- [ ] `VERSION` constant
- [ ] `FileHeader` struct (64 bytes)
- [ ] `EntityType` enum
- [ ] `EntityBlock` struct
- [ ] `TimeFile` struct
- [ ] `FileHeader::write_to()`
- [ ] `FileHeader::read_from()`
- [ ] `EntityBlock::write_to()`
- [ ] `EntityBlock::read_from()`
- [ ] `TimeFile::create()`
- [ ] `TimeFile::open()`
- [ ] `TimeFile::save()` (atomic write)
- [ ] `TimeFile::add()`
- [ ] `TimeFile::get()`
- [ ] `TimeFile::remove()`
- [ ] `TimeFile::list_by_type()`

### Phase 1 Verification
- [ ] `cargo build` succeeds
- [ ] Phase 1 unit tests pass

---

## PHASE 2: Core Engine

### Write Engine
- [ ] `WriteEngine` struct
- [ ] `add_duration()`
- [ ] `update_duration()`
- [ ] `add_deadline()`
- [ ] `update_deadline()`
- [ ] `complete_deadline()`
- [ ] `cancel_deadline()`
- [ ] `add_schedule()`
- [ ] `update_schedule()`
- [ ] `reschedule()`
- [ ] `check_schedule_conflicts()`
- [ ] `add_sequence()`
- [ ] `update_sequence()`
- [ ] `advance_sequence()`
- [ ] `add_decay()`
- [ ] `update_decay()`
- [ ] `refresh_decay()`
- [ ] `update_all_statuses()`
- [ ] `UpdateReport` struct

### Query Engine
- [ ] `QueryEngine` struct
- [ ] `overdue_deadlines()`
- [ ] `deadlines_due_within()`
- [ ] `deadlines_by_status()`
- [ ] `schedules_in_range()`
- [ ] `schedule_conflicts()`
- [ ] `available_slots()`
- [ ] `sequence()`
- [ ] `active_sequences()`
- [ ] `blocked_sequences()`
- [ ] `decays_below_threshold()`
- [ ] `decays_reaching_threshold()`
- [ ] `estimate_total()`
- [ ] `stats()`
- [ ] `TimeSlot` struct
- [ ] `TimeStats` struct

### Indexes
- [ ] `DeadlineIndex` struct
- [ ] `DeadlineIndex::insert()`
- [ ] `DeadlineIndex::remove()`
- [ ] `DeadlineIndex::due_before()`
- [ ] `DeadlineIndex::due_in_range()`
- [ ] `DeadlineIndex::next_deadline()`
- [ ] `SequenceIndex` struct
- [ ] `SequenceIndex::insert()`
- [ ] `SequenceIndex::update()`
- [ ] `SequenceIndex::in_progress()`
- [ ] `DecayIndex` struct
- [ ] `DecayIndex::insert()`
- [ ] `DecayIndex::below_threshold()`
- [ ] `DecayIndex::lowest()`

### Phase 2 Verification
- [ ] `cargo build` succeeds
- [ ] Phase 2 unit tests pass

---

## PHASE 3: MCP Layer

### MCP Tools (Part 9)
- [ ] `ToolDefinition` struct
- [ ] `TOOLS` constant array
- [ ] `time_deadline_add` tool
- [ ] `time_deadline_complete` tool
- [ ] `time_deadline_list` tool
- [ ] `time_deadline_overdue` tool
- [ ] `time_schedule_create` tool
- [ ] `time_schedule_reschedule` tool
- [ ] `time_schedule_range` tool
- [ ] `time_schedule_available` tool
- [ ] `time_schedule_conflicts` tool
- [ ] `time_sequence_create` tool
- [ ] `time_sequence_advance` tool
- [ ] `time_sequence_status` tool
- [ ] `time_decay_create` tool
- [ ] `time_decay_value` tool
- [ ] `time_decay_alert` tool
- [ ] `time_duration_estimate` tool
- [ ] `time_duration_aggregate` tool
- [ ] `time_stats` tool
- [ ] `time_refresh` tool
- [ ] Tool handler dispatch function

### MCP Resources (Part 10)
- [ ] `ResourceDefinition` struct
- [ ] `list_resources()` function
- [ ] `read_resource()` function
- [ ] `atime://deadline/` resource
- [ ] `atime://deadline/overdue` resource
- [ ] `atime://deadline/upcoming` resource
- [ ] `atime://schedule/` resource
- [ ] `atime://schedule/today` resource
- [ ] `atime://schedule/week` resource
- [ ] `atime://sequence/` resource
- [ ] `atime://sequence/active` resource
- [ ] `atime://decay/` resource
- [ ] `atime://decay/critical` resource
- [ ] `atime://stats` resource

### MCP Prompts (Part 11)
- [ ] `PromptDefinition` struct
- [ ] `PromptArgument` struct
- [ ] `list_prompts()` function
- [ ] `expand_prompt()` function
- [ ] `time_plan` prompt
- [ ] `time_review` prompt
- [ ] `time_estimate` prompt
- [ ] `time_schedule_day` prompt

### MCP Hardening (Part 24) — CRITICAL
- [ ] `MAX_CONTENT_LENGTH_BYTES` constant (8 MiB)
- [ ] `StdioTransport` struct
- [ ] `read_message()` with Content-Length framing
- [ ] Size validation BEFORE reading body
- [ ] `write_message()` with Content-Length header
- [ ] `validate_jsonrpc()` function
- [ ] `TransportError` enum
- [ ] No silent fallbacks anywhere

### MCP Server
- [ ] `main.rs` entry point
- [ ] Server initialization
- [ ] Tool call handling
- [ ] Resource read handling
- [ ] Prompt handling
- [ ] Error responses (JSON-RPC format)
- [ ] Graceful shutdown (SIGTERM)

### Phase 3 Verification
- [ ] `cargo build` succeeds
- [ ] MCP server starts (`agentic-time-mcp serve`)
- [ ] Phase 3 tests pass

---

## PHASE 4: Integration

### CLI (Part 13)
- [ ] `main.rs` with Clap
- [ ] `Commands` enum
- [ ] `deadline` subcommand
- [ ] `schedule` subcommand
- [ ] `sequence` subcommand
- [ ] `decay` subcommand
- [ ] `duration` subcommand
- [ ] `stats` subcommand
- [ ] `refresh` subcommand
- [ ] `info` subcommand
- [ ] `install` subcommand
- [ ] `serve` subcommand (delegates to MCP)

### FFI Bindings (Part 14)
- [ ] `lib.rs` with FFI functions
- [ ] `AtimeError` C enum
- [ ] `atime_open()` function
- [ ] `atime_create()` function
- [ ] `atime_close()` function
- [ ] `atime_save()` function
- [ ] `atime_deadline_add()` function
- [ ] `atime_deadline_complete()` function
- [ ] `atime_deadlines_overdue()` function
- [ ] `atime_stats()` function
- [ ] `include/agentic_time.h` header file

### Ghost Writer (Part 23) — CRITICAL
- [ ] `ghost_bridge.rs` module
- [ ] `detect_all_memory_dirs()` function
- [ ] Client detection: Claude (`~/.claude/memory/`)
- [ ] Client detection: Cursor (`~/.cursor/memory/`)
- [ ] Client detection: Windsurf (`~/.windsurf/memory/`)
- [ ] Client detection: Cody (`~/.sourcegraph/cody/memory/`)
- [ ] `format_time_context()` function
- [ ] Context includes: overdue deadlines
- [ ] Context includes: today's schedule
- [ ] Context includes: active sequences
- [ ] Context includes: critical decays
- [ ] `spawn_ghost_writer()` function
- [ ] Tokio background task
- [ ] 5-second sync interval
- [ ] Atomic writes (temp + rename)
- [ ] Different filenames per client

### Phase 4 Verification
- [ ] `cargo build --release` succeeds
- [ ] CLI works: `atime stats`
- [ ] CLI works: `atime deadline add "Test" --due "2025-03-01"`
- [ ] Ghost Writer creates `~/.claude/memory/TIME_CONTEXT.md`
- [ ] Ghost Writer updates every 5 seconds
- [ ] MCP server tested with Claude Desktop

---

## PHASE 5: Documentation

### CANONICAL_SISTER_KIT.md (Part 17)
- [ ] Created file
- [ ] Section 1: Release Artifact Contract
- [ ] Section 2: Install Contract Spec
- [ ] Section 3: Reusable CI Guardrails
- [ ] Section 4: README Canonical Layout
- [ ] Section 5: MCP Canonical Profile
- [ ] Section 6: Packaging Policy
- [ ] Section 7: Versioning and Release Policy
- [ ] Section 8: Design Asset Contract
- [ ] Section 9: Env Var Namespace Contract
- [ ] Section 10: New-Sister Bootstrap
- [ ] Section 11: Workspace Orchestrator Contract

### docs/public/ Pages (Part 18)
- [ ] `overview.md` with `status: published` frontmatter
- [ ] `quickstart.md` with frontmatter
- [ ] `installation.md` with frontmatter
- [ ] `command-surface.md` with frontmatter
- [ ] `experience-with-vs-without.md` with frontmatter
- [ ] `runtime-install-sync.md` with frontmatter
- [ ] `integration-guide.md` with frontmatter
- [ ] `concepts.md` with frontmatter
- [ ] `api-reference.md` with frontmatter
- [ ] `file-format.md` with frontmatter
- [ ] `benchmarks.md` with frontmatter
- [ ] `faq.md` with frontmatter
- [ ] `playbooks-agent-integration.md` with frontmatter

### Assets (Part 19)
- [ ] `assets/github-hero-pane.svg`
- [ ] `assets/architecture.svg`
- [ ] `assets/benchmarks.svg`

### README.md (Part 20)
- [ ] Hero image reference
- [ ] Badge row (crates.io, tests)
- [ ] Navigation links
- [ ] "The Problem" section
- [ ] "Install" section
- [ ] "Quickstart" section
- [ ] "How It Works" section
- [ ] "MCP Tools" table
- [ ] "Benchmarks" section

### Install Script (Part 21)
- [ ] `scripts/install.sh` created
- [ ] Profile-based install (desktop/terminal/server)
- [ ] Release artifact download
- [ ] Cargo install fallback
- [ ] MCP config merge (not overwrite)
- [ ] Completion block output
- [ ] Executable permission (`chmod +x`)

### CI Workflows (Part 22)
- [ ] `.github/workflows/ci.yml`
- [ ] `.github/workflows/release.yml`
- [ ] `.github/workflows/canonical-sister-guardrails.yml`
- [ ] `.github/workflows/install-command-guardrails.yml`

### Guardrail Scripts
- [ ] `scripts/check-canonical-sister.sh`
- [ ] `scripts/check-install-commands.sh`
- [ ] Both executable

### Manifest
- [ ] `sister.manifest.json` created
- [ ] `sister_key: "time"`
- [ ] `sister_name: "AgenticTime"`
- [ ] `page_ids` array with all docs

### Python/WASM/npm (Part 25)
- [ ] `python/pyproject.toml`
- [ ] `python/src/agentic_time/__init__.py`
- [ ] `wasm/Cargo.toml`
- [ ] `wasm/src/lib.rs`
- [ ] `package.json` at root

---

## PHASE 6: Validation

### Edge Cases (Part 12)
- [ ] Test 1: Invalid date-time formats → rejection
- [ ] Test 2: Malformed JSON → error -32700
- [ ] Test 3: Oversized input (>8 MiB) → rejection
- [ ] Test 4: Invalid temporal IDs → 404
- [ ] Test 5: Negative durations → rejection
- [ ] Test 6: Empty file → empty arrays
- [ ] Test 7: Corrupted .atime file → error with hint
- [ ] Test 8: Missing file → create or clear error
- [ ] Test 9: Timezone handling → UTC internal
- [ ] Test 10: Date overflow → graceful handling
- [ ] Test 11: SIGTERM → clean shutdown
- [ ] Test 12: Concurrent access → file locking
- [ ] Test 13: Zero duration → valid
- [ ] Test 14: Maximum dates → no overflow
- [ ] Test 15: Empty strings → require non-empty
- [ ] Test 16: 100K entities → <100ms queries

### Code Quality
- [ ] `cargo clippy` — zero warnings
- [ ] `cargo fmt --check` — passes
- [ ] All doc comments present
- [ ] No `unwrap()` in library code
- [ ] No `println!()` in library code

### Guardrails
- [ ] `./scripts/check-canonical-sister.sh` passes (23/23)
- [ ] Workspace consistency check passes (27/27)

### Performance Benchmarks
- [ ] 100K entities load time measured
- [ ] Deadline query time measured
- [ ] File size for 100K entities measured
- [ ] Results documented in `docs/public/benchmarks.md`

### Research Paper (Part 16)
- [ ] `paper/agentictime-paper.tex` created
- [ ] Real benchmark data included
- [ ] `pdflatex` compiles successfully
- [ ] PDF is 5-10 pages
- [ ] All figures render
- [ ] All tables formatted

---

## PHASE 7: Publish

### Crates.io
- [ ] `agentic-time` published
- [ ] `agentic-time-mcp` published
- [ ] `agentic-time-cli` published
- [ ] `agentic-time-ffi` published
- [ ] All versions consistent

### GitHub
- [ ] All changes committed
- [ ] Git tag created (`v0.1.0`)
- [ ] Tag pushed
- [ ] GitHub release created
- [ ] Release binaries uploaded (macOS, Linux, Windows)

### Verification
- [ ] `cargo install agentic-time-cli` works
- [ ] Install script works: `curl -fsSL .../install/time | bash`
- [ ] MCP server registers in Claude Desktop
- [ ] Ghost Writer creates context files
- [ ] All MCP tools work from Claude

---

## FINAL VERIFICATION

```
□ cargo test               — ALL PASS
□ cargo clippy             — ZERO WARNINGS  
□ cargo fmt --check        — PASSES
□ cargo build --release    — COMPILES
□ check-canonical-sister   — 23/23 SECTIONS
□ workspace consistency    — 27/27 SECTIONS
□ Claude Desktop test      — WORKS
□ Ghost Writer             — SYNCING
□ 4 crates on crates.io    — PUBLISHED
□ GitHub release           — CREATED
```

---

## COMPLETION

**AgenticTime is COMPLETE when all boxes above are checked.**

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  🕐 AGENTIC TIME — BORN                                                   ║
║                                                                           ║
║  The 5th sister lives.                                                    ║
║  Temporal reasoning for AI agents.                                        ║
║  "When should this happen?"                                               ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```
