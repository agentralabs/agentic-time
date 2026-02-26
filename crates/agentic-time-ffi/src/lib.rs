//! FFI bindings for AgenticTime.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use agentic_time::{TimeFile, WriteEngine};

/// FFI error codes.
#[repr(C)]
pub enum AtimeError {
    /// Success.
    Ok = 0,
    /// Entity not found.
    NotFound = 1,
    /// Invalid time range.
    InvalidRange = 2,
    /// Deadline already passed.
    DeadlinePassed = 3,
    /// Schedule conflict.
    ScheduleConflict = 4,
    /// Dependency not met.
    DependencyNotMet = 5,
    /// File format error.
    FileFormat = 6,
    /// IO error.
    Io = 7,
    /// Null pointer.
    NullPointer = 8,
}

/// Opaque handle to a TimeFile.
pub struct AtimeHandle {
    engine: WriteEngine,
}

/// Open an existing .atime file.
///
/// # Safety
/// `path` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn atime_open(path: *const c_char, err: *mut AtimeError) -> *mut AtimeHandle {
    if path.is_null() {
        if !err.is_null() {
            *err = AtimeError::NullPointer;
        }
        return ptr::null_mut();
    }

    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            if !err.is_null() {
                *err = AtimeError::Io;
            }
            return ptr::null_mut();
        }
    };

    match TimeFile::open(path_str) {
        Ok(file) => {
            if !err.is_null() {
                *err = AtimeError::Ok;
            }
            Box::into_raw(Box::new(AtimeHandle {
                engine: WriteEngine::new(file),
            }))
        }
        Err(_) => {
            if !err.is_null() {
                *err = AtimeError::FileFormat;
            }
            ptr::null_mut()
        }
    }
}

/// Create a new .atime file.
///
/// # Safety
/// `path` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn atime_create(
    path: *const c_char,
    err: *mut AtimeError,
) -> *mut AtimeHandle {
    if path.is_null() {
        if !err.is_null() {
            *err = AtimeError::NullPointer;
        }
        return ptr::null_mut();
    }

    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            if !err.is_null() {
                *err = AtimeError::Io;
            }
            return ptr::null_mut();
        }
    };

    match TimeFile::create(path_str) {
        Ok(file) => {
            if !err.is_null() {
                *err = AtimeError::Ok;
            }
            Box::into_raw(Box::new(AtimeHandle {
                engine: WriteEngine::new(file),
            }))
        }
        Err(_) => {
            if !err.is_null() {
                *err = AtimeError::Io;
            }
            ptr::null_mut()
        }
    }
}

/// Close and free an atime handle.
///
/// # Safety
/// `handle` must be a valid pointer from `atime_open` or `atime_create`.
#[no_mangle]
pub unsafe extern "C" fn atime_close(handle: *mut AtimeHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Save the current state to disk.
///
/// # Safety
/// `handle` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn atime_save(handle: *mut AtimeHandle) -> AtimeError {
    if handle.is_null() {
        return AtimeError::NullPointer;
    }

    let h = &mut *handle;
    match h.engine.file().save() {
        Ok(_) => AtimeError::Ok,
        Err(_) => AtimeError::Io,
    }
}

/// Get temporal statistics as JSON.
///
/// # Safety
/// All pointers must be valid. `json_out` must point to a buffer of at least `json_out_len` bytes.
#[no_mangle]
pub unsafe extern "C" fn atime_stats(
    handle: *mut AtimeHandle,
    json_out: *mut c_char,
    json_out_len: usize,
    written: *mut usize,
) -> AtimeError {
    if handle.is_null() || json_out.is_null() || written.is_null() {
        return AtimeError::NullPointer;
    }

    let h = &*handle;
    let query = agentic_time::QueryEngine::new(h.engine.file());
    match query.stats() {
        Ok(stats) => {
            let json = serde_json::to_string(&stats).unwrap_or_default();
            let bytes = json.as_bytes();
            let copy_len = bytes.len().min(json_out_len.saturating_sub(1));
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), json_out as *mut u8, copy_len);
            *json_out.add(copy_len) = 0; // null terminator
            *written = copy_len;
            AtimeError::Ok
        }
        Err(_) => AtimeError::Io,
    }
}
