/**
 * AgenticTime FFI — C bindings for temporal reasoning.
 *
 * Opaque handle-based API. All temporal operations go through
 * an AtimeHandle obtained from atime_open() or atime_create().
 */

#ifndef AGENTIC_TIME_H
#define AGENTIC_TIME_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Opaque handle to a temporal graph.
 */
typedef struct AtimeHandle AtimeHandle;

/**
 * Error codes.
 */
typedef enum {
    ATIME_OK = 0,
    ATIME_ERR_NOT_FOUND = 1,
    ATIME_ERR_INVALID_RANGE = 2,
    ATIME_ERR_DEADLINE_PASSED = 3,
    ATIME_ERR_SCHEDULE_CONFLICT = 4,
    ATIME_ERR_DEPENDENCY_NOT_MET = 5,
    ATIME_ERR_FILE_FORMAT = 6,
    ATIME_ERR_IO = 7,
    ATIME_ERR_NULL_POINTER = 8,
} AtimeError;

/**
 * Open an existing .atime file.
 */
AtimeHandle* atime_open(const char* path, AtimeError* err);

/**
 * Create a new .atime file.
 */
AtimeHandle* atime_create(const char* path, AtimeError* err);

/**
 * Close and free an atime handle.
 */
void atime_close(AtimeHandle* handle);

/**
 * Save current state to disk.
 */
AtimeError atime_save(AtimeHandle* handle);

/**
 * Get temporal statistics as JSON.
 *
 * @param handle  Valid handle
 * @param json_out  Buffer to write JSON into
 * @param json_out_len  Buffer capacity
 * @param written  Number of bytes written (excluding null terminator)
 * @return ATIME_OK on success
 */
AtimeError atime_stats(
    AtimeHandle* handle,
    char* json_out,
    size_t json_out_len,
    size_t* written
);

#ifdef __cplusplus
}
#endif

#endif /* AGENTIC_TIME_H */
