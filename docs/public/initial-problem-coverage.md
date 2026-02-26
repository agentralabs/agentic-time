---
status: stable
---

# Initial Problem Coverage

The foundational temporal reasoning gaps that motivated AgenticTime.

## Gap 1: No Persistent Timeline

AI agents have no built-in mechanism for persisting temporal data across conversations. Every deadline, estimate, and schedule exists only in the current context window.

**Solution**: The `.atime` file format provides a portable, persistent temporal graph.

## Gap 2: No Structured Temporal Types

Without structured types, temporal information is stored as unstructured text, making it impossible to reason about programmatically.

**Solution**: Five entity types (Deadline, Duration, Schedule, Sequence, Decay) cover all common temporal patterns.

## Gap 3: No Conflict Detection

Agents cannot detect when two scheduled events overlap or when a deadline is unreachable given existing commitments.

**Solution**: Built-in conflict detection for schedule overlaps and deadline dependency analysis.

## Gap 4: No Freshness Model

All information is treated as equally valid regardless of age. A fact from six months ago has the same weight as one from five minutes ago.

**Solution**: Configurable decay curves model information freshness with mathematical precision.

## Gap 5: No Temporal Portability

Even when temporal data exists in a conversation, it cannot be carried to a different model, client, or deployment.

**Solution**: The `.atime` format is model-agnostic and client-agnostic, working across any MCP-compatible system.
