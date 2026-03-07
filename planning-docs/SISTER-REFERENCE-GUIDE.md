# Sister Reference Guide — What to Copy From Where

> **For each component you need to build, this guide tells you which sister has the best implementation to reference.**

---

## QUICK LOOKUP TABLE

| Component | Best Sister | File Path | Notes |
|-----------|-------------|-----------|-------|
| Ghost Writer | Memory | `crates/agentic-memory-mcp/src/ghost_bridge.rs` | Gold standard, copy exactly |
| MCP Hardening | Memory | `crates/agentic-memory-mcp/src/stdio.rs` | Content-Length, 8 MiB limit |
| File Format | Memory | `crates/agentic-memory/src/file_format.rs` | Binary format pattern |
| MCP Tools | Codebase | `crates/agentic-codebase-mcp/src/tools.rs` | Clean tool definitions |
| MCP Resources | Memory | `crates/agentic-memory-mcp/src/resources.rs` | URI patterns |
| CLI Structure | Memory | `crates/agentic-memory-cli/src/` | Command organization |
| FFI Bindings | Identity | `crates/agentic-identity-ffi/` | Clean C bindings |
| Error Types | Codebase | `crates/agentic-codebase/src/error.rs` | thiserror pattern |
| Indexes | Memory | `crates/agentic-memory/src/indexes/` | B-tree indexes |
| Install Script | Memory | `scripts/install.sh` | Profile-based install |
| CI Workflows | Memory | `.github/workflows/` | Full CI setup |
| README Layout | Memory | `README.md` | Badge rows, sections |
| CANONICAL_KIT | Memory | `CANONICAL_SISTER_KIT.md` | All required sections |
| Manifest | Memory | `sister.manifest.json` | page_ids format |
| docs/public/ | Memory | `docs/public/*.md` | Frontmatter format |

---

## DETAILED REFERENCE BY COMPONENT

### 1. Ghost Writer

**Copy from:** Memory (`agentic-memory-mcp/src/ghost_bridge.rs`)

**What to copy:**
```rust
// Function: detect_all_memory_dirs()
// - Returns Vec<(String, PathBuf)> for each client
// - Creates dirs if they don't exist
// - Handles: Claude, Cursor, Windsurf, Cody

// Function: spawn_ghost_writer()
// - Spawns tokio background task
// - 5-second interval using tokio::time::interval
// - Calls format function and writes to all dirs
// - Atomic writes: write to .tmp, then rename

// Function: format_for_client() or similar
// - Formats context as markdown
// - Different filenames per client:
//   - Claude: TIME_CONTEXT.md (or V3_CONTEXT.md for Memory)
//   - Others: agentic-time.md
```

**Adapt for Time:**
- Format includes: overdue deadlines, today's schedule, active sequences, critical decays
- Filename: `TIME_CONTEXT.md` for Claude, `agentic-time.md` for others

---

### 2. MCP Hardening (stdio transport)

**Copy from:** Memory (`agentic-memory-mcp/src/stdio.rs`) or Codebase

**What to copy:**
```rust
// Constant
const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024;

// Function: read_message()
// - Read headers until \r\n\r\n
// - Parse Content-Length header
// - VALIDATE size < MAX before reading body
// - Return error if too large (don't just truncate)

// Function: write_message()
// - Write Content-Length: {len}\r\n\r\n{body}
// - Flush stdout

// Function: validate_jsonrpc()
// - Check "jsonrpc": "2.0"
// - Return error if missing or wrong version
```

**Critical:** No silent fallbacks. If validation fails, return error.

---

### 3. File Format (.atime)

**Copy from:** Memory (`agentic-memory/src/file_format.rs`)

**What to copy:**
```rust
// Magic bytes (change for Time)
const MAGIC: [u8; 4] = *b"ATIM";
const VERSION: u32 = 1;

// Header structure (64 bytes)
struct FileHeader {
    magic: [u8; 4],
    version: u32,
    flags: u32,
    // ... your fields
    checksum: [u8; 32], // BLAKE3
}

// Entity block pattern
struct EntityBlock {
    entity_type: u8,
    id: [u8; 16],  // UUID
    payload_len: u32,
    payload: Vec<u8>, // JSON-serialized entity
}

// Atomic save pattern
fn save(&self) -> Result<()> {
    let tmp = path.with_extension("atime.tmp");
    // Write to tmp
    // Rename tmp to final (atomic)
}
```

---

### 4. MCP Tools Definition

**Copy from:** Codebase (`agentic-codebase-mcp/src/tools.rs`)

**What to copy:**
```rust
// Tool definition structure
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub input_schema: &'static str, // JSON Schema
}

// Tool list as const array
pub const TOOLS: &[ToolDefinition] = &[
    ToolDefinition {
        name: "time_deadline_add",
        description: "Add a new deadline",
        input_schema: r#"{ ... }"#,
    },
    // ...
];

// Tool handler pattern
async fn handle_tool_call(name: &str, args: Value) -> Result<Value> {
    match name {
        "time_deadline_add" => { /* ... */ }
        _ => Err(McpError::ToolNotFound)
    }
}
```

---

### 5. MCP Resources

**Copy from:** Memory (`agentic-memory-mcp/src/resources.rs`)

**What to copy:**
```rust
// Resource definition
struct ResourceDefinition {
    uri: String,
    name: String,
    description: String,
    mime_type: String,
}

// List resources function
fn list_resources() -> Vec<ResourceDefinition> { ... }

// Read resource function
fn read_resource(uri: &str) -> Result<String> {
    match uri {
        "atime://deadline/overdue" => { /* query and serialize */ }
        uri if uri.starts_with("atime://deadline/") => {
            let id = uri.strip_prefix("atime://deadline/").unwrap();
            // Look up by ID
        }
        _ => Err(ResourceNotFound)
    }
}
```

---

### 6. CLI Structure

**Copy from:** Memory (`agentic-memory-cli/src/`)

**What to copy:**
```
src/
├── main.rs           # Clap app setup
└── commands/
    ├── mod.rs        # Command exports
    ├── deadline.rs   # deadline subcommand
    ├── schedule.rs   # schedule subcommand
    ├── install.rs    # install subcommand
    └── serve.rs      # serve subcommand (starts MCP)
```

**Clap pattern:**
```rust
#[derive(Parser)]
#[command(name = "atime")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Deadline(DeadlineArgs),
    Schedule(ScheduleArgs),
    Install(InstallArgs),
    Serve(ServeArgs),
}
```

---

### 7. FFI Bindings

**Copy from:** Identity (`agentic-identity-ffi/`)

**What to copy:**
```rust
// lib.rs pattern
#[no_mangle]
pub extern "C" fn atime_open(
    path: *const c_char,
    err: *mut AtimeError,
) -> *mut TimeFile {
    // Convert path
    // Call Rust function
    // Handle errors
    // Return opaque pointer
}

// Error enum
#[repr(C)]
pub enum AtimeError {
    Ok = 0,
    NotFound = 1,
    // ...
}

// Header file (include/agentic_time.h)
// - Opaque handle typedef
// - Error enum
// - Function declarations
```

---

### 8. Install Script

**Copy from:** Memory (`scripts/install.sh`)

**What to copy:**
```bash
#!/usr/bin/env bash
set -euo pipefail

# Profile detection
PROFILE="${1:-desktop}"

# Binary installation
# - Try release artifact first
# - Fall back to cargo install

# MCP config merge
# - NEVER overwrite existing config
# - Merge new server into existing

# Completion block
# - MCP client summary
# - Generic guidance
# - Test command
```

---

### 9. CI Workflows

**Copy from:** Memory (`.github/workflows/`)

**What to copy:**
```yaml
# ci.yml
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test

# release.yml
- Build release binaries
- Create GitHub release
- Upload artifacts

# canonical-sister-guardrails.yml
- Run check-canonical-sister.sh
- Fail if any section fails
```

---

### 10. Error Handling Pattern

**Copy from:** Codebase (`agentic-codebase/src/error.rs`)

**What to copy:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type TimeResult<T> = Result<T, TimeError>;
```

---

## SISTER REPO URLS

```
Memory:   github.com/agentralabs/agentic-memory
Vision:   github.com/agentralabs/agentic-vision
Codebase: github.com/agentralabs/agentic-codebase
Identity: github.com/agentralabs/agentic-identity
```

---

## WORKFLOW

1. **Before implementing a component:**
   - Look up in table above
   - Open the sister's file
   - Read and understand the pattern

2. **While implementing:**
   - Copy the structure
   - Adapt for Time's needs
   - Keep the same patterns (naming, error handling, etc.)

3. **After implementing:**
   - Compare with sister's version
   - Ensure consistency
   - Run tests

---

## THE GOLDEN RULE

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  IF A SISTER SOLVED IT, COPY THE SOLUTION.                               ║
║                                                                           ║
║  Memory is the eldest and wisest.                                         ║
║  When in doubt, do what Memory does.                                      ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```
