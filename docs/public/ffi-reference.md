---
status: stable
---

# FFI Reference

AgenticTime exposes a C-compatible FFI layer through the `agentic-time-ffi` crate. This enables integration from any language that supports C function calls (Python ctypes, Node.js ffi-napi, Ruby FFI, Go cgo, etc.).

## Shared Library

Build the shared library:

```bash
cargo build --release -p agentic-time-ffi
# Output: target/release/libagentic_time_ffi.{so,dylib,dll}
```

## Functions

### `atime_open`

Open or create a temporal graph file.

```c
AtimeHandle* atime_open(const char* path);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `const char*` | Path to `.atime` file (created if missing) |

**Returns:** Opaque handle pointer, or `NULL` on error.

### `atime_create`

Create a new empty temporal graph.

```c
AtimeHandle* atime_create(const char* path);
```

**Returns:** Opaque handle pointer, or `NULL` if file already exists.

### `atime_close`

Close and free a temporal graph handle.

```c
void atime_close(AtimeHandle* handle);
```

### `atime_save`

Persist in-memory changes to disk.

```c
int atime_save(AtimeHandle* handle);
```

**Returns:** `0` on success, `-1` on error.

### `atime_stats`

Get graph statistics as a JSON string.

```c
const char* atime_stats(AtimeHandle* handle);
```

**Returns:** JSON string (caller must free with `atime_free_string`). Returns `NULL` on error.

### `atime_add_deadline`

Add a deadline.

```c
int64_t atime_add_deadline(
    AtimeHandle* handle,
    const char* title,
    const char* due_at_iso8601,
    const char* priority
);
```

**Returns:** Deadline ID on success, `-1` on error.

### `atime_list_deadlines`

List deadlines as a JSON array.

```c
const char* atime_list_deadlines(AtimeHandle* handle, const char* filter_json);
```

**Returns:** JSON string (caller must free). `NULL` on error.

### `atime_add_schedule`

Add a schedule entry.

```c
int64_t atime_add_schedule(
    AtimeHandle* handle,
    const char* title,
    const char* recurrence,
    int duration_minutes
);
```

**Returns:** Schedule ID on success, `-1` on error.

### `atime_create_sequence`

Create a new sequence.

```c
int64_t atime_create_sequence(AtimeHandle* handle, const char* title);
```

**Returns:** Sequence ID on success, `-1` on error.

### `atime_add_sequence_step`

Add a step to a sequence.

```c
int atime_add_sequence_step(
    AtimeHandle* handle,
    int64_t sequence_id,
    const char* label,
    int duration_minutes
);
```

**Returns:** `0` on success, `-1` on error.

### `atime_configure_decay`

Configure a decay curve.

```c
int atime_configure_decay(
    AtimeHandle* handle,
    const char* name,
    const char* config_json
);
```

**Returns:** `0` on success, `-1` on error.

### `atime_query_decay`

Query freshness at a given age.

```c
double atime_query_decay(
    AtimeHandle* handle,
    const char* name,
    double age_hours
);
```

**Returns:** Freshness value (0.0 to 1.0), or `-1.0` on error.

### `atime_export`

Export temporal data as JSON.

```c
const char* atime_export(AtimeHandle* handle, const char* entity_type);
```

**Returns:** JSON string (caller must free). `NULL` on error.

### `atime_free_string`

Free a string returned by other FFI functions.

```c
void atime_free_string(const char* s);
```

### `atime_last_error`

Get the last error message.

```c
const char* atime_last_error(void);
```

**Returns:** Error string (caller must NOT free; valid until next FFI call). `NULL` if no error.

## Example: Python ctypes

```python
import ctypes
import json

lib = ctypes.CDLL("libagentic_time_ffi.dylib")

lib.atime_open.restype = ctypes.c_void_p
lib.atime_open.argtypes = [ctypes.c_char_p]

lib.atime_stats.restype = ctypes.c_char_p
lib.atime_stats.argtypes = [ctypes.c_void_p]

lib.atime_close.argtypes = [ctypes.c_void_p]

handle = lib.atime_open(b"project.atime")
stats_json = lib.atime_stats(handle)
stats = json.loads(stats_json)
print(stats)
lib.atime_close(handle)
```

## Thread Safety

All FFI functions are thread-safe when called with different handles. Concurrent access to the same handle from multiple threads requires external synchronization.
