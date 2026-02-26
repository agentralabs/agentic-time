---
status: stable
---

# CLI Reference

The `atime` CLI provides command-line access to AgenticTime temporal graphs.

## Global Options

| Option | Description |
|--------|-------------|
| `--file <path>` | Path to `.atime` file (default: auto-detected) |
| `--format <fmt>` | Output format: `text`, `json`, `table` (default: `text`) |
| `-h, --help` | Print help information |
| `-V, --version` | Print version |

## Commands

### `atime deadline`

Manage deadlines.

```bash
# Add a deadline
atime deadline add "Ship v1.0" --due "2026-03-15T17:00:00Z" --priority high

# List deadlines
atime deadline list
atime deadline list --status pending --sort due_at

# Update a deadline
atime deadline update 42 --status completed

# Remove a deadline
atime deadline remove 42

# Show upcoming deadlines (next 7 days)
atime deadline upcoming

# Show overdue deadlines
atime deadline overdue
```

### `atime schedule`

Manage schedules.

```bash
# Add a one-time schedule
atime schedule add "Design review" --at "2026-03-10T14:00:00Z" --duration 60

# Add a recurring schedule
atime schedule add "Weekly standup" --cron "0 9 * * MON" --duration 30

# List schedules
atime schedule list
atime schedule list --from "2026-03-01" --to "2026-03-31"

# Check for conflicts
atime schedule conflicts --from "2026-03-10" --to "2026-03-10"

# Remove a schedule
atime schedule remove 7
```

### `atime sequence`

Manage sequences (ordered step chains).

```bash
# Create a sequence
atime sequence create "Deploy pipeline"

# Add steps
atime sequence step add 1 "Run tests" --duration 10
atime sequence step add 1 "Build artifacts" --duration 5
atime sequence step add 1 "Deploy staging" --duration 2

# Complete a step
atime sequence step complete 1 0

# View sequence status
atime sequence status 1

# List all sequences
atime sequence list
```

### `atime decay`

Configure and query decay curves.

```bash
# Configure exponential decay
atime decay configure memory-freshness --curve exponential --halflife 168

# Configure linear decay
atime decay configure task-urgency --curve linear --window 720

# Query freshness at a given age
atime decay query memory-freshness --age 48
# Output: freshness=0.814 (exponential, halflife=168h)

# Reset to defaults
atime decay reset memory-freshness

# List all decay configs
atime decay list
```

### `atime stats`

Print temporal graph statistics.

```bash
atime stats
# Output:
#   Deadlines:     42
#   Schedules:     15
#   Sequences:      3
#   Decay configs:  2
#   Total:         62
#   File:          /path/to/project.atime
#   Size:          8.2 KB
```

### `atime export`

Export temporal data.

```bash
# Export all as JSON
atime export --format json > timeline.json

# Export deadlines only
atime export --type deadline --format json

# Export schedules as iCal
atime export --type schedule --format ical > calendar.ics
```

### `atime import`

Import temporal data.

```bash
# Import from JSON
atime import timeline.json

# Import with merge (default)
atime import timeline.json --merge

# Import replacing existing
atime import timeline.json --replace
```

### `atime validate`

Validate a `.atime` file.

```bash
atime validate project.atime
# Output: Valid .atime file (v1, 62 entities)
```

### `atime info`

Print file metadata.

```bash
atime info project.atime
# Output:
#   Format:    ATIM v1
#   Created:   2026-02-20T10:00:00Z
#   Modified:  2026-02-26T14:30:00Z
#   Entities:  62
#   Size:      8.2 KB
```
