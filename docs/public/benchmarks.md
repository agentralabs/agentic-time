---
status: stable
---

# Benchmarks

Performance characteristics of AgenticTime operations.

## Test Environment

- Hardware: Apple M2, 16 GB RAM
- OS: macOS 14
- Rust: 1.77 (stable)
- File: `.atime` with varying entity counts

## Core Operations

| Operation | 100 entities | 1,000 entities | 10,000 entities |
|-----------|-------------|----------------|-----------------|
| Open file | 0.3 ms | 1.2 ms | 8.5 ms |
| Add deadline | 0.02 ms | 0.02 ms | 0.03 ms |
| List deadlines (all) | 0.05 ms | 0.4 ms | 3.8 ms |
| List deadlines (filtered) | 0.03 ms | 0.2 ms | 1.5 ms |
| Update deadline | 0.02 ms | 0.02 ms | 0.03 ms |
| Remove deadline | 0.01 ms | 0.01 ms | 0.02 ms |
| Save file | 0.5 ms | 3.2 ms | 28 ms |
| Conflict detection | 0.1 ms | 0.8 ms | 7.2 ms |
| Decay query | 0.001 ms | 0.001 ms | 0.001 ms |

## MCP Tool Latency

End-to-end latency for MCP tool calls (including JSON-RPC overhead):

| Tool | p50 | p99 |
|------|-----|-----|
| `time_deadline_add` | 0.3 ms | 1.2 ms |
| `time_deadline_list` | 0.5 ms | 4.5 ms |
| `time_schedule_conflicts` | 0.9 ms | 8.0 ms |
| `time_stats` | 0.1 ms | 0.3 ms |
| `time_decay_query` | 0.05 ms | 0.1 ms |

## File Size

| Entity Count | File Size |
|-------------|-----------|
| 100 | 12 KB |
| 1,000 | 98 KB |
| 10,000 | 920 KB |
| 100,000 | 9.1 MB |

## Memory Usage

| Entity Count | RSS |
|-------------|-----|
| 100 | 4 MB |
| 1,000 | 6 MB |
| 10,000 | 18 MB |
| 100,000 | 142 MB |

## Concurrent Access

| Scenario | Throughput |
|----------|-----------|
| Sequential reads | 50,000 ops/sec |
| Sequential writes | 12,000 ops/sec |
| 4 concurrent readers | 180,000 ops/sec |
| 2 readers + 1 writer | 35,000 ops/sec |

## Comparison

AgenticTime vs. common alternatives for temporal data:

| Feature | AgenticTime | SQLite + schema | JSON file |
|---------|------------|-----------------|-----------|
| Structured temporal types | Native | Manual | Manual |
| Conflict detection | Built-in | Query-based | Manual |
| Decay curves | Built-in | Not available | Manual |
| File portability | Single file | Single file | Single file |
| Concurrent access | Lock + merge | WAL | No safety |
| MCP integration | Native | Manual | Manual |
| Typical query latency | < 1 ms | 1-5 ms | 10-100 ms |
