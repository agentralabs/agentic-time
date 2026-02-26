# AgenticTime — Claude Code Master Instructions

> **READ THIS FIRST. This is the meta-instruction for birthing AgenticTime.**

---

## WHAT YOU ARE BUILDING

AgenticTime is the 5th sister in the Agentic ecosystem. She provides temporal reasoning for AI agents — duration modeling, deadlines, scheduling, sequence constraints, and decay curves.

**The problem she solves:** AI agents have no concept of time. They can't answer "when should this happen?", "what's overdue?", "how long will this take?", or "what decays if ignored?"

**The file format:** `.atime` — a binary temporal graph

**The MCP prefix:** `time_*` tools, `atime://` resources

---

## YOUR SISTERS (USE THEM AS REFERENCE)

You have 4 older sisters. They are your blueprints. When in doubt, look at how they solved the same problem.

```
SISTER REPOS TO REFERENCE:
══════════════════════════

Memory (oldest, most mature):
  github.com/agentralabs/agentic-memory
  → Best reference for: V3 architecture, Ghost Writer, MCP hardening
  → Key files: ghost_bridge.rs, v3/engine.rs, mcp/tools.rs

Vision:
  github.com/agentralabs/agentic-vision  
  → Best reference for: Capture patterns, observation storage
  → Key files: ghost_bridge.rs, capture.rs

Codebase:
  github.com/agentralabs/agentic-codebase
  → Best reference for: Graph structures, multi-language parsing
  → Key files: ghost_bridge.rs, parse/*.rs, graph.rs

Identity:
  github.com/agentralabs/agentic-identity
  → Best reference for: Cryptographic patterns, receipts
  → Key files: ghost_bridge.rs, identity_file.rs, receipts.rs
```

**RULE: Before implementing any component, check how your sisters did it.**

---

## THE 25 SPECS (ALL IN ONE FILE)

All 25 specifications are in `AGENTIC-TIME-COMPLETE-SPEC.md`. Read it in order:

```
ENGINEERING (1-16):
───────────────────
Part 1:  CLAUDE-CODE-INSTRUCTIONS    → Build phases, rules, success criteria
Part 2:  PROJECT-STRUCTURE           → 4-crate layout
Part 3:  DEPENDENCIES                → Cargo.toml for each crate
Part 4:  DATA-STRUCTURES             → Duration, Deadline, Schedule, Sequence, Decay
Part 5:  FILE-FORMAT                 → .atime binary spec
Part 6:  WRITE-ENGINE                → Creating/updating temporal data
Part 7:  QUERY-ENGINE                → Querying temporal relationships
Part 8:  INDEXES                     → Deadline, sequence, decay indexes
Part 9:  MCP-TOOLS                   → 15+ MCP tools
Part 10: MCP-RESOURCES               → atime:// URI patterns
Part 11: MCP-PROMPTS                 → Pre-built prompt templates
Part 12: EDGE-CASES                  → 16 mandatory edge cases
Part 13: CLI                         → `atime` command structure
Part 14: FFI                         → C bindings
Part 15: TESTS                       → Test phases and scenarios
Part 16: RESEARCH-PAPER              → LaTeX paper spec

CANONICAL INFRASTRUCTURE (17-25):
─────────────────────────────────
Part 17: CANONICAL-KIT               → CANONICAL_SISTER_KIT.md
Part 18: DOCS-PUBLIC                 → 13 required doc pages
Part 19: ASSETS                      → SVG specifications
Part 20: README-LAYOUT               → README structure
Part 21: INSTALL-SCRIPT              → scripts/install.sh
Part 22: CI-WORKFLOWS                → GitHub Actions
Part 23: GHOST-WRITER                → ghost_bridge.rs
Part 24: MCP-HARDENING               → Protocol hardening
Part 25: BINDINGS                    → Python, WASM, npm
```

---

## BUILD ORDER — FOLLOW EXACTLY

### Phase 1: Foundation (Day 1)
```
1. Create directory structure (Part 2)
2. Set up Cargo.toml files (Part 3)
3. Implement core data structures (Part 4)
4. Implement file format (Part 5)
5. Run Phase 1 tests
```

### Phase 2: Core Engine (Day 2)
```
6. Implement write engine (Part 6)
7. Implement query engine (Part 7)
8. Implement indexes (Part 8)
9. Run Phase 2 tests
```

### Phase 3: MCP Layer (Day 3)
```
10. Implement MCP tools (Part 9)
11. Implement MCP resources (Part 10)
12. Implement MCP prompts (Part 11)
13. Add MCP hardening (Part 24)
14. Run Phase 3 tests
```

### Phase 4: Integration (Day 4)
```
15. Implement CLI (Part 13)
16. Implement FFI (Part 14)
17. Implement Ghost Writer (Part 23)
18. Run Phase 4 tests
19. Test with Claude Desktop
```

### Phase 5: Documentation (Day 5)
```
20. Create all docs/public/ pages (Part 18)
21. Create SVG assets (Part 19)
22. Write README (Part 20)
23. Create install script (Part 21)
24. Set up CI workflows (Part 22)
```

### Phase 6: Validation (Day 6)
```
25. Run all edge case tests (Part 12)
26. Run full test suite (Part 15)
27. Generate research paper (Part 16)
28. Run guardrails — MUST PASS:
    - scripts/check-canonical-sister.sh (23 sections)
    - Workspace consistency (27 sections)
```

### Phase 7: Publish
```
29. Publish to crates.io (4 crates)
30. Create GitHub release
31. Verify install script works
```

---

## CRITICAL PATTERNS TO COPY FROM SISTERS

### 1. Ghost Writer Pattern (from Memory)
```rust
// crates/agentic-time-mcp/src/ghost_bridge.rs

// Copy the EXACT pattern from agentic-memory-mcp/src/ghost_bridge.rs
// - Spawn tokio task
// - 5-second sync interval
// - Detect all client dirs (Claude, Cursor, Windsurf, Cody)
// - Atomic writes (temp + rename)
// - Format context as markdown
```

### 2. MCP Hardening Pattern (from Memory/Codebase)
```rust
// crates/agentic-time-mcp/src/stdio.rs

const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024; // 8 MiB

// Copy from agentic-memory-mcp or agentic-codebase-mcp:
// - Content-Length framing
// - Frame size validation
// - JSON-RPC 2.0 version check
// - No silent fallbacks
```

### 3. 4-Crate Structure (all sisters)
```
crates/
├── agentic-time/           # Core library
├── agentic-time-mcp/       # MCP server
├── agentic-time-cli/       # CLI binary
└── agentic-time-ffi/       # FFI bindings
```

### 4. File Format Pattern (from Memory)
```rust
// .atime file header — copy pattern from .amem
const MAGIC: [u8; 4] = *b"ATIM";
const VERSION: u32 = 1;

struct FileHeader {
    magic: [u8; 4],
    version: u32,
    flags: u32,
    block_count: u64,
    index_offset: u64,
    created_at: u64,
    modified_at: u64,
    checksum: [u8; 32], // BLAKE3
}
```

---

## GUARDRAIL COMPLIANCE CHECKLIST

Before first CI push, verify:

```
PER-SISTER (23 sections):
─────────────────────────
[ ] 4-crate structure exists
[ ] CANONICAL_SISTER_KIT.md present with all sections
[ ] docs/public/ has all 13 required pages
[ ] All docs have status: frontmatter
[ ] sister.manifest.json present and valid
[ ] SVG assets present (hero, architecture, benchmark)
[ ] README has required sections
[ ] scripts/install.sh present and executable
[ ] scripts/check-canonical-sister.sh present
[ ] .github/workflows/ has required workflows
[ ] ghost_bridge.rs implemented
[ ] MCP hardening implemented (8 MiB limit, JSON-RPC validation)
[ ] Content-Length framing in stdio transport

WORKSPACE CONSISTENCY (27 sections):
────────────────────────────────────
[ ] check-canonical-sister.sh matches other sisters
[ ] CANONICAL_SISTER_KIT.md structure matches other sisters
[ ] MCP tool naming follows family pattern (time_*)
[ ] Manifest parity with other sisters
[ ] Ghost Writer output paths match family pattern
```

---

## SUCCESS CRITERIA

AgenticTime is complete when:

```
FUNCTIONAL:
───────────
[ ] cargo test — all tests pass, zero failures
[ ] cargo clippy — zero warnings
[ ] cargo fmt --check — passes
[ ] cargo build --release — compiles
[ ] `atime` CLI works
[ ] MCP server works with Claude Desktop
[ ] Ghost Writer syncs to Claude/Cursor/Windsurf/Cody

PERFORMANCE:
────────────
[ ] 100K temporal events loads in <100ms
[ ] Deadline queries complete in <1ms
[ ] .atime file for 100K events is <20MB

GUARDRAILS:
───────────
[ ] check-canonical-sister.sh passes (23/23)
[ ] Workspace consistency passes (27/27)

PUBLICATION:
────────────
[ ] 4 crates published to crates.io
[ ] GitHub release created
[ ] Install script verified working
[ ] Research paper PDF generated
```

---

## NAMING CONVENTIONS

```
FILE FORMAT:      .atime
BINARY:           atime
MCP SERVER KEY:   agentic-time
MCP TOOLS:        time_* (time_deadline_add, time_schedule_create, etc.)
MCP RESOURCES:    atime:// (atime://deadline/{id}, atime://schedule/{id})
ENV VAR PREFIX:   ATIME_
CRATE NAMES:      agentic-time, agentic-time-mcp, agentic-time-cli, agentic-time-ffi
```

---

## WHEN STUCK

1. **Check your sisters first** — They solved this problem already
2. **Run the guardrails** — They tell you exactly what's missing
3. **Look at Memory V3** — It's the most mature reference
4. **Don't invent new patterns** — Use family patterns for consistency

---

## START NOW

1. Read `AGENTIC-TIME-COMPLETE-SPEC.md` from Part 1 to Part 25
2. Create the directory structure
3. Begin Phase 1

**Give birth to AgenticTime. She's ready to be born.** 🕐
