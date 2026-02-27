//! Agentic-contracts trait implementations for AgenticTime.
//!
//! Implements: Sister, Queryable, FileFormatReader, FileFormatWriter
//! Does NOT implement: SessionManagement (stateless per-project),
//!                     WorkspaceManagement (stateless per-project),
//!                     Grounding (temporal entities aren't knowledge claims)

use std::path::{Path, PathBuf};
use std::time::Instant;

use agentic_contracts::prelude::*;
use chrono::Utc;

use crate::error::TimeError;
use crate::file_format::{self, FileHeader, TimeFile};
use crate::{Deadline, DecayModel, DurationEstimate, Schedule, Sequence, TemporalId, WriteEngine};

// ═══════════════════════════════════════════════════════════════════
// ERROR BRIDGE
// ═══════════════════════════════════════════════════════════════════

impl From<TimeError> for SisterError {
    fn from(e: TimeError) -> Self {
        match &e {
            TimeError::NotFound(entity) => {
                SisterError::not_found(format!("temporal entity not found: {entity}"))
            }
            TimeError::InvalidRange { start, end } => {
                SisterError::invalid_input(format!("Invalid time range: {start} to {end}"))
            }
            TimeError::DeadlinePassed(msg) => {
                SisterError::new(ErrorCode::InvalidState, format!("Deadline passed: {msg}"))
            }
            TimeError::ScheduleConflict(a, b) => SisterError::new(
                ErrorCode::InvalidState,
                format!("Schedule conflict: {a} overlaps with {b}"),
            ),
            TimeError::DependencyNotMet { step, dependency } => SisterError::new(
                ErrorCode::InvalidState,
                format!("Dependency not met: {step} depends on {dependency}"),
            ),
            TimeError::InvalidRecurrence(msg) => {
                SisterError::invalid_input(format!("Invalid recurrence: {msg}"))
            }
            TimeError::InvalidDuration(msg) => {
                SisterError::invalid_input(format!("Invalid duration: {msg}"))
            }
            TimeError::FileFormat(msg) => SisterError::new(
                ErrorCode::VersionMismatch,
                format!("File format error: {msg}"),
            ),
            TimeError::Io(err) => {
                SisterError::new(ErrorCode::StorageError, format!("IO error: {err}"))
            }
            TimeError::Serialization(err) => SisterError::new(
                ErrorCode::StorageError,
                format!("Serialization error: {err}"),
            ),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// FACADE
// ═══════════════════════════════════════════════════════════════════

/// Contract facade wrapping the TimeFile and engines.
///
/// Time is a stateless per-project sister. No sessions or workspaces —
/// each .atime file is an independent temporal graph.
pub struct TimeSister {
    /// The underlying write engine (owns the TimeFile)
    engine: WriteEngine,

    /// Path to the .atime file
    #[allow(dead_code)]
    file_path: PathBuf,

    /// Startup time for uptime tracking
    started_at: Instant,
}

// ═══════════════════════════════════════════════════════════════════
// SISTER TRAIT
// ═══════════════════════════════════════════════════════════════════

impl Sister for TimeSister {
    const SISTER_TYPE: SisterType = SisterType::Time;
    const FILE_EXTENSION: &'static str = "atime";

    fn init(config: SisterConfig) -> SisterResult<Self>
    where
        Self: Sized,
    {
        let path = config
            .data_path
            .unwrap_or_else(|| PathBuf::from("project.atime"));

        let time_file = if path.exists() {
            TimeFile::open(&path).map_err(SisterError::from)?
        } else if config.create_if_missing {
            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        SisterError::new(
                            ErrorCode::StorageError,
                            format!("Failed to create parent dir: {e}"),
                        )
                    })?;
                }
            }
            TimeFile::create(&path).map_err(SisterError::from)?
        } else {
            return Err(SisterError::not_found(format!(
                "Time file not found: {}",
                path.display()
            )));
        };

        let engine = WriteEngine::new(time_file);

        Ok(Self {
            engine,
            file_path: path,
            started_at: Instant::now(),
        })
    }

    fn health(&self) -> HealthStatus {
        let uptime = self.started_at.elapsed();
        let entity_count = self.engine.file().entity_count();

        HealthStatus {
            healthy: true,
            status: Status::Ready,
            uptime,
            resources: ResourceUsage {
                memory_bytes: 0,
                disk_bytes: 0,
                open_handles: entity_count,
            },
            warnings: vec![],
            last_error: None,
        }
    }

    fn version(&self) -> Version {
        Version::new(0, 1, 0)
    }

    fn shutdown(&mut self) -> SisterResult<()> {
        self.engine.file().save().map_err(SisterError::from)?;
        Ok(())
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![
            Capability::new(
                "time_deadline_add",
                "Add a deadline with urgency and consequences",
            ),
            Capability::new(
                "time_deadline_query",
                "Query deadlines by status or due date",
            ),
            Capability::new("time_schedule_add", "Add a schedule with recurrence"),
            Capability::new("time_schedule_query", "Query schedules and find conflicts"),
            Capability::new("time_sequence_add", "Add an ordered sequence of steps"),
            Capability::new("time_sequence_query", "Query sequences by status"),
            Capability::new("time_decay_add", "Add a decay model for value degradation"),
            Capability::new("time_decay_query", "Query current decay values"),
            Capability::new("time_duration_add", "Add a PERT duration estimate"),
            Capability::new("time_stats", "Get temporal graph statistics"),
            Capability::new("time_available_slots", "Find available time slots"),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════
// QUERYABLE
// ═══════════════════════════════════════════════════════════════════

impl Queryable for TimeSister {
    fn query(&self, query: Query) -> SisterResult<QueryResult> {
        let start = Instant::now();
        let file = self.engine.file();

        match query.query_type.as_str() {
            "list" => {
                let limit = query.limit.unwrap_or(50);
                let offset = query.offset.unwrap_or(0);
                let entity_filter = query.get_string("entity_type");

                let mut results: Vec<serde_json::Value> = Vec::new();

                // List by entity type or all
                match entity_filter.as_deref() {
                    Some("deadline") | None => {
                        for block in file.list_by_type(crate::file_format::EntityType::Deadline) {
                            if let Ok(d) = block.deserialize::<Deadline>() {
                                results.push(serde_json::json!({
                                    "id": d.id.to_string(),
                                    "type": "deadline",
                                    "label": d.label,
                                    "due_at": d.due_at.to_rfc3339(),
                                    "status": format!("{:?}", d.status),
                                }));
                            }
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("schedule") | None => {
                        for block in file.list_by_type(crate::file_format::EntityType::Schedule) {
                            if let Ok(s) = block.deserialize::<Schedule>() {
                                results.push(serde_json::json!({
                                    "id": s.id.to_string(),
                                    "type": "schedule",
                                    "label": s.label,
                                    "start_at": s.start_at.to_rfc3339(),
                                    "status": format!("{:?}", s.status),
                                }));
                            }
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("sequence") | None => {
                        for block in file.list_by_type(crate::file_format::EntityType::Sequence) {
                            if let Ok(seq) = block.deserialize::<Sequence>() {
                                results.push(serde_json::json!({
                                    "id": seq.id.to_string(),
                                    "type": "sequence",
                                    "label": seq.label,
                                    "status": format!("{:?}", seq.status),
                                    "steps": seq.steps.len(),
                                }));
                            }
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("decay") | None => {
                        for block in file.list_by_type(crate::file_format::EntityType::Decay) {
                            if let Ok(d) = block.deserialize::<DecayModel>() {
                                results.push(serde_json::json!({
                                    "id": d.id.to_string(),
                                    "type": "decay",
                                    "label": d.label,
                                    "current_value": d.current_value,
                                }));
                            }
                        }
                    }
                    _ => {}
                }

                match entity_filter.as_deref() {
                    Some("duration") | None => {
                        for block in file.list_by_type(crate::file_format::EntityType::Duration) {
                            if let Ok(d) = block.deserialize::<DurationEstimate>() {
                                results.push(serde_json::json!({
                                    "id": d.id.to_string(),
                                    "type": "duration",
                                    "label": d.label,
                                    "expected_secs": d.expected_secs,
                                }));
                            }
                        }
                    }
                    _ => {}
                }

                let total = results.len();
                let paged: Vec<serde_json::Value> =
                    results.into_iter().skip(offset).take(limit).collect();

                Ok(QueryResult::new(query, paged, start.elapsed())
                    .with_pagination(total, offset + limit < total))
            }
            "search" => {
                let query_text = query.get_string("text").unwrap_or_default();
                let query_lower = query_text.to_lowercase();
                let limit = query.limit.unwrap_or(20);

                let mut scored: Vec<(f64, serde_json::Value)> = Vec::new();

                // Search deadlines
                for block in file.list_by_type(crate::file_format::EntityType::Deadline) {
                    if let Ok(d) = block.deserialize::<Deadline>() {
                        let score = word_overlap_score(&query_text, &d.label);
                        let tag_score = d
                            .tags
                            .iter()
                            .any(|t| query_lower.contains(&t.to_lowercase()))
                            as u8 as f64
                            * 0.3;
                        let total_score = (score + tag_score).min(1.0);
                        if total_score > 0.2 {
                            scored.push((
                                total_score,
                                serde_json::json!({
                                    "id": d.id.to_string(),
                                    "type": "deadline",
                                    "label": d.label,
                                    "due_at": d.due_at.to_rfc3339(),
                                    "score": total_score,
                                }),
                            ));
                        }
                    }
                }

                // Search schedules
                for block in file.list_by_type(crate::file_format::EntityType::Schedule) {
                    if let Ok(s) = block.deserialize::<Schedule>() {
                        let score = word_overlap_score(&query_text, &s.label);
                        if score > 0.2 {
                            scored.push((
                                score,
                                serde_json::json!({
                                    "id": s.id.to_string(),
                                    "type": "schedule",
                                    "label": s.label,
                                    "score": score,
                                }),
                            ));
                        }
                    }
                }

                // Search sequences
                for block in file.list_by_type(crate::file_format::EntityType::Sequence) {
                    if let Ok(seq) = block.deserialize::<Sequence>() {
                        let score = word_overlap_score(&query_text, &seq.label);
                        if score > 0.2 {
                            scored.push((
                                score,
                                serde_json::json!({
                                    "id": seq.id.to_string(),
                                    "type": "sequence",
                                    "label": seq.label,
                                    "score": score,
                                }),
                            ));
                        }
                    }
                }

                scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                let total = scored.len();
                scored.truncate(limit);
                let results: Vec<serde_json::Value> = scored.into_iter().map(|(_, v)| v).collect();

                Ok(QueryResult::new(query, results, start.elapsed())
                    .with_pagination(total, total > limit))
            }
            "recent" => {
                let limit = query.limit.unwrap_or(10);

                // Collect all entities with timestamps, sort by most recent
                let mut entries: Vec<(chrono::DateTime<Utc>, serde_json::Value)> = Vec::new();

                for block in file.list_by_type(crate::file_format::EntityType::Deadline) {
                    if let Ok(d) = block.deserialize::<Deadline>() {
                        entries.push((
                            d.created_at,
                            serde_json::json!({
                                "id": d.id.to_string(),
                                "type": "deadline",
                                "label": d.label,
                                "created_at": d.created_at.to_rfc3339(),
                            }),
                        ));
                    }
                }

                for block in file.list_by_type(crate::file_format::EntityType::Schedule) {
                    if let Ok(s) = block.deserialize::<Schedule>() {
                        entries.push((
                            s.created_at,
                            serde_json::json!({
                                "id": s.id.to_string(),
                                "type": "schedule",
                                "label": s.label,
                                "created_at": s.created_at.to_rfc3339(),
                            }),
                        ));
                    }
                }

                for block in file.list_by_type(crate::file_format::EntityType::Sequence) {
                    if let Ok(seq) = block.deserialize::<Sequence>() {
                        entries.push((
                            seq.created_at,
                            serde_json::json!({
                                "id": seq.id.to_string(),
                                "type": "sequence",
                                "label": seq.label,
                                "created_at": seq.created_at.to_rfc3339(),
                            }),
                        ));
                    }
                }

                entries.sort_by(|a, b| b.0.cmp(&a.0));
                let total = entries.len();
                entries.truncate(limit);
                let results: Vec<serde_json::Value> = entries.into_iter().map(|(_, v)| v).collect();

                Ok(QueryResult::new(query, results, start.elapsed())
                    .with_pagination(total, total > limit))
            }
            "get" => {
                let id_str = query.get_string("id").ok_or_else(|| {
                    SisterError::invalid_input("'get' query requires 'id' parameter")
                })?;

                let temporal_id: TemporalId = id_str.parse().map_err(|_| {
                    SisterError::invalid_input(format!("Invalid temporal ID: {id_str}"))
                })?;

                // Try each entity type — get() returns TimeResult<Option<T>>
                if let Ok(Some(d)) = file.get::<Deadline>(&temporal_id) {
                    let result = serde_json::json!({
                        "id": d.id.to_string(),
                        "type": "deadline",
                        "label": d.label,
                        "due_at": d.due_at.to_rfc3339(),
                        "status": format!("{:?}", d.status),
                        "consequence": format!("{:?}", d.consequence),
                        "tags": d.tags,
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed())
                        .with_pagination(1, false));
                }

                if let Ok(Some(s)) = file.get::<Schedule>(&temporal_id) {
                    let result = serde_json::json!({
                        "id": s.id.to_string(),
                        "type": "schedule",
                        "label": s.label,
                        "start_at": s.start_at.to_rfc3339(),
                        "duration_secs": s.duration_secs,
                        "status": format!("{:?}", s.status),
                        "tags": s.tags,
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed())
                        .with_pagination(1, false));
                }

                if let Ok(Some(seq)) = file.get::<Sequence>(&temporal_id) {
                    let result = serde_json::json!({
                        "id": seq.id.to_string(),
                        "type": "sequence",
                        "label": seq.label,
                        "status": format!("{:?}", seq.status),
                        "steps": seq.steps.len(),
                        "current_step": seq.current_step,
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed())
                        .with_pagination(1, false));
                }

                if let Ok(Some(d)) = file.get::<DecayModel>(&temporal_id) {
                    let result = serde_json::json!({
                        "id": d.id.to_string(),
                        "type": "decay",
                        "label": d.label,
                        "current_value": d.current_value,
                        "initial_value": d.initial_value,
                        "floor": d.floor,
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed())
                        .with_pagination(1, false));
                }

                if let Ok(Some(d)) = file.get::<DurationEstimate>(&temporal_id) {
                    let result = serde_json::json!({
                        "id": d.id.to_string(),
                        "type": "duration",
                        "label": d.label,
                        "optimistic_secs": d.optimistic_secs,
                        "expected_secs": d.expected_secs,
                        "pessimistic_secs": d.pessimistic_secs,
                    });
                    return Ok(QueryResult::new(query, vec![result], start.elapsed())
                        .with_pagination(1, false));
                }

                Err(SisterError::not_found(format!(
                    "Temporal entity not found: {id_str}"
                )))
            }
            other => Err(SisterError::invalid_input(format!(
                "Unknown query type: {other}. Supported: list, search, recent, get"
            ))),
        }
    }

    fn supports_query(&self, query_type: &str) -> bool {
        matches!(query_type, "list" | "search" | "recent" | "get")
    }

    fn query_types(&self) -> Vec<QueryTypeInfo> {
        vec![
            QueryTypeInfo::new("list", "List all temporal entities with pagination")
                .optional(vec!["entity_type"]),
            QueryTypeInfo::new("search", "Search entities by label text").required(vec!["text"]),
            QueryTypeInfo::new("recent", "Get most recently created entities"),
            QueryTypeInfo::new("get", "Get a specific entity by ID").required(vec!["id"]),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════
// FILE FORMAT READER / WRITER
// ═══════════════════════════════════════════════════════════════════

impl FileFormatReader for TimeSister {
    fn read_file(path: &Path) -> SisterResult<Self> {
        let time_file = TimeFile::open(path).map_err(SisterError::from)?;
        let engine = WriteEngine::new(time_file);

        Ok(Self {
            engine,
            file_path: path.to_path_buf(),
            started_at: Instant::now(),
        })
    }

    fn can_read(path: &Path) -> SisterResult<FileInfo> {
        let mut file = std::fs::File::open(path)?;
        let header = FileHeader::read_from(&mut file).map_err(SisterError::from)?;

        let created_at = chrono::DateTime::from_timestamp(
            (header.created_at / 1_000_000) as i64,
            ((header.created_at % 1_000_000) * 1000) as u32,
        )
        .unwrap_or_else(Utc::now);

        let updated_at = chrono::DateTime::from_timestamp(
            (header.modified_at / 1_000_000) as i64,
            ((header.modified_at % 1_000_000) * 1000) as u32,
        )
        .unwrap_or_else(Utc::now);

        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

        Ok(FileInfo {
            sister_type: SisterType::Time,
            version: Version::new(0, header.version as u8, 0),
            created_at,
            updated_at,
            content_length: file_size.saturating_sub(FileHeader::SIZE as u64),
            needs_migration: header.version != file_format::VERSION,
            format_id: "ATIM".to_string(),
        })
    }

    fn file_version(path: &Path) -> SisterResult<Version> {
        let info = Self::can_read(path)?;
        Ok(info.version)
    }

    fn migrate(_data: &[u8], _from_version: Version) -> SisterResult<Vec<u8>> {
        // v0.1.0 only — no migration needed yet
        Err(SisterError::new(
            ErrorCode::VersionMismatch,
            "No migration paths available yet (v0.1.0 only)".to_string(),
        ))
    }
}

impl FileFormatWriter for TimeSister {
    fn write_file(&self, path: &Path) -> SisterResult<()> {
        let file = self.engine.file();

        if file.path == path {
            file.save().map_err(SisterError::from)?;
        } else {
            // Write entity data to the target path
            let bytes = self.to_bytes()?;
            std::fs::write(path, &bytes)?;
        }
        Ok(())
    }

    fn to_bytes(&self) -> SisterResult<Vec<u8>> {
        // Serialize by saving to disk then reading the bytes
        let file = self.engine.file();
        file.save().map_err(SisterError::from)?;
        std::fs::read(&file.path).map_err(|e| {
            SisterError::new(
                ErrorCode::StorageError,
                format!("Failed to read .atime file: {e}"),
            )
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════

/// Simple word-overlap score between a query and text.
fn word_overlap_score(query: &str, text: &str) -> f64 {
    let query_words: std::collections::HashSet<String> = query
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .collect();

    if query_words.is_empty() {
        return 0.0;
    }

    let text_lower = text.to_lowercase();
    let matched = query_words
        .iter()
        .filter(|w| text_lower.contains(w.as_str()))
        .count();

    matched as f64 / query_words.len() as f64
}

// ═══════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Create a test TimeSister backed by a temp file.
    fn test_sister() -> (TimeSister, TempDir) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.atime");
        let config = SisterConfig {
            data_path: Some(path),
            create_if_missing: true,
            ..SisterConfig::stateless()
        };
        let sister = TimeSister::init(config).unwrap();
        (sister, dir)
    }

    /// Add a test deadline to the sister.
    fn add_test_deadline(sister: &mut TimeSister, label: &str) -> TemporalId {
        let deadline = Deadline::new(label.to_string(), Utc::now() + chrono::Duration::hours(24));
        let id = deadline.id;
        sister
            .engine
            .add_deadline(deadline)
            .expect("add_deadline failed");
        id
    }

    #[test]
    fn test_sister_trait() {
        let (sister, _dir) = test_sister();

        assert_eq!(TimeSister::SISTER_TYPE, SisterType::Time);
        assert_eq!(TimeSister::FILE_EXTENSION, "atime");

        let health = sister.health();
        assert!(health.healthy);
        assert_eq!(health.status, Status::Ready);

        let version = sister.version();
        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 0);

        let caps = sister.capabilities();
        assert!(caps.iter().any(|c| c.name == "time_deadline_add"));
        assert!(caps.iter().any(|c| c.name == "time_schedule_add"));
    }

    #[test]
    fn test_queryable_list() {
        let (mut sister, _dir) = test_sister();

        add_test_deadline(&mut sister, "Deploy v2.0");
        add_test_deadline(&mut sister, "Submit report");

        let result = sister.query(Query::list()).unwrap();
        assert!(result.results.len() >= 2);
    }

    #[test]
    fn test_queryable_search() {
        let (mut sister, _dir) = test_sister();

        add_test_deadline(&mut sister, "Deploy application to production");
        add_test_deadline(&mut sister, "Fix critical bug");
        add_test_deadline(&mut sister, "Update deployment pipeline");

        let result = sister.query(Query::search("deploy")).unwrap();
        assert!(!result.results.is_empty());
        assert!(result.results.iter().any(|r| {
            r.get("label")
                .and_then(|l| l.as_str())
                .map(|l| l.to_lowercase().contains("deploy"))
                .unwrap_or(false)
        }));
    }

    #[test]
    fn test_queryable_get() {
        let (mut sister, _dir) = test_sister();

        let id = add_test_deadline(&mut sister, "Test deadline for get");

        let result = sister.query(Query::get(id.to_string())).unwrap();
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0]["id"].as_str().unwrap(), id.to_string());
    }

    #[test]
    fn test_error_bridge() {
        let err: SisterError = TimeError::NotFound("test-id".into()).into();
        assert_eq!(err.code, ErrorCode::NotFound);

        let err: SisterError = TimeError::InvalidDuration("negative".into()).into();
        assert_eq!(err.code, ErrorCode::InvalidInput);

        let err: SisterError = TimeError::FileFormat("bad header".into()).into();
        assert_eq!(err.code, ErrorCode::VersionMismatch);

        let err: SisterError = TimeError::DeadlinePassed("old deadline".into()).into();
        assert_eq!(err.code, ErrorCode::InvalidState);
    }

    #[test]
    fn test_shutdown() {
        let (mut sister, _dir) = test_sister();
        add_test_deadline(&mut sister, "Before shutdown");
        sister.shutdown().unwrap();
    }

    #[test]
    fn test_config_patterns() {
        let config = SisterConfig::stateless();
        assert!(config.data_path.is_none());
        assert!(config.create_if_missing);

        let dir = TempDir::new().unwrap();
        let config = SisterConfig::new(dir.path().join("test.atime"));
        assert!(config.data_path.is_some());
    }

    #[test]
    fn test_file_format_reader() {
        let (_sister, dir) = test_sister();
        let path = dir.path().join("test.atime");

        // can_read should succeed
        let info = TimeSister::can_read(&path).unwrap();
        assert_eq!(info.sister_type, SisterType::Time);
        assert_eq!(info.format_id, "ATIM");

        // file_version should return v0.1.0
        let version = TimeSister::file_version(&path).unwrap();
        assert_eq!(version.major, 0);
    }

    #[test]
    fn test_supports_query() {
        let (sister, _dir) = test_sister();
        assert!(sister.supports_query("list"));
        assert!(sister.supports_query("search"));
        assert!(sister.supports_query("recent"));
        assert!(sister.supports_query("get"));
        assert!(!sister.supports_query("aggregate"));
    }
}
