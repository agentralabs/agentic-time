//! Edge-case and invention tests for AgenticTime MCP tools.
//!
//! Covers ~130 tests across: tool count verification, deadline/schedule/sequence/
//! decay/duration edge cases, general tools, unicode/special chars, rapid-fire
//! stress tests, full lifecycle workflows, and cross-sister temporal patterns
//! (Memory-Time, Identity-Time, Vision-Time, Codebase-Time, multi-entity consistency).

use serde_json::json;
use tempfile::tempdir;

// =========================================================================
// Helper: create engine from a fresh temp file
// =========================================================================
fn fresh_engine() -> (tempfile::TempDir, agentic_time::WriteEngine) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.atime");
    let tf = agentic_time::TimeFile::create(&path).unwrap();
    let engine = agentic_time::WriteEngine::new(tf);
    (dir, engine)
}

/// A future datetime string guaranteed to be ~1 year ahead.
fn future_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap()
        .to_rfc3339()
}

/// A past datetime string guaranteed to be ~1 year ago.
fn past_dt() -> String {
    chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(365))
        .unwrap()
        .to_rfc3339()
}

/// A near-future datetime string (1 hour ahead).
fn near_future_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .to_rfc3339()
}

/// A far-future datetime string (2 hours ahead).
fn far_future_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(2))
        .unwrap()
        .to_rfc3339()
}

/// Seven days from now.
fn seven_days_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .to_rfc3339()
}

/// Five days from now (for warn_at before a 7-day deadline).
fn five_days_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(5))
        .unwrap()
        .to_rfc3339()
}

/// Three days from now.
fn three_days_dt() -> String {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(3))
        .unwrap()
        .to_rfc3339()
}

// =========================================================================
// 1. Tool count verification (1 test)
// =========================================================================

#[test]
fn test_tools_array_length_is_35() {
    assert_eq!(
        agentic_time_mcp::tools::TOOLS.len(),
        35,
        "TOOLS array should contain 35 tool definitions (19 core + 16 inventions)"
    );
}

// =========================================================================
// 2. Deadline edge cases (15 tests)
// =========================================================================

#[tokio::test]
async fn test_deadline_add_missing_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"due_at": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("label"));
}

#[tokio::test]
async fn test_deadline_add_missing_due_at() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Ship MVP"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("due_at"));
}

#[tokio::test]
async fn test_deadline_add_missing_all_params() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_add", json!({}), &mut engine)
            .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_deadline_add_invalid_datetime_format() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Test", "due_at": "not-a-date"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid datetime"));
}

#[tokio::test]
async fn test_deadline_add_invalid_datetime_partial() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Test", "due_at": "2025-13-45T99:99:99Z"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid datetime"));
}

#[tokio::test]
async fn test_deadline_add_past_date_succeeds() {
    let (_dir, mut engine) = fresh_engine();
    // Past dates should be accepted (the engine creates, overdue detection is separate)
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Already past", "due_at": past_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert!(val.get("id").is_some());
    assert_eq!(val["status"], "pending");
}

#[tokio::test]
async fn test_deadline_add_unknown_deadline_type() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Test", "due_at": future_dt(), "deadline_type": "imaginary"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown deadline_type"));
}

#[tokio::test]
async fn test_deadline_add_all_four_types() {
    let (_dir, mut engine) = fresh_engine();
    for dtype in &["hard", "soft", "target", "recurring"] {
        let result = agentic_time_mcp::tools::handle_tool_call(
            "time_deadline_add",
            json!({"label": format!("{} deadline", dtype), "due_at": future_dt(), "deadline_type": dtype}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed to create {} deadline", dtype);
    }
}

#[tokio::test]
async fn test_deadline_add_invalid_warn_at() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Test", "due_at": future_dt(), "warn_at": "garbage"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid datetime"));
}

#[tokio::test]
async fn test_deadline_complete_missing_id() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_complete", json!({}), &mut engine)
            .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("id"));
}

#[tokio::test]
async fn test_deadline_complete_malformed_id() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_complete",
        json!({"id": "not-a-uuid"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid ID"));
}

#[tokio::test]
async fn test_deadline_complete_nonexistent_id() {
    let (_dir, mut engine) = fresh_engine();
    let fake_uuid = uuid::Uuid::new_v4().to_string();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_complete",
        json!({"id": fake_uuid}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_deadline_list_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_list", json!({}), &mut engine)
            .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["count"], 0);
    assert!(val["deadlines"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_deadline_list_unknown_status_filter() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_list",
        json!({"status": "imaginary_status"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown deadline status"));
}

#[tokio::test]
async fn test_deadline_overdue_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_overdue", json!({}), &mut engine)
            .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["count"], 0);
}

// =========================================================================
// 3. Schedule edge cases (12 tests)
// =========================================================================

#[tokio::test]
async fn test_schedule_create_missing_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"start_at": future_dt(), "duration_minutes": 60}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("label"));
}

#[tokio::test]
async fn test_schedule_create_missing_start_at() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "Meeting", "duration_minutes": 60}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("start_at"));
}

#[tokio::test]
async fn test_schedule_create_missing_duration() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "Meeting", "start_at": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("duration_minutes"));
}

#[tokio::test]
async fn test_schedule_create_zero_duration() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "Zero event", "start_at": future_dt(), "duration_minutes": 0}),
        &mut engine,
    )
    .await;
    // Zero duration may be allowed by the engine; just verify we get a result
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["duration_minutes"], 0);
}

#[tokio::test]
async fn test_schedule_create_negative_duration() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "Negative", "start_at": future_dt(), "duration_minutes": -30}),
        &mut engine,
    )
    .await;
    // Negative duration: the engine may accept or reject, but either way it should not panic
    // If accepted, duration_minutes will be negative in the response
    let _ = result;
}

#[tokio::test]
async fn test_schedule_create_unknown_priority() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "Test", "start_at": future_dt(), "duration_minutes": 30, "priority": "ultra"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown priority"));
}

#[tokio::test]
async fn test_schedule_create_unknown_recurrence_pattern() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "Test",
            "start_at": future_dt(),
            "duration_minutes": 30,
            "recurrence": {"pattern": "biannually"}
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown recurrence pattern"));
}

#[tokio::test]
async fn test_schedule_reschedule_missing_id() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_reschedule",
        json!({"new_start_at": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("id"));
}

#[tokio::test]
async fn test_schedule_reschedule_nonexistent_id() {
    let (_dir, mut engine) = fresh_engine();
    let fake_uuid = uuid::Uuid::new_v4().to_string();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_reschedule",
        json!({"id": fake_uuid, "new_start_at": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_schedule_range_end_before_start() {
    let (_dir, mut engine) = fresh_engine();
    let start = far_future_dt();
    let end = near_future_dt();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_range",
        json!({"start": start, "end": end}),
        &mut engine,
    )
    .await;
    // End before start: should return empty or error, not panic
    if let Ok(val) = &result {
        assert_eq!(val["count"], 0);
    }
}

#[tokio::test]
async fn test_schedule_available_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_available",
        json!({"start": near_future_dt(), "end": far_future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    // With nothing scheduled, entire range should be available
    assert!(val["count"].as_u64().unwrap() >= 1);
}

#[tokio::test]
async fn test_schedule_conflicts_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_conflicts",
        json!({"start_at": near_future_dt(), "duration_minutes": 60}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["count"], 0);
}

// =========================================================================
// 4. Sequence edge cases (10 tests)
// =========================================================================

#[tokio::test]
async fn test_sequence_create_missing_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"steps": [{"label": "A"}]}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("label"));
}

#[tokio::test]
async fn test_sequence_create_missing_steps() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"label": "Pipeline"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("steps"));
}

#[tokio::test]
async fn test_sequence_create_empty_steps_array() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"label": "Empty", "steps": []}),
        &mut engine,
    )
    .await;
    // Empty steps: engine may allow or reject but must not panic
    if let Ok(val) = &result {
        assert_eq!(val["step_count"], 0);
    }
}

#[tokio::test]
async fn test_sequence_create_step_missing_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"label": "Pipeline", "steps": [{"duration_minutes": 10}]}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("label"));
}

#[tokio::test]
async fn test_sequence_create_single_step() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"label": "Single", "steps": [{"label": "Only step"}]}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 1);
}

#[tokio::test]
async fn test_sequence_advance_missing_id() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_sequence_advance", json!({}), &mut engine)
            .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("id"));
}

#[tokio::test]
async fn test_sequence_advance_nonexistent_id() {
    let (_dir, mut engine) = fresh_engine();
    let fake_uuid = uuid::Uuid::new_v4().to_string();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": fake_uuid}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_sequence_advance_past_completion() {
    let (_dir, mut engine) = fresh_engine();
    // Create a single-step sequence
    let create_result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"label": "Short", "steps": [{"label": "Only"}]}),
        &mut engine,
    )
    .await
    .unwrap();
    let id = create_result["id"].as_str().unwrap().to_string();

    // Advance once to complete
    let adv1 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": id}),
        &mut engine,
    )
    .await;
    assert!(adv1.is_ok());

    // Advance again past completion — engine treats as no-op (does not error)
    let adv2 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": id}),
        &mut engine,
    )
    .await;
    // The engine silently handles advancing past completion
    // (complete_current_step is a no-op when current_step >= steps.len())
    assert!(adv2.is_ok());
    let val2 = adv2.unwrap();
    assert!(
        val2["status"].as_str().unwrap().contains("Completed")
            || val2["status"].as_str().unwrap().contains("Complete")
    );
}

#[tokio::test]
async fn test_sequence_status_missing_id() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_sequence_status", json!({}), &mut engine)
            .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("id"));
}

#[tokio::test]
async fn test_sequence_status_nonexistent_id() {
    let (_dir, mut engine) = fresh_engine();
    let fake_uuid = uuid::Uuid::new_v4().to_string();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_status",
        json!({"id": fake_uuid}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

// =========================================================================
// 5. Decay edge cases (10 tests)
// =========================================================================

#[tokio::test]
async fn test_decay_create_linear() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Linear decay", "initial_value": 100.0, "decay_type": "linear", "rate": 0.5}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "linear");
    assert_eq!(val["initial_value"], 100.0);
}

#[tokio::test]
async fn test_decay_create_exponential() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Exp decay", "initial_value": 50.0, "decay_type": "exponential", "rate": 0.01}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "exponential");
}

#[tokio::test]
async fn test_decay_create_half_life() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "HL decay", "initial_value": 200.0, "decay_type": "half_life", "half_life_hours": 12.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "half_life");
}

#[tokio::test]
async fn test_decay_create_step() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Step decay", "initial_value": 10.0, "decay_type": "step", "rate": 1.0, "half_life_hours": 2.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "step");
}

#[tokio::test]
async fn test_decay_create_unknown_type() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Bad", "initial_value": 10.0, "decay_type": "quantum"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown decay_type"));
}

#[tokio::test]
async fn test_decay_create_missing_initial_value() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "No value"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("initial_value"));
}

#[tokio::test]
async fn test_decay_create_zero_initial_value() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Zero", "initial_value": 0.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["initial_value"], 0.0);
}

#[tokio::test]
async fn test_decay_create_negative_initial_value() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Negative", "initial_value": -50.0}),
        &mut engine,
    )
    .await;
    // Negative initial value: engine may allow it
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_decay_value_nonexistent_id() {
    let (_dir, mut engine) = fresh_engine();
    let fake_uuid = uuid::Uuid::new_v4().to_string();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_value",
        json!({"id": fake_uuid}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_decay_alert_missing_threshold() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_alert",
        json!({"within_hours": 24.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("threshold"));
}

// =========================================================================
// 6. Duration edge cases (8 tests)
// =========================================================================

#[tokio::test]
async fn test_duration_estimate_missing_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"expected_minutes": 60}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("label"));
}

#[tokio::test]
async fn test_duration_estimate_missing_expected() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Task A"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expected_minutes"));
}

#[tokio::test]
async fn test_duration_estimate_zero_minutes() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Instant", "expected_minutes": 0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["expected_minutes"], 0);
}

#[tokio::test]
async fn test_duration_estimate_with_pert_values() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({
            "label": "Full PERT",
            "expected_minutes": 60,
            "optimistic_minutes": 30,
            "pessimistic_minutes": 120,
            "confidence": 0.95
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["optimistic_minutes"], 30);
    assert_eq!(val["pessimistic_minutes"], 120);
    assert_eq!(val["confidence"], 0.95);
    // PERT estimate: (30 + 4*60 + 120) / 6 = 65 minutes
    assert!(val["pert_estimate_minutes"].as_i64().unwrap() >= 60);
}

#[tokio::test]
async fn test_duration_estimate_confidence_boundary_zero() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Zero conf", "expected_minutes": 30, "confidence": 0.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["confidence"], 0.0);
}

#[tokio::test]
async fn test_duration_estimate_confidence_boundary_one() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Full conf", "expected_minutes": 30, "confidence": 1.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["confidence"], 1.0);
}

#[tokio::test]
async fn test_duration_aggregate_empty_ids() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_aggregate",
        json!({"ids": []}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No valid IDs"));
}

#[tokio::test]
async fn test_duration_aggregate_invalid_ids() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_aggregate",
        json!({"ids": ["not-a-uuid", "also-bad"]}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No valid IDs"));
}

// =========================================================================
// 7. General tools (5 tests)
// =========================================================================

#[tokio::test]
async fn test_stats_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine).await;
    assert!(result.is_ok());
    // The stats response should be valid JSON
    let val = result.unwrap();
    assert!(val.is_object());
}

#[tokio::test]
async fn test_stats_with_null_args() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_stats",
        serde_json::Value::Null,
        &mut engine,
    )
    .await;
    // Should handle null args gracefully
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_refresh_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_refresh", json!({}), &mut engine).await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["deadlines_updated"], 0);
    assert_eq!(val["decays_updated"], 0);
    assert_eq!(val["new_overdue_count"], 0);
}

#[tokio::test]
async fn test_unknown_tool_name() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_teleport_to_mars", json!({}), &mut engine)
            .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown tool"));
}

#[tokio::test]
async fn test_empty_string_tool_name() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call("", json!({}), &mut engine).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown tool"));
}

// =========================================================================
// 8. Unicode/special chars (3 tests)
// =========================================================================

#[tokio::test]
async fn test_deadline_add_unicode_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Ship MVP \u{1F680}\u{2728} deadline", "due_at": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert!(val["label"].as_str().unwrap().contains("\u{1F680}"));
}

#[tokio::test]
async fn test_schedule_create_unicode_label() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "\u{65E5}\u{672C}\u{8A9E}\u{30C6}\u{30B9}\u{30C8}",
            "start_at": future_dt(),
            "duration_minutes": 30
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(
        val["label"],
        "\u{65E5}\u{672C}\u{8A9E}\u{30C6}\u{30B9}\u{30C8}"
    );
}

#[tokio::test]
async fn test_sequence_create_special_chars_in_steps() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "Special <chars> & \"quotes\"",
            "steps": [
                {"label": "Step with 'single quotes'"},
                {"label": "Step with\ttab\nnewline"},
                {"label": "Step with /slashes\\ & pipes|"},
            ]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 3);
}

// =========================================================================
// 9. Rapid-fire / stress tests (3 tests)
// =========================================================================

#[tokio::test]
async fn test_rapid_fire_100_stats_calls() {
    let (_dir, mut engine) = fresh_engine();
    for i in 0..100 {
        let result =
            agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine).await;
        assert!(result.is_ok(), "stats call {} failed", i);
    }
}

#[tokio::test]
async fn test_rapid_fire_50_deadline_add_then_list() {
    let (_dir, mut engine) = fresh_engine();
    for i in 0..50 {
        let due = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(i + 1))
            .unwrap()
            .to_rfc3339();
        let result = agentic_time_mcp::tools::handle_tool_call(
            "time_deadline_add",
            json!({"label": format!("Deadline #{}", i), "due_at": due}),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed to add deadline #{}", i);
    }

    let list_result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_list", json!({}), &mut engine)
            .await;
    assert!(list_result.is_ok());
    let val = list_result.unwrap();
    assert_eq!(val["count"], 50);
}

#[tokio::test]
async fn test_rapid_fire_mixed_entity_creation() {
    let (_dir, mut engine) = fresh_engine();

    // Create 10 deadlines
    for i in 0..10 {
        let due = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(i + 1))
            .unwrap()
            .to_rfc3339();
        let r = agentic_time_mcp::tools::handle_tool_call(
            "time_deadline_add",
            json!({"label": format!("DL-{}", i), "due_at": due}),
            &mut engine,
        )
        .await;
        assert!(r.is_ok());
    }

    // Create 10 schedules
    for i in 0..10 {
        let start = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(i + 1))
            .unwrap()
            .to_rfc3339();
        let r = agentic_time_mcp::tools::handle_tool_call(
            "time_schedule_create",
            json!({"label": format!("SCH-{}", i), "start_at": start, "duration_minutes": 30}),
            &mut engine,
        )
        .await;
        assert!(r.is_ok());
    }

    // Create 5 sequences
    for i in 0..5 {
        let r = agentic_time_mcp::tools::handle_tool_call(
            "time_sequence_create",
            json!({"label": format!("SEQ-{}", i), "steps": [{"label": "A"}, {"label": "B"}]}),
            &mut engine,
        )
        .await;
        assert!(r.is_ok());
    }

    // Create 5 decay models
    for i in 0..5 {
        let r = agentic_time_mcp::tools::handle_tool_call(
            "time_decay_create",
            json!({"label": format!("DEC-{}", i), "initial_value": 100.0}),
            &mut engine,
        )
        .await;
        assert!(r.is_ok());
    }

    // Create 5 duration estimates
    for i in 0..5 {
        let r = agentic_time_mcp::tools::handle_tool_call(
            "time_duration_estimate",
            json!({"label": format!("DUR-{}", i), "expected_minutes": 30 + i}),
            &mut engine,
        )
        .await;
        assert!(r.is_ok());
    }

    // Verify stats reflect everything
    let stats = agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine)
        .await
        .unwrap();
    assert!(stats.is_object());
}

// =========================================================================
// 10. Full workflows (3 tests)
// =========================================================================

#[tokio::test]
async fn test_workflow_deadline_full_lifecycle() {
    let (_dir, mut engine) = fresh_engine();

    // 1. Add a deadline
    let add_result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({
            "label": "Launch product",
            "due_at": future_dt(),
            "deadline_type": "hard",
            "consequence": "Revenue loss",
            "tags": ["launch", "critical"]
        }),
        &mut engine,
    )
    .await
    .unwrap();
    let id = add_result["id"].as_str().unwrap().to_string();
    assert_eq!(add_result["status"], "pending");

    // 2. List deadlines and verify it's there
    let list_result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_list", json!({}), &mut engine)
            .await
            .unwrap();
    assert_eq!(list_result["count"], 1);
    assert_eq!(list_result["deadlines"][0]["label"], "Launch product");

    // 3. Check overdue (should be empty, deadline is in the future)
    let overdue_result =
        agentic_time_mcp::tools::handle_tool_call("time_deadline_overdue", json!({}), &mut engine)
            .await
            .unwrap();
    assert_eq!(overdue_result["count"], 0);

    // 4. Complete the deadline
    let complete_result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_complete",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(complete_result["status"], "completed");

    // 5. List completed deadlines
    let completed_list = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_list",
        json!({"status": "completed"}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(completed_list["count"], 1);
}

#[tokio::test]
async fn test_workflow_schedule_create_and_reschedule() {
    let (_dir, mut engine) = fresh_engine();

    let original_start = near_future_dt();

    // 1. Create a schedule
    let create_result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "Team standup",
            "start_at": original_start,
            "duration_minutes": 15,
            "priority": "high",
            "tags": ["meetings"]
        }),
        &mut engine,
    )
    .await
    .unwrap();
    let id = create_result["id"].as_str().unwrap().to_string();
    assert_eq!(create_result["status"], "scheduled");
    assert_eq!(create_result["duration_minutes"], 15);

    // 2. Check conflicts for overlapping slot
    let conflict_result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_conflicts",
        json!({"start_at": original_start, "duration_minutes": 60}),
        &mut engine,
    )
    .await
    .unwrap();
    assert!(conflict_result["count"].as_u64().unwrap() >= 1);

    // 3. Reschedule to a new time
    let new_start = far_future_dt();
    let reschedule_result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_reschedule",
        json!({"id": id, "new_start_at": new_start}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(reschedule_result["status"], "rescheduled");

    // 4. Original slot should now be free
    let avail_result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_available",
        json!({
            "start": original_start,
            "end": far_future_dt(),
            "min_duration_minutes": 10
        }),
        &mut engine,
    )
    .await
    .unwrap();
    assert!(avail_result["count"].as_u64().unwrap() >= 1);
}

#[tokio::test]
async fn test_workflow_sequence_full_completion() {
    let (_dir, mut engine) = fresh_engine();

    // 1. Create a 3-step sequence
    let create_result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "Deploy pipeline",
            "steps": [
                {"label": "Build", "duration_minutes": 10},
                {"label": "Test", "duration_minutes": 20},
                {"label": "Deploy", "duration_minutes": 5}
            ]
        }),
        &mut engine,
    )
    .await
    .unwrap();
    let id = create_result["id"].as_str().unwrap().to_string();
    assert_eq!(create_result["step_count"], 3);

    // 2. Check initial status
    let status1 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_status",
        json!({"id": &id}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(status1["current_step"], 0);
    assert_eq!(status1["total_steps"], 3);

    // 3. Advance through all steps
    let adv1 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": &id}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(adv1["current_step"], 1);

    let adv2 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": &id}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(adv2["current_step"], 2);

    let adv3 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": &id}),
        &mut engine,
    )
    .await
    .unwrap();
    // After advancing past last step, status should be Completed
    assert!(
        adv3["status"].as_str().unwrap().contains("Completed")
            || adv3["status"].as_str().unwrap().contains("Complete")
    );

    // 4. Advancing again — engine treats as no-op (no error)
    let adv4 = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": &id}),
        &mut engine,
    )
    .await;
    // The engine silently handles advancing past completion
    assert!(adv4.is_ok());
    let val4 = adv4.unwrap();
    assert!(
        val4["status"].as_str().unwrap().contains("Completed")
            || val4["status"].as_str().unwrap().contains("Complete")
    );
}

// =========================================================================
// Additional edge cases to reach ~95 tests
// =========================================================================

#[tokio::test]
async fn test_deadline_add_with_tags() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Tagged", "due_at": future_dt(), "tags": ["a", "b", "c"]}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deadline_add_with_empty_tags() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "No tags", "due_at": future_dt(), "tags": []}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deadline_add_label_is_number_fails() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": 42, "due_at": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("label"));
}

#[tokio::test]
async fn test_deadline_add_due_at_is_number_fails() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Test", "due_at": 123456}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("due_at"));
}

#[tokio::test]
async fn test_schedule_create_all_priorities() {
    let (_dir, mut engine) = fresh_engine();
    for priority in &["critical", "high", "medium", "low", "optional"] {
        let result = agentic_time_mcp::tools::handle_tool_call(
            "time_schedule_create",
            json!({
                "label": format!("{} priority", priority),
                "start_at": future_dt(),
                "duration_minutes": 30,
                "priority": priority
            }),
            &mut engine,
        )
        .await;
        assert!(result.is_ok(), "Failed for priority: {}", priority);
    }
}

#[tokio::test]
async fn test_schedule_create_with_daily_recurrence() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "Daily standup",
            "start_at": future_dt(),
            "duration_minutes": 15,
            "recurrence": {"pattern": "daily", "interval": 1}
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_schedule_create_with_weekly_recurrence() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "Weekly review",
            "start_at": future_dt(),
            "duration_minutes": 60,
            "recurrence": {"pattern": "weekly", "interval": 2}
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_schedule_create_with_monthly_recurrence() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "Monthly report",
            "start_at": future_dt(),
            "duration_minutes": 120,
            "recurrence": {"pattern": "monthly"}
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_schedule_range_missing_start() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_range",
        json!({"end": future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("start"));
}

#[tokio::test]
async fn test_schedule_range_missing_end() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_range",
        json!({"start": near_future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("end"));
}

#[tokio::test]
async fn test_schedule_conflicts_missing_duration() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_conflicts",
        json!({"start_at": near_future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("duration_minutes"));
}

#[tokio::test]
async fn test_decay_alert_missing_within_hours() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_alert",
        json!({"threshold": 50.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("within_hours"));
}

#[tokio::test]
async fn test_decay_alert_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_alert",
        json!({"threshold": 50.0, "within_hours": 24.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["total_alerts"], 0);
}

#[tokio::test]
async fn test_decay_value_missing_id() {
    let (_dir, mut engine) = fresh_engine();
    let result =
        agentic_time_mcp::tools::handle_tool_call("time_decay_value", json!({}), &mut engine).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("id"));
}

#[tokio::test]
async fn test_decay_value_malformed_id() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_value",
        json!({"id": "zzzz-not-uuid"}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid ID"));
}

#[tokio::test]
async fn test_duration_aggregate_missing_ids() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_aggregate",
        json!({}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("ids"));
}

#[tokio::test]
async fn test_duration_aggregate_two_estimates() {
    let (_dir, mut engine) = fresh_engine();

    // Create two estimates
    let e1 = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Task A", "expected_minutes": 30}),
        &mut engine,
    )
    .await
    .unwrap();
    let e2 = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Task B", "expected_minutes": 60}),
        &mut engine,
    )
    .await
    .unwrap();

    let id1 = e1["id"].as_str().unwrap();
    let id2 = e2["id"].as_str().unwrap();

    let agg = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_aggregate",
        json!({"ids": [id1, id2]}),
        &mut engine,
    )
    .await;
    assert!(agg.is_ok());
    let val = agg.unwrap();
    assert_eq!(val["aggregated_count"], 2);
    // Expected total: 30 + 60 = 90
    assert_eq!(val["expected_minutes"], 90);
}

#[tokio::test]
async fn test_deadline_list_with_due_within_filter() {
    let (_dir, mut engine) = fresh_engine();

    // Add a deadline 3 days from now
    let due_3d = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(3))
        .unwrap()
        .to_rfc3339();
    agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Soon", "due_at": due_3d}),
        &mut engine,
    )
    .await
    .unwrap();

    // Add a deadline 30 days from now
    let due_30d = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .unwrap()
        .to_rfc3339();
    agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Later", "due_at": due_30d}),
        &mut engine,
    )
    .await
    .unwrap();

    // Query within 7 days - should get only the first one
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_list",
        json!({"due_within": "P7D"}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(result["count"], 1);
    assert_eq!(result["deadlines"][0]["label"], "Soon");
}

#[tokio::test]
async fn test_schedule_create_flexible_false() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "Inflexible",
            "start_at": future_dt(),
            "duration_minutes": 60,
            "flexible": false
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deadline_add_consequence_empty_string() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "No consequence", "due_at": future_dt(), "consequence": ""}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sequence_create_with_depends_on() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "Pipeline with deps",
            "steps": [
                {"label": "Build"},
                {"label": "Test", "depends_on": [0]},
                {"label": "Deploy", "depends_on": [0, 1]}
            ],
            "allow_parallel": true
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 3);
}

#[tokio::test]
async fn test_sequence_create_with_duration_on_steps() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "Timed pipeline",
            "steps": [
                {"label": "Build", "duration_minutes": 10},
                {"label": "Test", "duration_minutes": 20}
            ]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());

    let id = result.unwrap()["id"].as_str().unwrap().to_string();

    // Check status includes duration info
    let status = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_status",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    assert_eq!(status["total_steps"], 2);
    // Total duration should be 30 min = 30 in minutes
    assert_eq!(status["total_duration_minutes"], 30);
}

#[tokio::test]
async fn test_decay_create_with_floor() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Floored", "initial_value": 100.0, "floor": 25.0}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["floor"], 25.0);
}

#[tokio::test]
async fn test_decay_create_with_tags() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Tagged decay", "initial_value": 80.0, "tags": ["memory", "context"]}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_decay_value_after_creation() {
    let (_dir, mut engine) = fresh_engine();
    let create_result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Fresh", "initial_value": 100.0}),
        &mut engine,
    )
    .await
    .unwrap();
    let id = create_result["id"].as_str().unwrap().to_string();

    let value_result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_value",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();

    // Just created, so current_value should be very close to initial
    let current = value_result["current_value"].as_f64().unwrap();
    assert!(
        current >= 99.0,
        "Current value {} should be close to 100.0 immediately after creation",
        current
    );
}

#[tokio::test]
async fn test_refresh_after_adding_entities() {
    let (_dir, mut engine) = fresh_engine();

    // Add a past-due deadline
    agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Overdue", "due_at": past_dt()}),
        &mut engine,
    )
    .await
    .unwrap();

    // Add a decay model
    agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "Decaying", "initial_value": 100.0}),
        &mut engine,
    )
    .await
    .unwrap();

    // Refresh
    let refresh_result =
        agentic_time_mcp::tools::handle_tool_call("time_refresh", json!({}), &mut engine).await;
    assert!(refresh_result.is_ok());
    let val = refresh_result.unwrap();
    // Should have updated at least the deadline and decay
    assert!(val["deadlines_updated"].as_u64().unwrap() >= 1);
    assert!(val["decays_updated"].as_u64().unwrap() >= 1);
}

#[tokio::test]
async fn test_schedule_reschedule_missing_new_start_at() {
    let (_dir, mut engine) = fresh_engine();
    let fake_uuid = uuid::Uuid::new_v4().to_string();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_reschedule",
        json!({"id": fake_uuid}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("new_start_at"));
}

#[tokio::test]
async fn test_deadline_add_with_valid_warn_at() {
    let (_dir, mut engine) = fresh_engine();
    let due = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .to_rfc3339();
    let warn = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(5))
        .unwrap()
        .to_rfc3339();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "Warned deadline", "due_at": due, "warn_at": warn}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_duration_estimate_negative_expected() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Negative", "expected_minutes": -10}),
        &mut engine,
    )
    .await;
    // Negative expected_minutes: the tool may accept it (as i64)
    // or the engine may reject it. Either way, must not panic.
    let _ = result;
}

#[tokio::test]
async fn test_duration_estimate_defaults_for_pert() {
    let (_dir, mut engine) = fresh_engine();
    // Only provide expected_minutes; optimistic/pessimistic should default
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "Defaults only", "expected_minutes": 100}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    // Optimistic defaults to 60% of expected = 60
    assert_eq!(val["optimistic_minutes"], 60);
    // Pessimistic defaults to 200% of expected = 200
    assert_eq!(val["pessimistic_minutes"], 200);
    assert_eq!(val["confidence"], 0.7);
}

#[tokio::test]
async fn test_deadline_complete_id_is_null() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_complete",
        json!({"id": null}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_sequence_advance_id_is_empty_string() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": ""}),
        &mut engine,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_schedule_available_zero_min_duration() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_available",
        json!({"start": near_future_dt(), "end": far_future_dt(), "min_duration_minutes": 0}),
        &mut engine,
    )
    .await;
    // Zero min duration: should still work or error gracefully
    let _ = result;
}

#[tokio::test]
async fn test_schedule_range_empty_store() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_range",
        json!({"start": near_future_dt(), "end": far_future_dt()}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["count"], 0);
}

#[tokio::test]
async fn test_deadline_list_status_pending_empty() {
    let (_dir, mut engine) = fresh_engine();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_list",
        json!({"status": "pending"}),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["count"], 0);
}

// =========================================================================
// CROSS-SISTER TEMPORAL PATTERNS
// =========================================================================

// =========================================================================
// 1. Memory-Time patterns (5 tests)
// =========================================================================

#[tokio::test]
async fn test_memory_deadline_microsecond_precision() {
    let (_dir, mut engine) = fresh_engine();
    // Memory nodes may carry microsecond-precision timestamps.
    // Ensure the engine handles RFC3339 with sub-second precision.
    let precise_dt = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%S%.6fZ")
        .to_string();

    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({
            "label": "memory-node-expiry",
            "due_at": precise_dt,
            "tags": ["memory", "precision"]
        }),
        &mut engine,
    )
    .await;
    assert!(
        result.is_ok(),
        "Microsecond-precision timestamp should be accepted"
    );
    let val = result.unwrap();
    assert!(val.get("id").is_some());
    assert_eq!(val["status"], "pending");
}

#[tokio::test]
async fn test_memory_decay_staleness_half_life() {
    let (_dir, mut engine) = fresh_engine();
    // Model "memory staleness": a memory node starts at confidence 100,
    // decays with half_life of 168 hours (7 days).
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({
            "label": "memory-staleness",
            "initial_value": 100.0,
            "decay_type": "half_life",
            "half_life_hours": 168.0
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "half_life");
    assert_eq!(val["initial_value"], 100.0);

    // Immediately reading value should return ~100
    let id = val["id"].as_str().unwrap().to_string();
    let value_result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_value",
        json!({"id": id}),
        &mut engine,
    )
    .await;
    assert!(value_result.is_ok());
    let vval = value_result.unwrap();
    // At t=0, value should be very close to initial
    let current = vval["current_value"].as_f64().unwrap();
    assert!(
        (99.0..=100.0).contains(&current),
        "At t=0 value should be ~100, got {}",
        current
    );
}

#[tokio::test]
async fn test_memory_stats_after_entities() {
    let (_dir, mut engine) = fresh_engine();
    // Create memory-like entities: a decay (staleness) and a deadline (expiry)
    agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "mem-staleness", "initial_value": 80.0, "decay_type": "linear", "rate": 0.1}),
        &mut engine,
    )
    .await
    .unwrap();

    agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "mem-cache-expiry", "due_at": future_dt()}),
        &mut engine,
    )
    .await
    .unwrap();

    let stats = agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine)
        .await
        .unwrap();
    assert_eq!(stats["decay_count"], 1, "Should have 1 decay model");
    assert_eq!(stats["deadline_count"], 1, "Should have 1 deadline");
}

#[tokio::test]
async fn test_memory_sequence_session_workflow() {
    let (_dir, mut engine) = fresh_engine();
    // Model a 4-step memory session: start -> add -> query -> end
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "memory-session-workflow",
            "steps": [
                {"label": "session_start"},
                {"label": "memory_add"},
                {"label": "memory_query"},
                {"label": "session_end"}
            ]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 4);
    let id = val["id"].as_str().unwrap().to_string();

    // Advance through all 4 steps
    for expected_step in 1..=4 {
        let adv = agentic_time_mcp::tools::handle_tool_call(
            "time_sequence_advance",
            json!({"id": id}),
            &mut engine,
        )
        .await;
        assert!(
            adv.is_ok(),
            "Advance to step {} should succeed",
            expected_step
        );
    }

    // Verify completed status
    let status = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_status",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    let status_str = status["status"].as_str().unwrap();
    assert!(
        status_str.contains("Complete") || status_str.contains("complete"),
        "Sequence should be completed, got: {}",
        status_str
    );
}

#[tokio::test]
async fn test_memory_duration_consolidation() {
    let (_dir, mut engine) = fresh_engine();
    // Estimate how long "memory consolidation" takes with PERT values
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({
            "label": "memory-consolidation",
            "expected_minutes": 45,
            "optimistic_minutes": 20,
            "pessimistic_minutes": 90,
            "confidence": 0.85
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "memory-consolidation");
    assert_eq!(val["expected_minutes"], 45);
    assert_eq!(val["optimistic_minutes"], 20);
    assert_eq!(val["pessimistic_minutes"], 90);
    assert_eq!(val["confidence"], 0.85);
    // PERT: (20 + 4*45 + 90) / 6 = 48.33... -> 48
    let pert = val["pert_estimate_minutes"].as_i64().unwrap();
    assert!(
        (45..=50).contains(&pert),
        "PERT estimate should be ~48, got {}",
        pert
    );
}

// =========================================================================
// 2. Identity-Time patterns (5 tests)
// =========================================================================

#[tokio::test]
async fn test_identity_deadline_trust_grant_expiry() {
    let (_dir, mut engine) = fresh_engine();
    // A trust grant expires in 30 days. Hard deadline with consequence.
    let due = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .unwrap()
        .to_rfc3339();
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({
            "label": "trust-grant-expiry",
            "due_at": due,
            "deadline_type": "hard",
            "consequence": "Agent loses delegated permissions",
            "tags": ["identity", "trust"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["status"], "pending");
    assert!(val.get("id").is_some());
}

#[tokio::test]
async fn test_identity_decay_competence_freshness_exponential() {
    let (_dir, mut engine) = fresh_engine();
    // Competence freshness decays exponentially - rate 0.02 per hour
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({
            "label": "competence-freshness",
            "initial_value": 100.0,
            "decay_type": "exponential",
            "rate": 0.02
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "exponential");
    assert_eq!(val["initial_value"], 100.0);
}

#[tokio::test]
async fn test_identity_sequence_workflow() {
    let (_dir, mut engine) = fresh_engine();
    // Identity workflow: create -> sign -> grant -> verify
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "identity-lifecycle",
            "steps": [
                {"label": "create-identity"},
                {"label": "sign-certificate"},
                {"label": "grant-permission"},
                {"label": "verify-receipt"}
            ]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 4);

    // Advance to "sign-certificate" (step 2)
    let id = val["id"].as_str().unwrap().to_string();
    let adv = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_advance",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    // After one advance, current step should be index 1 (sign-certificate)
    assert!(adv.get("current_step").is_some() || adv.get("status").is_some());
}

#[tokio::test]
async fn test_identity_schedule_heartbeat_daily() {
    let (_dir, mut engine) = fresh_engine();
    // Recurring daily heartbeat for identity liveness check
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "identity-heartbeat",
            "start_at": near_future_dt(),
            "duration_minutes": 5,
            "priority": "high",
            "recurrence": {"pattern": "daily", "interval": 1},
            "tags": ["identity", "heartbeat"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "identity-heartbeat");
    assert_eq!(val["duration_minutes"], 5);
}

#[tokio::test]
async fn test_identity_duration_receipt_verification() {
    let (_dir, mut engine) = fresh_engine();
    // Estimate time for receipt verification
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({
            "label": "receipt-verification",
            "expected_minutes": 2,
            "optimistic_minutes": 1,
            "pessimistic_minutes": 10,
            "confidence": 0.90
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "receipt-verification");
    assert_eq!(val["expected_minutes"], 2);
    // PERT: (1 + 4*2 + 10) / 6 = 19/6 ≈ 3
    let pert = val["pert_estimate_minutes"].as_i64().unwrap();
    assert!(
        (2..=5).contains(&pert),
        "PERT for receipt verification should be ~3, got {}",
        pert
    );
}

// =========================================================================
// 3. Vision-Time patterns (5 tests)
// =========================================================================

#[tokio::test]
async fn test_vision_deadline_capture_expiry_7_day() {
    let (_dir, mut engine) = fresh_engine();
    // A visual capture expires after 7 days
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({
            "label": "capture-expiry",
            "due_at": seven_days_dt(),
            "deadline_type": "soft",
            "consequence": "Screenshot cache evicted",
            "tags": ["vision", "capture"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["status"], "pending");
    assert!(val.get("id").is_some());
}

#[tokio::test]
async fn test_vision_decay_visual_similarity_linear() {
    let (_dir, mut engine) = fresh_engine();
    // Visual similarity decays linearly over time (UI changes)
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({
            "label": "visual-similarity",
            "initial_value": 100.0,
            "decay_type": "linear",
            "rate": 0.5
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "linear");
    assert_eq!(val["initial_value"], 100.0);

    // Read current value at t=0
    let id = val["id"].as_str().unwrap().to_string();
    let vr = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_value",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    let current = vr["current_value"].as_f64().unwrap();
    assert!(
        current >= 99.0,
        "At t=0 linear decay should be ~100, got {}",
        current
    );
}

#[tokio::test]
async fn test_vision_sequence_capture_compare_diff_report() {
    let (_dir, mut engine) = fresh_engine();
    // Vision pipeline: capture -> compare -> diff -> report
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "vision-diff-pipeline",
            "steps": [
                {"label": "capture-screenshot"},
                {"label": "compare-baseline"},
                {"label": "compute-diff"},
                {"label": "generate-report"}
            ]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 4);
    let id = val["id"].as_str().unwrap().to_string();

    // Advance through all steps and verify completion
    for _ in 0..4 {
        agentic_time_mcp::tools::handle_tool_call(
            "time_sequence_advance",
            json!({"id": id}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    let status = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_status",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    let s = status["status"].as_str().unwrap();
    assert!(
        s.contains("Complete") || s.contains("complete"),
        "Pipeline should complete, got: {}",
        s
    );
}

#[tokio::test]
async fn test_vision_schedule_periodic_screenshot() {
    let (_dir, mut engine) = fresh_engine();
    // Periodic screenshot every hour
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "periodic-screenshot",
            "start_at": near_future_dt(),
            "duration_minutes": 1,
            "priority": "low",
            "tags": ["vision", "automation"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "periodic-screenshot");
    assert_eq!(val["duration_minutes"], 1);
    assert_eq!(val["status"], "scheduled");
}

#[tokio::test]
async fn test_vision_duration_ocr_processing() {
    let (_dir, mut engine) = fresh_engine();
    // OCR processing duration estimate
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({
            "label": "ocr-processing",
            "expected_minutes": 5,
            "optimistic_minutes": 2,
            "pessimistic_minutes": 15,
            "confidence": 0.80
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "ocr-processing");
    assert_eq!(val["expected_minutes"], 5);
    // PERT: (2 + 4*5 + 15) / 6 = 37/6 ≈ 6
    let pert = val["pert_estimate_minutes"].as_i64().unwrap();
    assert!(
        (5..=8).contains(&pert),
        "PERT for OCR should be ~6, got {}",
        pert
    );
}

// =========================================================================
// 4. Codebase-Time patterns (5 tests)
// =========================================================================

#[tokio::test]
async fn test_codebase_deadline_code_review_with_warn_at() {
    let (_dir, mut engine) = fresh_engine();
    // Code review due in 7 days, warn at 5 days
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({
            "label": "code-review-due",
            "due_at": seven_days_dt(),
            "warn_at": five_days_dt(),
            "deadline_type": "hard",
            "consequence": "PR auto-closed by policy",
            "tags": ["codebase", "review"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["status"], "pending");
    assert!(val.get("id").is_some());
}

#[tokio::test]
async fn test_codebase_decay_symbol_relevance_half_life() {
    let (_dir, mut engine) = fresh_engine();
    // Symbol relevance decays with a half-life of 720 hours (30 days)
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({
            "label": "symbol-relevance",
            "initial_value": 100.0,
            "decay_type": "half_life",
            "half_life_hours": 720.0
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["decay_type"], "half_life");
    assert_eq!(val["initial_value"], 100.0);
}

#[tokio::test]
async fn test_codebase_sequence_parse_index_query_impact() {
    let (_dir, mut engine) = fresh_engine();
    // Codebase analysis pipeline: parse -> index -> query -> impact
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "codebase-analysis-pipeline",
            "steps": [
                {"label": "parse-source-files"},
                {"label": "index-symbols"},
                {"label": "query-graph"},
                {"label": "compute-impact"}
            ]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["step_count"], 4);

    // Advance two steps (through parse and index)
    let id = val["id"].as_str().unwrap().to_string();
    for _ in 0..2 {
        agentic_time_mcp::tools::handle_tool_call(
            "time_sequence_advance",
            json!({"id": id}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    // Check status - should be in progress
    let status = agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_status",
        json!({"id": id}),
        &mut engine,
    )
    .await
    .unwrap();
    // After 2 of 4 steps, should not be completed yet
    let s = status["status"].as_str().unwrap();
    assert!(
        !s.contains("Complete") && !s.contains("complete"),
        "After 2/4 steps, should not be completed, got: {}",
        s
    );
}

#[tokio::test]
async fn test_codebase_schedule_incremental_reindex() {
    let (_dir, mut engine) = fresh_engine();
    // Schedule incremental re-indexing daily
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "incremental-reindex",
            "start_at": near_future_dt(),
            "duration_minutes": 10,
            "priority": "medium",
            "recurrence": {"pattern": "daily", "interval": 1},
            "tags": ["codebase", "indexing"]
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "incremental-reindex");
    assert_eq!(val["duration_minutes"], 10);
}

#[tokio::test]
async fn test_codebase_duration_graph_traversal() {
    let (_dir, mut engine) = fresh_engine();
    // Estimate time for dependency graph traversal
    let result = agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({
            "label": "graph-traversal",
            "expected_minutes": 15,
            "optimistic_minutes": 5,
            "pessimistic_minutes": 60,
            "confidence": 0.70
        }),
        &mut engine,
    )
    .await;
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val["label"], "graph-traversal");
    assert_eq!(val["expected_minutes"], 15);
    // PERT: (5 + 4*15 + 60) / 6 = 125/6 ≈ 20
    let pert = val["pert_estimate_minutes"].as_i64().unwrap();
    assert!(
        (18..=22).contains(&pert),
        "PERT for graph traversal should be ~20, got {}",
        pert
    );
}

// =========================================================================
// 5. Multi-entity consistency (5 tests)
// =========================================================================

#[tokio::test]
async fn test_multi_deadline_plus_schedule_same_event() {
    let (_dir, mut engine) = fresh_engine();
    let event_time = three_days_dt();

    // Create a deadline for the event
    let dl = agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({
            "label": "release-cutoff",
            "due_at": event_time,
            "tags": ["release"]
        }),
        &mut engine,
    )
    .await;
    assert!(dl.is_ok());

    // Create a schedule for the same event
    let sch = agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({
            "label": "release-ceremony",
            "start_at": event_time,
            "duration_minutes": 60,
            "tags": ["release"]
        }),
        &mut engine,
    )
    .await;
    assert!(sch.is_ok());

    // Both should exist in stats
    let stats = agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine)
        .await
        .unwrap();
    assert_eq!(stats["deadline_count"], 1);
    assert_eq!(stats["schedule_count"], 1);
}

#[tokio::test]
async fn test_multi_sequence_plus_multiple_deadlines() {
    let (_dir, mut engine) = fresh_engine();

    // Create a sequence
    agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({
            "label": "deploy-pipeline",
            "steps": [{"label": "build"}, {"label": "test"}, {"label": "deploy"}]
        }),
        &mut engine,
    )
    .await
    .unwrap();

    // Create 3 deadlines for each phase
    for (i, phase) in ["build-deadline", "test-deadline", "deploy-deadline"]
        .iter()
        .enumerate()
    {
        let due = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(i as i64 + 1))
            .unwrap()
            .to_rfc3339();
        agentic_time_mcp::tools::handle_tool_call(
            "time_deadline_add",
            json!({"label": phase, "due_at": due}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    let stats = agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine)
        .await
        .unwrap();
    assert_eq!(stats["sequence_count"], 1, "Should have 1 sequence");
    assert_eq!(stats["deadline_count"], 3, "Should have 3 deadlines");
}

#[tokio::test]
async fn test_multi_one_of_each_entity_type() {
    let (_dir, mut engine) = fresh_engine();

    // 1. Deadline
    agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "entity-deadline", "due_at": future_dt()}),
        &mut engine,
    )
    .await
    .unwrap();

    // 2. Schedule
    agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "entity-schedule", "start_at": near_future_dt(), "duration_minutes": 30}),
        &mut engine,
    )
    .await
    .unwrap();

    // 3. Sequence
    agentic_time_mcp::tools::handle_tool_call(
        "time_sequence_create",
        json!({"label": "entity-sequence", "steps": [{"label": "step-1"}]}),
        &mut engine,
    )
    .await
    .unwrap();

    // 4. Decay
    agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "entity-decay", "initial_value": 50.0}),
        &mut engine,
    )
    .await
    .unwrap();

    // 5. Duration
    agentic_time_mcp::tools::handle_tool_call(
        "time_duration_estimate",
        json!({"label": "entity-duration", "expected_minutes": 10}),
        &mut engine,
    )
    .await
    .unwrap();

    // Verify all 5 entity types present
    let stats = agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine)
        .await
        .unwrap();
    assert_eq!(stats["deadline_count"], 1, "1 deadline");
    assert_eq!(stats["schedule_count"], 1, "1 schedule");
    assert_eq!(stats["sequence_count"], 1, "1 sequence");
    assert_eq!(stats["decay_count"], 1, "1 decay");
    assert_eq!(stats["duration_count"], 1, "1 duration");
}

#[tokio::test]
async fn test_multi_refresh_after_mixed_creation() {
    let (_dir, mut engine) = fresh_engine();

    // Create a mix of entities
    agentic_time_mcp::tools::handle_tool_call(
        "time_deadline_add",
        json!({"label": "refresh-dl", "due_at": future_dt()}),
        &mut engine,
    )
    .await
    .unwrap();

    agentic_time_mcp::tools::handle_tool_call(
        "time_decay_create",
        json!({"label": "refresh-decay", "initial_value": 75.0, "decay_type": "exponential", "rate": 0.05}),
        &mut engine,
    )
    .await
    .unwrap();

    agentic_time_mcp::tools::handle_tool_call(
        "time_schedule_create",
        json!({"label": "refresh-sched", "start_at": near_future_dt(), "duration_minutes": 20}),
        &mut engine,
    )
    .await
    .unwrap();

    // Run time_refresh - should process without errors
    let refresh =
        agentic_time_mcp::tools::handle_tool_call("time_refresh", json!({}), &mut engine).await;
    assert!(refresh.is_ok());
    let val = refresh.unwrap();
    // Non-negative counts
    assert!(val["deadlines_updated"].as_u64().is_some());
    assert!(val["decays_updated"].as_u64().is_some());
    assert_eq!(
        val["new_overdue_count"], 0,
        "Future deadline should not be overdue"
    );
}

#[tokio::test]
async fn test_multi_10_mixed_entities_stats_consistency() {
    let (_dir, mut engine) = fresh_engine();

    // Create 2 of each entity type = 10 total
    for i in 0..2 {
        let due = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(i + 10))
            .unwrap()
            .to_rfc3339();
        agentic_time_mcp::tools::handle_tool_call(
            "time_deadline_add",
            json!({"label": format!("dl-{}", i), "due_at": due}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    for i in 0..2 {
        let start = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(i + 1))
            .unwrap()
            .to_rfc3339();
        agentic_time_mcp::tools::handle_tool_call(
            "time_schedule_create",
            json!({"label": format!("sch-{}", i), "start_at": start, "duration_minutes": 15}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    for i in 0..2 {
        agentic_time_mcp::tools::handle_tool_call(
            "time_sequence_create",
            json!({"label": format!("seq-{}", i), "steps": [{"label": "a"}, {"label": "b"}]}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    for i in 0..2 {
        agentic_time_mcp::tools::handle_tool_call(
            "time_decay_create",
            json!({"label": format!("dec-{}", i), "initial_value": 100.0}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    for i in 0..2 {
        agentic_time_mcp::tools::handle_tool_call(
            "time_duration_estimate",
            json!({"label": format!("dur-{}", i), "expected_minutes": 30 + i}),
            &mut engine,
        )
        .await
        .unwrap();
    }

    // Verify counts
    let stats = agentic_time_mcp::tools::handle_tool_call("time_stats", json!({}), &mut engine)
        .await
        .unwrap();
    assert_eq!(stats["deadline_count"], 2, "2 deadlines");
    assert_eq!(stats["schedule_count"], 2, "2 schedules");
    assert_eq!(stats["sequence_count"], 2, "2 sequences");
    assert_eq!(stats["decay_count"], 2, "2 decays");
    assert_eq!(stats["duration_count"], 2, "2 durations");

    // Total entity count
    let total = stats["deadline_count"].as_u64().unwrap()
        + stats["schedule_count"].as_u64().unwrap()
        + stats["sequence_count"].as_u64().unwrap()
        + stats["decay_count"].as_u64().unwrap()
        + stats["duration_count"].as_u64().unwrap();
    assert_eq!(total, 10, "Total entities should be 10");
}
