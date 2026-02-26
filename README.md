<p align="center">
  <img src="assets/github-hero-pane.svg" alt="AgenticTime — Temporal reasoning for AI agents" width="980" />
</p>

<p align="center">
  <a href="https://github.com/agentralabs/agentic-time/actions"><img src="https://github.com/agentralabs/agentic-time/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://crates.io/crates/agentic-time"><img src="https://img.shields.io/crates/v/agentic-time.svg" alt="crates.io" /></a>
  <a href="https://pypi.org/project/agentic-time/"><img src="https://img.shields.io/pypi/v/agentic-time.svg" alt="PyPI" /></a>
  <a href="https://www.npmjs.com/package/@agenticamem/agentic-time"><img src="https://img.shields.io/npm/v/@agenticamem/agentic-time.svg" alt="npm" /></a>
  <a href="https://github.com/agentralabs/agentic-time/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License" /></a>
</p>

<p align="center">
  <a href="#quickstart">Quickstart</a> &middot;
  <a href="#problems-solved">Problems Solved</a> &middot;
  <a href="#how-it-works">How It Works</a> &middot;
  <a href="#benchmarks">Benchmarks</a> &middot;
  <a href="#install">Install</a>
</p>

---

AgenticTime gives AI agents structured temporal reasoning: deadlines, durations, schedules, sequences, and decay curves -- all persisted in a portable `.atime` file and exposed through MCP.

<p align="center">
  <img src="assets/github-terminal-pane.svg" alt="AgenticTime terminal demo" width="980" />
</p>

## Install

**One-liner (recommended):**

```bash
curl -fsSL https://agentralabs.tech/install/time | bash
```

This downloads `agentic-time-mcp`, installs it to `~/.local/bin/`, and auto-configures detected MCP clients (Claude Desktop, Cursor, Windsurf, VS Code, Codex). Restart your MCP client after install.

**All profiles:**

```bash
# Desktop (default — auto-configures MCP clients)
curl -fsSL https://agentralabs.tech/install/time/desktop | bash

# Terminal (same as desktop, optimized for CLI use)
curl -fsSL https://agentralabs.tech/install/time/terminal | bash

# Server (token-gated, no desktop config changes)
curl -fsSL https://agentralabs.tech/install/time/server | bash
```

**Package managers:**

| Method | Command |
|--------|---------|
| npm | `npm install @agenticamem/agentic-time` |
| pip | `pip install agentic-time` |
| cargo | `cargo install --git https://github.com/agentralabs/agentic-time --locked agentic-time-mcp` |

**Standalone guarantee:** AgenticTime is fully standalone. It does not require any other Agentra sister to be installed.

## Quickstart

After install and MCP client restart, ask your agent:

```
Set a deadline for "finish API review" on Friday at 5pm
```

The agent calls `time_deadline_add` and persists it to your `.atime` file.

```
How long will the auth refactor take?
```

The agent calls `time_duration_estimate` with confidence intervals.

```
Schedule a 2-hour code review every Tuesday at 10am
```

The agent calls `time_schedule_add` with recurrence.

```
What's on my timeline this week?
```

The agent calls `time_deadline_list` and `time_schedule_list` to show your upcoming commitments.

**Python:**

```python
from agentic_time import TimeGraph

tg = TimeGraph("project.atime")
tg.add_deadline("Ship v1.0", "2026-03-15T17:00:00Z", priority="high")
tg.add_duration_estimate("Auth refactor", hours=8, confidence=0.7)
tg.save()
```

## Problems Solved

| Problem | Before | After |
|---------|--------|-------|
| **Deadline amnesia** | Agent forgets deadlines between conversations | Deadlines persist in `.atime` across sessions |
| **No duration tracking** | Estimates vanish; no actual vs estimated comparison | Stored with confidence; actuals tracked |
| **Schedule blindness** | No conflict detection for overlapping events | Automatic overlap detection |
| **No temporal dependencies** | Multi-step workflows have no structure | Sequences model ordered steps with status |
| **Stale information** | All facts weighted equally regardless of age | Decay curves quantify freshness |
| **Cross-project contamination** | Timeline state bleeds between projects | Per-project `.atime` with path-based isolation |

## How It Works

AgenticTime models temporal data through five entity types:

1. **Deadline** -- A fixed point in time with priority, status, and dependencies.
2. **Duration** -- An estimated time span with confidence and optional actual tracking.
3. **Schedule** -- A recurring or one-time calendar block with conflict detection.
4. **Sequence** -- An ordered chain of steps with dependency constraints.
5. **Decay** -- A function modeling information freshness over time.

All entities live in a `.atime` binary file (magic `ATIM`, version 1). The file is portable across models, clients, and deployments.

**19 MCP tools** expose all operations: add, list, update, remove deadlines; estimate and track durations; manage schedules with conflict detection; create and step through sequences; configure and query decay curves.

**4 MCP prompts** provide structured reasoning: `time_plan` for project planning, `time_review` for timeline health checks, `time_estimate` for duration estimation, and `time_schedule_day` for daily planning.

<p align="center">
  <img src="assets/architecture-agentra.svg" alt="Agentra ecosystem architecture" width="980" />
</p>

## Benchmarks

| Operation | 100 entities | 1K entities | 10K entities |
|-----------|-------------|-------------|--------------|
| Add deadline | 0.02 ms | 0.02 ms | 0.03 ms |
| List deadlines | 0.05 ms | 0.4 ms | 3.8 ms |
| Conflict detection | 0.1 ms | 0.8 ms | 7.2 ms |
| Decay query | 0.001 ms | 0.001 ms | 0.001 ms |
| Save file | 0.5 ms | 3.2 ms | 28 ms |

<p align="center">
  <img src="assets/benchmark-chart.svg" alt="AgenticTime benchmarks" width="980" />
</p>

## Architecture

AgenticTime is a 4-crate Rust workspace:

| Crate | Purpose |
|-------|---------|
| `agentic-time-core` | Core library: entities, file format, decay math |
| `agentic-time-mcp` | MCP server: 19 tools, resources, prompts, stdio transport |
| `agentic-time-cli` | CLI binary (`atime`): terminal interface |
| `agentic-time-ffi` | C FFI: cross-language bindings |

Plus `python/` (PyO3 SDK) and `npm/wasm/` (WebAssembly package).

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ATIME_FILE` | Auto-detected | Explicit `.atime` file path |
| `ATIME_TIMEZONE` | `UTC` | Default timezone for display |
| `ATIME_DECAY_MODEL` | `exponential` | Default decay curve |
| `ATIME_DECAY_HALFLIFE` | `168` | Half-life in hours |
| `AGENTIC_TOKEN` | None | Auth token (server profile) |

## Sister Integration

AgenticTime works standalone, but integrates with other Agentra sisters:

- **AgenticMemory**: Decay curves inform memory freshness. Deadlines link to decision nodes.
- **AgenticVision**: Schedules trigger periodic visual captures.
- **AgenticCodebase**: Sequences model deployment pipelines. Durations track refactoring effort.
- **AgenticIdentity**: Temporal operations signed with identity receipts.

## Documentation

- [Overview](docs/public/overview.md)
- [Quickstart](docs/public/quickstart.md)
- [Installation](docs/public/installation.md)
- [MCP Tools](docs/public/mcp-tools.md)
- [CLI Reference](docs/public/cli-reference.md)
- [API Reference](docs/public/api-reference.md)
- [Architecture](docs/public/architecture.md)
- [Benchmarks](docs/public/benchmarks.md)
- [FAQ](docs/public/faq.md)

## License

Dual-licensed under MIT and Apache 2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
