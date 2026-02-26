// AgenticTime Core Library
//
// Temporal reasoning engine: deadlines, durations, schedules, sequences, decay.
// File format: .atime (magic ATIM, version 1)

pub mod types;

/// Magic bytes for .atime file format.
pub const MAGIC: &[u8; 4] = b"ATIM";

/// Current file format version.
pub const VERSION: u16 = 1;
