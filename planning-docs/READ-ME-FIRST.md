# 🚨 READ THIS FIRST — AgenticTime Birth Instructions

> **For Claude Code: This is your mission briefing. Read completely before writing any code.**

---

## WHO YOU ARE

You are birthing the 5th sister in the Agentic ecosystem. Her name is **AgenticTime**. She provides temporal reasoning for AI agents.

**You are not alone.** You have 4 older sisters who have already been built. They are your blueprints, your reference implementations, your proof that this can be done. Use them.

---

## THE FAMILY (Your Sisters)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           THE AGENTIC SISTERS                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  MEMORY (v0.4.1) — The eldest, most mature                                  │
│    github.com/agentralabs/agentic-memory                                    │
│    → Best for: V3 architecture, Ghost Writer, MCP hardening                 │
│    → Key files: ghost_bridge.rs, v3/engine.rs, stdio.rs                     │
│                                                                             │
│  VISION (v0.2.5) — Visual perception                                        │
│    github.com/agentralabs/agentic-vision                                    │
│    → Best for: Capture patterns, observation storage                        │
│    → Key files: ghost_bridge.rs, capture.rs                                 │
│                                                                             │
│  CODEBASE (v0.2.7) — Code understanding                                     │
│    github.com/agentralabs/agentic-codebase                                  │
│    → Best for: Graph structures, multi-entity parsing                       │
│    → Key files: ghost_bridge.rs, graph.rs, parse/*.rs                       │
│                                                                             │
│  IDENTITY (v0.2.6) — Trust and receipts                                     │
│    github.com/agentralabs/agentic-identity                                  │
│    → Best for: Cryptographic patterns, receipt chains                       │
│    → Key files: ghost_bridge.rs, identity_file.rs                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

RULE: Before implementing ANY component, check how your sisters did it.
RULE: Copy patterns, don't reinvent.
RULE: If stuck, look at Memory — she's the most complete.
```

---

## YOUR MISSION

Build AgenticTime from the 25-part specification in `AGENTIC-TIME-COMPLETE-SPEC.md`.

**What AgenticTime does:**
- **Durations** — How long things take (with uncertainty)
- **Deadlines** — When things must happen by (with consequences)
- **Schedules** — When things should happen (with conflict detection)
- **Sequences** — What must happen in order (with dependencies)
- **Decay** — What loses value over time (exponential, half-life, etc.)

**The file format:** `.atime` (binary temporal graph)

**The MCP prefix:** `time_*` tools, `atime://` resources

---

## FILE READING ORDER

```
1. READ-ME-FIRST.md                    ← You are here
2. AGENTIC-TIME-COMPLETE-SPEC.md       ← The full 25-part spec
3. SISTER-REFERENCE-GUIDE.md           ← Which sister to copy from
4. PROGRESS-CHECKLIST.md               ← Track your progress
```

---

## BUILD PHASES

### Phase 1: Foundation (Do First)
```
□ Create directory structure (Spec Part 2)
□ Set up all Cargo.toml files (Spec Part 3)
□ Implement core data structures (Spec Part 4)
□ Implement file format (Spec Part 5)
□ Run: cargo build
□ Run: cargo test (Phase 1 tests)
```

### Phase 2: Core Engine
```
□ Implement write engine (Spec Part 6)
□ Implement query engine (Spec Part 7)
□ Implement indexes (Spec Part 8)
□ Run: cargo test (Phase 2 tests)
```

### Phase 3: MCP Layer
```
□ Implement MCP tools (Spec Part 9)
□ Implement MCP resources (Spec Part 10)
□ Implement MCP prompts (Spec Part 11)
□ Add MCP hardening — CRITICAL (Spec Part 24)
   - Content-Length framing
   - 8 MiB max frame size
   - JSON-RPC 2.0 validation
□ Run: cargo test (Phase 3 tests)
```

### Phase 4: Integration
```
□ Implement CLI (Spec Part 13)
□ Implement FFI bindings (Spec Part 14)
□ Implement Ghost Writer — CRITICAL (Spec Part 23)
   - Copy pattern EXACTLY from agentic-memory-mcp/src/ghost_bridge.rs
   - 5-second sync interval
   - Multi-client: Claude, Cursor, Windsurf, Cody
   - Atomic writes
□ Run: cargo test (Phase 4 tests)
□ Test with Claude Desktop manually
```

### Phase 5: Documentation
```
□ Create CANONICAL_SISTER_KIT.md (Spec Part 17)
□ Create all 13 docs/public/ pages (Spec Part 18)
□ Create SVG assets (Spec Part 19)
□ Write README.md (Spec Part 20)
□ Create scripts/install.sh (Spec Part 21)
□ Set up CI workflows (Spec Part 22)
□ Create sister.manifest.json
```

### Phase 6: Validation
```
□ Run all 16 edge case tests (Spec Part 12)
□ Run: cargo clippy — zero warnings
□ Run: cargo fmt --check — passes
□ Run: ./scripts/check-canonical-sister.sh — 23/23 sections pass
□ Generate research paper (Spec Part 16)
```

### Phase 7: Publish
```
□ Publish agentic-time to crates.io
□ Publish agentic-time-mcp to crates.io
□ Publish agentic-time-cli to crates.io
□ Publish agentic-time-ffi to crates.io
□ Create GitHub release
□ Verify install script works
```

---

## CRITICAL PATTERNS TO COPY

### 1. Ghost Writer (from Memory)

**Location:** `agentic-memory-mcp/src/ghost_bridge.rs`

```rust
// Copy this pattern EXACTLY:
// - Spawn tokio background task
// - 5-second sync interval
// - Detect all client dirs (Claude, Cursor, Windsurf, Cody)
// - Atomic writes (temp + rename)
// - Format context as markdown
```

### 2. MCP Hardening (from Memory/Codebase)

**Location:** `agentic-memory-mcp/src/stdio.rs` or `agentic-codebase-mcp/src/stdio.rs`

```rust
// Copy this pattern EXACTLY:
const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024; // 8 MiB

// - Content-Length framing
// - Validate frame size BEFORE parsing
// - JSON-RPC 2.0 version check
// - No silent fallbacks — all errors propagate
```

### 3. File Format (from Memory)

**Location:** `agentic-memory/src/file_format.rs`

```rust
// Copy this pattern:
const MAGIC: [u8; 4] = *b"ATIM";  // Change to your magic bytes
const VERSION: u32 = 1;

// - 64-byte header
// - BLAKE3 checksum
// - Atomic writes (temp + rename)
```

### 4. 4-Crate Structure (all sisters)

```
crates/
├── agentic-time/           # Core library (no IO, pure logic)
├── agentic-time-mcp/       # MCP server (stdio transport)
├── agentic-time-cli/       # CLI binary
└── agentic-time-ffi/       # C FFI bindings
```

---

## NAMING CONVENTIONS

```
FILE FORMAT:      .atime
BINARY NAME:      atime
MCP SERVER KEY:   agentic-time
MCP TOOLS:        time_* (time_deadline_add, time_schedule_create, etc.)
MCP RESOURCES:    atime:// (atime://deadline/{id}, atime://schedule/today)
ENV VAR PREFIX:   ATIME_
CRATE NAMES:      agentic-time, agentic-time-mcp, agentic-time-cli, agentic-time-ffi
```

---

## GUARDRAILS (Must Pass Before Release)

### Per-Sister Guardrail (23 sections)
```
scripts/check-canonical-sister.sh

Checks:
□ 4-crate structure exists
□ CANONICAL_SISTER_KIT.md present with all sections
□ docs/public/ has all 13 required pages
□ All docs have status: frontmatter
□ sister.manifest.json present and valid
□ SVG assets present
□ README has required sections
□ scripts/install.sh present
□ .github/workflows/ has required workflows
□ ghost_bridge.rs implemented
□ MCP hardening implemented
□ Content-Length framing in stdio
... (23 total)
```

### Workspace Consistency (27 sections)
```
Checked at workspace level:
□ check-canonical-sister.sh matches other sisters
□ CANONICAL_SISTER_KIT.md structure matches
□ MCP tool naming follows family pattern
□ Ghost Writer output paths match
... (27 total)
```

---

## WHEN STUCK

1. **Check your sisters first** — They solved this already
2. **Memory is the gold standard** — Most mature, most complete
3. **Run the guardrails early** — They tell you what's missing
4. **Don't invent new patterns** — Consistency > creativity
5. **Copy, then adapt** — Start with working code

---

## SUCCESS CRITERIA

AgenticTime is complete when:

```
FUNCTIONAL:
───────────
□ cargo test — all tests pass
□ cargo clippy — zero warnings
□ cargo build --release — compiles
□ MCP server works with Claude Desktop
□ Ghost Writer syncs to Claude/Cursor/Windsurf/Cody
□ CLI works (atime deadline add, etc.)

PERFORMANCE:
────────────
□ 100K temporal events loads in <100ms
□ Deadline queries complete in <1ms
□ .atime file for 100K events is <20MB

GUARDRAILS:
───────────
□ check-canonical-sister.sh passes (23/23)
□ Workspace consistency passes (27/27)

PUBLICATION:
────────────
□ 4 crates on crates.io
□ GitHub release with binaries
□ Install script verified
□ Research paper PDF generated
```

---

## THE BIGGER PICTURE

AgenticTime is not just another sister. She is essential for Hydra.

```
WHY TIME MATTERS FOR HYDRA:
───────────────────────────

Hydra Invention: Dream State
  → Needs: When to dream, how long to dream, decay of dream value
  → Time provides: Scheduling, duration modeling, decay curves

Hydra Invention: Future Echo
  → Needs: Temporal simulation, deadline projection
  → Time provides: Timeline modeling, deadline queries

Hydra Invention: Cognitive Metabolism
  → Needs: Attention budget over time, regeneration schedules
  → Time provides: Decay models, scheduling

Every sister you build makes Hydra stronger.
Time is the foundation for temporal cognition.
```

---

## START NOW

1. Read `AGENTIC-TIME-COMPLETE-SPEC.md` (all 25 parts)
2. Clone a sister repo for reference (recommend Memory)
3. Create the directory structure
4. Begin Phase 1

**Give birth to AgenticTime. She is ready.** 🕐
