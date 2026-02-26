---
status: stable
---

# File Format

The `.atime` binary file format for AgenticTime temporal graphs.

## Overview

AgenticTime stores all temporal data in a single `.atime` binary file. The format is designed for:

- Fast random access to entities by ID
- Compact storage using MessagePack encoding
- Forward compatibility via versioned headers
- Safe concurrent access with file-level locking

## Header

| Offset | Size (bytes) | Field | Value |
|--------|-------------|-------|-------|
| 0 | 4 | Magic | `ATIM` (0x4154494D) |
| 4 | 2 | Version | `0x0001` |
| 6 | 2 | Flags | Reserved (0x0000) |
| 8 | 8 | Entity count | u64 little-endian |
| 16 | 8 | Index offset | u64 little-endian (byte offset to index section) |
| 24 | 8 | Created timestamp | u64 little-endian (Unix epoch seconds) |
| 32 | 8 | Modified timestamp | u64 little-endian (Unix epoch seconds) |
| 40 | 24 | Reserved | Zero-filled for future use |

Total header size: 64 bytes.

## Entity Data Section

Starts at offset 64. Contains MessagePack-encoded entities in insertion order.

Each entity is prefixed with:

| Field | Size | Description |
|-------|------|-------------|
| Entity type | 1 byte | 0x01=Deadline, 0x02=Duration, 0x03=Schedule, 0x04=Sequence, 0x05=Decay |
| Entity size | 4 bytes | u32 little-endian, size of MessagePack payload |
| Payload | variable | MessagePack-encoded entity data |

## Index Section

Located at the byte offset specified in the header. Contains a sorted array of index entries for O(log n) lookup:

| Field | Size | Description |
|-------|------|-------------|
| Entity ID | 8 bytes | u64 little-endian |
| Entity type | 1 byte | Type discriminator |
| Data offset | 8 bytes | u64 little-endian, byte offset into data section |

## Entity Schemas (MessagePack)

### Deadline

```
{
  "id": u64,
  "title": string,
  "due_at": i64 (Unix epoch seconds),
  "priority": u8 (0=low, 1=medium, 2=high, 3=critical),
  "status": u8 (0=pending, 1=in_progress, 2=completed, 3=missed, 4=cancelled),
  "tags": [string],
  "depends_on": [u64],
  "created_at": i64,
  "updated_at": i64
}
```

### Duration

```
{
  "id": u64,
  "label": string,
  "estimate_seconds": u64,
  "confidence": f64,
  "actual_seconds": u64 | null,
  "started_at": i64 | null,
  "completed_at": i64 | null,
  "created_at": i64
}
```

### Schedule

```
{
  "id": u64,
  "title": string,
  "recurrence": string (cron expression or "once"),
  "duration_minutes": u32,
  "timezone": string (IANA),
  "start_date": i64 | null,
  "end_date": i64 | null,
  "created_at": i64
}
```

### Sequence

```
{
  "id": u64,
  "title": string,
  "steps": [
    {
      "label": string,
      "duration_minutes": u32,
      "status": u8 (0=pending, 1=completed, 2=skipped)
    }
  ],
  "created_at": i64,
  "updated_at": i64
}
```

### Decay

```
{
  "id": u64,
  "name": string,
  "curve": u8 (0=exponential, 1=linear, 2=step),
  "halflife_hours": f64 | null,
  "window_hours": f64 | null,
  "threshold_hours": f64 | null,
  "floor": f64 | null,
  "created_at": i64
}
```

## Locking

Concurrent access uses a sidecar `.atime.lock` file:

- Lock file contains the PID of the holder
- Stale locks (dead PID) are automatically recovered
- Lock acquisition timeout: 5 seconds
- Merge-on-save when concurrent writes are detected

## Validation

```bash
atime validate project.atime
```

Checks:
- Magic bytes match `ATIM`
- Version is supported
- Entity count matches actual entities
- Index is consistent with data section
- All MessagePack payloads are well-formed
