//! MCP server — handles JSON-RPC messages over stdio.

use std::path::PathBuf;

use serde_json::{json, Value};

use crate::ghost_bridge;
use crate::greeting;
use crate::invention_exploration;
use crate::invention_management;
use crate::invention_protection;
use crate::prompts;
use crate::stdio::{validate_jsonrpc, StdioTransport, TransportError};
use crate::tools;

#[derive(Debug, Clone)]
struct RuntimeConfig {
    auto_capture_mode: String,
    auto_capture_redact: bool,
    auto_capture_max_chars: usize,
    storage_budget_mode: String,
    storage_budget_bytes: u64,
    storage_budget_horizon_years: u32,
    storage_budget_target_fraction: f64,
}

impl RuntimeConfig {
    fn from_env() -> Self {
        Self {
            auto_capture_mode: env_with_fallback("ATIME_AUTO_CAPTURE_MODE", "AUTO_CAPTURE_MODE")
                .unwrap_or_else(|| "summary".to_string()),
            auto_capture_redact: env_with_fallback(
                "ATIME_AUTO_CAPTURE_REDACT",
                "AUTO_CAPTURE_REDACT",
            )
            .map(|v| parse_bool(&v))
            .unwrap_or(true),
            auto_capture_max_chars: env_with_fallback(
                "ATIME_AUTO_CAPTURE_MAX_CHARS",
                "AUTO_CAPTURE_MAX_CHARS",
            )
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(768),
            storage_budget_mode: env_with_fallback(
                "ATIME_STORAGE_BUDGET_MODE",
                "STORAGE_BUDGET_MODE",
            )
            .unwrap_or_else(|| "auto-rollup".to_string()),
            storage_budget_bytes: env_with_fallback(
                "ATIME_STORAGE_BUDGET_BYTES",
                "STORAGE_BUDGET_BYTES",
            )
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(536_870_912),
            storage_budget_horizon_years: env_with_fallback(
                "ATIME_STORAGE_BUDGET_HORIZON_YEARS",
                "STORAGE_BUDGET_HORIZON_YEARS",
            )
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(5),
            storage_budget_target_fraction: env_with_fallback(
                "ATIME_STORAGE_BUDGET_TARGET_FRACTION",
                "STORAGE_BUDGET_TARGET_FRACTION",
            )
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.85),
        }
    }

    fn as_json(&self) -> Value {
        json!({
            "auto_capture_mode": self.auto_capture_mode,
            "auto_capture_redact": self.auto_capture_redact,
            "auto_capture_max_chars": self.auto_capture_max_chars,
            "storage_budget_mode": self.storage_budget_mode,
            "storage_budget_bytes": self.storage_budget_bytes,
            "storage_budget_horizon_years": self.storage_budget_horizon_years,
            "storage_budget_target_fraction": self.storage_budget_target_fraction
        })
    }

    fn enforce_storage_budget(&self, atime_path: &PathBuf) -> Result<(), String> {
        if self.storage_budget_mode.eq_ignore_ascii_case("off") {
            return Ok(());
        }

        let metadata = std::fs::metadata(atime_path).map_err(|e| e.to_string())?;
        let used = metadata.len();
        let threshold =
            (self.storage_budget_bytes as f64 * self.storage_budget_target_fraction).round() as u64;
        if used > threshold {
            tracing::warn!(
                "storage budget near/exceeded: used={} threshold={} mode={} horizon_years={}",
                used,
                threshold,
                self.storage_budget_mode,
                self.storage_budget_horizon_years
            );
        }
        Ok(())
    }
}

fn env_with_fallback(primary: &str, fallback: &str) -> Option<String> {
    std::env::var(primary)
        .ok()
        .or_else(|| std::env::var(fallback).ok())
}

fn parse_bool(v: &str) -> bool {
    matches!(
        v.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn mcp_tool_surface_is_compact() -> bool {
    env_with_fallback("ATIME_MCP_TOOL_SURFACE", "MCP_TOOL_SURFACE")
        .map(|v| v.eq_ignore_ascii_case("compact"))
        .unwrap_or(false)
}

fn compact_input_schema(ops: &[String], description: &str) -> Value {
    json!({
        "type": "object",
        "required": ["operation"],
        "properties": {
            "operation": {
                "type": "string",
                "enum": ops,
                "description": description
            },
            "params": {
                "type": "object",
                "description": "Arguments for the selected operation"
            }
        }
    })
}

fn ops_from_prefix(defs: &[tools::ToolDefinition], prefix: &str) -> Vec<String> {
    defs.iter()
        .filter_map(|d| d.name.strip_prefix(prefix).map(str::to_string))
        .collect()
}

fn ops_from_time_prefix(defs: &[tools::ToolDefinition]) -> Vec<String> {
    defs.iter()
        .filter_map(|d| d.name.strip_prefix("time_").map(str::to_string))
        .collect()
}

fn compact_tool_list() -> Vec<Value> {
    vec![
        json!({
            "name": "time_deadline",
            "description": "Compact deadline facade",
            "inputSchema": compact_input_schema(&ops_from_prefix(tools::TOOLS, "time_deadline_"), "Deadline operation")
        }),
        json!({
            "name": "time_schedule",
            "description": "Compact schedule facade",
            "inputSchema": compact_input_schema(&ops_from_prefix(tools::TOOLS, "time_schedule_"), "Schedule operation")
        }),
        json!({
            "name": "time_sequence",
            "description": "Compact sequence facade",
            "inputSchema": compact_input_schema(&ops_from_prefix(tools::TOOLS, "time_sequence_"), "Sequence operation")
        }),
        json!({
            "name": "time_decay",
            "description": "Compact decay facade",
            "inputSchema": compact_input_schema(&ops_from_prefix(tools::TOOLS, "time_decay_"), "Decay operation")
        }),
        json!({
            "name": "time_duration",
            "description": "Compact duration facade",
            "inputSchema": compact_input_schema(&ops_from_prefix(tools::TOOLS, "time_duration_"), "Duration operation")
        }),
        json!({
            "name": "time_core",
            "description": "Compact core temporal facade",
            "inputSchema": compact_input_schema(
                &[
                    "stats".to_string(),
                    "refresh".to_string(),
                    "ground".to_string(),
                    "evidence".to_string(),
                    "suggest".to_string(),
                ],
                "Core temporal operation",
            )
        }),
        json!({
            "name": "time_workspace",
            "description": "Compact workspace/session facade",
            "inputSchema": compact_input_schema(
                &[
                    "workspace_create".to_string(),
                    "workspace_add".to_string(),
                    "workspace_list".to_string(),
                    "workspace_query".to_string(),
                    "workspace_compare".to_string(),
                    "workspace_xref".to_string(),
                    "session_start".to_string(),
                    "session_end".to_string(),
                    "session_resume".to_string(),
                ],
                "Workspace/session operation",
            )
        }),
        json!({
            "name": "time_exploration",
            "description": "Compact exploration inventions facade",
            "inputSchema": compact_input_schema(&ops_from_time_prefix(invention_exploration::TOOL_DEFS), "Exploration operation")
        }),
        json!({
            "name": "time_protection",
            "description": "Compact protection inventions facade",
            "inputSchema": compact_input_schema(&ops_from_time_prefix(invention_protection::TOOL_DEFS), "Protection operation")
        }),
        json!({
            "name": "time_management",
            "description": "Compact management inventions facade",
            "inputSchema": compact_input_schema(&ops_from_time_prefix(invention_management::TOOL_DEFS), "Management operation")
        }),
    ]
}

fn decode_compact_operation(args: Value) -> Result<(String, Value), String> {
    let obj = args
        .as_object()
        .ok_or_else(|| "arguments must be an object".to_string())?;
    let operation = obj
        .get("operation")
        .and_then(Value::as_str)
        .ok_or_else(|| "'operation' is required".to_string())?
        .to_string();

    if let Some(params) = obj.get("params") {
        return Ok((operation, params.clone()));
    }

    let mut passthrough = obj.clone();
    passthrough.remove("operation");
    Ok((operation, Value::Object(passthrough)))
}

fn tool_in_defs(defs: &[tools::ToolDefinition], tool_name: &str) -> bool {
    defs.iter().any(|d| d.name == tool_name)
}

fn resolve_compact_tool(group: &str, operation: &str) -> Option<String> {
    let with_prefix = |prefix: &str| {
        if operation.starts_with(prefix) {
            operation.to_string()
        } else {
            format!("{prefix}{operation}")
        }
    };

    let candidate = match group {
        "time_deadline" => with_prefix("time_deadline_"),
        "time_schedule" => with_prefix("time_schedule_"),
        "time_sequence" => with_prefix("time_sequence_"),
        "time_decay" => with_prefix("time_decay_"),
        "time_duration" => with_prefix("time_duration_"),
        "time_core" => with_prefix("time_"),
        "time_workspace" => match operation {
            "workspace_create" => "time_workspace_create".to_string(),
            "workspace_add" => "time_workspace_add".to_string(),
            "workspace_list" => "time_workspace_list".to_string(),
            "workspace_query" => "time_workspace_query".to_string(),
            "workspace_compare" => "time_workspace_compare".to_string(),
            "workspace_xref" => "time_workspace_xref".to_string(),
            "session_start" => "session_start".to_string(),
            "session_end" => "session_end".to_string(),
            "session_resume" => "time_session_resume".to_string(),
            _ => return None,
        },
        "time_exploration" | "time_protection" | "time_management" => {
            if operation.starts_with("time_") {
                operation.to_string()
            } else {
                format!("time_{operation}")
            }
        }
        _ => return None,
    };

    let valid = match group {
        "time_deadline" | "time_schedule" | "time_sequence" | "time_decay" | "time_duration"
        | "time_core" | "time_workspace" => tool_in_defs(tools::TOOLS, &candidate),
        "time_exploration" => tool_in_defs(invention_exploration::TOOL_DEFS, &candidate),
        "time_protection" => tool_in_defs(invention_protection::TOOL_DEFS, &candidate),
        "time_management" => tool_in_defs(invention_management::TOOL_DEFS, &candidate),
        _ => false,
    };

    if valid {
        Some(candidate)
    } else {
        None
    }
}

fn normalize_compact_tool_call(tool_name: &str, args: Value) -> Result<(String, Value), String> {
    if !matches!(
        tool_name,
        "time_deadline"
            | "time_schedule"
            | "time_sequence"
            | "time_decay"
            | "time_duration"
            | "time_core"
            | "time_workspace"
            | "time_exploration"
            | "time_protection"
            | "time_management"
    ) {
        return Ok((tool_name.to_string(), args));
    }

    let (operation, params) = decode_compact_operation(args)?;
    let resolved = resolve_compact_tool(tool_name, &operation)
        .ok_or_else(|| format!("Unknown {tool_name} operation: {operation}"))?;
    Ok((resolved, params))
}

/// Check AGENTIC_TOKEN for server-mode authentication.
fn check_server_auth() -> Result<(), Box<dyn std::error::Error>> {
    let is_server_mode = std::env::var("AGENTRA_RUNTIME_MODE")
        .map(|v| v == "server")
        .unwrap_or(false)
        || std::env::var("AGENTRA_SERVER")
            .map(|v| v == "1")
            .unwrap_or(false);

    if is_server_mode {
        let has_token =
            std::env::var("AGENTIC_TOKEN").is_ok() || std::env::var("AGENTIC_TOKEN_FILE").is_ok();
        if !has_token {
            return Err("Server mode requires AGENTIC_TOKEN or AGENTIC_TOKEN_FILE".into());
        }
    }
    Ok(())
}

/// Run the MCP server on stdio.
pub fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    check_server_auth()?;
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    let mut transport = StdioTransport::new(stdin, stdout);

    // Determine .atime file path
    let atime_path = std::env::var("ATIME_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs_home().join(".agentic").join("time.atime"));

    // Ensure parent dir exists
    if let Some(parent) = atime_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let runtime = RuntimeConfig::from_env();

    // Print startup greeting to stderr (MCP uses stdout for JSON-RPC)
    greeting::print_greeting();

    // Open or create the file
    let file = if atime_path.exists() {
        agentic_time::TimeFile::open(&atime_path)?
    } else {
        agentic_time::TimeFile::create(&atime_path)?
    };

    let mut engine = agentic_time::WriteEngine::new(file);

    // Spawn Ghost Writer (5-second sync to IDE memory dirs)
    let _ghost_handle = ghost_bridge::spawn_ghost_writer();

    tracing::info!("AgenticTime MCP server started");

    loop {
        if let Err(e) = runtime.enforce_storage_budget(&atime_path) {
            tracing::warn!("storage budget check failed: {}", e);
        }

        let msg = match transport.read_message() {
            Ok(m) => m,
            Err(TransportError::Io(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                tracing::info!("Client disconnected, saving before exit");
                if let Err(e) = engine.file().save() {
                    tracing::error!("Failed to save on disconnect: {}", e);
                }
                break;
            }
            Err(e) => {
                tracing::error!("Transport error: {}", e);
                let err_response = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": e.to_string() }
                });
                let _ = transport.write_message(&err_response.to_string());
                continue;
            }
        };

        let request: Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(e) => {
                let err_response = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) }
                });
                let _ = transport.write_message(&err_response.to_string());
                continue;
            }
        };

        if let Err(e) = validate_jsonrpc(&request) {
            let err_response = json!({
                "jsonrpc": "2.0",
                "id": request.get("id").cloned().unwrap_or(Value::Null),
                "error": { "code": -32600, "message": e.to_string() }
            });
            let _ = transport.write_message(&err_response.to_string());
            continue;
        }

        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let response = match method {
            "initialize" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": { "listChanged": false },
                            "resources": { "subscribe": false, "listChanged": false },
                            "prompts": { "listChanged": false }
                        },
                        "serverInfo": {
                            "name": "agentic-time",
                            "version": env!("CARGO_PKG_VERSION")
                        },
                        "runtimeConfig": runtime.as_json()
                    }
                })
            }
            "notifications/initialized" => continue, // No response needed
            "tools/list" => {
                let tool_list: Vec<Value> = if mcp_tool_surface_is_compact() {
                    compact_tool_list()
                } else {
                    tools::TOOLS
                        .iter()
                        .chain(invention_exploration::TOOL_DEFS.iter())
                        .chain(invention_protection::TOOL_DEFS.iter())
                        .chain(invention_management::TOOL_DEFS.iter())
                        .map(|t| {
                            json!({
                                "name": t.name,
                                "description": t.description,
                                "inputSchema": serde_json::from_str::<Value>(t.input_schema).unwrap_or(json!({}))
                            })
                        })
                        .collect()
                };

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "tools": tool_list }
                })
            }
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(json!({}));
                let requested_tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let raw_args = params.get("arguments").cloned().unwrap_or(json!({}));
                let (tool_name, args) =
                    match normalize_compact_tool_call(requested_tool_name, raw_args) {
                        Ok(mapped) => mapped,
                        Err(e) => {
                            let err = format!("Error: {}", e);
                            let response = json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "content": [{
                                        "type": "text",
                                        "text": err
                                    }],
                                    "isError": true
                                }
                            });
                            transport.write_message(&response.to_string())?;
                            continue;
                        }
                    };

                // Try invention modules first, then fall back to core tools
                let result = if let Some(r) =
                    invention_exploration::try_handle(&tool_name, args.clone(), &mut engine)
                {
                    r
                } else if let Some(r) =
                    invention_protection::try_handle(&tool_name, args.clone(), &mut engine)
                {
                    r
                } else if let Some(r) =
                    invention_management::try_handle(&tool_name, args.clone(), &mut engine)
                {
                    r
                } else {
                    tools::handle_tool_call(&tool_name, args, &mut engine)
                };

                match result {
                    Ok(result) => {
                        // Ensure state is persisted after every successful tool call
                        if let Err(save_err) = engine.file().save() {
                            tracing::error!("Failed to save after tool call: {}", save_err);
                        }
                        json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                                }]
                            }
                        })
                    }
                    Err(e) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{
                                "type": "text",
                                "text": format!("Error: {}", e)
                            }],
                            "isError": true
                        }
                    }),
                }
            }
            "resources/list" => {
                let resources: Vec<Value> = crate::resources::list_resources()
                    .iter()
                    .map(|r| {
                        json!({
                            "uri": r.uri,
                            "name": r.name,
                            "description": r.description,
                            "mimeType": r.mime_type
                        })
                    })
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "resources": resources }
                })
            }
            "prompts/list" => {
                let prompt_list: Vec<Value> = crate::prompts::PROMPTS
                    .iter()
                    .map(|p| {
                        let args: Vec<Value> = p
                            .arguments
                            .iter()
                            .map(|a| {
                                json!({
                                    "name": a.name,
                                    "description": a.description,
                                    "required": a.required
                                })
                            })
                            .collect();
                        json!({
                            "name": p.name,
                            "description": p.description,
                            "arguments": args
                        })
                    })
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "prompts": prompt_list }
                })
            }
            "prompts/get" => {
                let params = request.get("params").cloned().unwrap_or(json!({}));
                let prompt_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let arguments = params
                    .get("arguments")
                    .and_then(|a| {
                        serde_json::from_value::<std::collections::HashMap<String, String>>(
                            a.clone(),
                        )
                        .ok()
                    })
                    .unwrap_or_default();

                match prompts::expand_prompt(prompt_name, &arguments) {
                    Some(content) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "messages": [{
                                "role": "user",
                                "content": { "type": "text", "text": content }
                            }]
                        }
                    }),
                    None => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32602, "message": format!("Prompt not found: {}", prompt_name) }
                    }),
                }
            }
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": format!("Method not found: {}", method) }
                })
            }
        };

        transport.write_message(&response.to_string())?;
    }

    Ok(())
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn compact_tool_list_has_expected_surface() {
        let defs = compact_tool_list();
        let names: Vec<&str> = defs
            .iter()
            .filter_map(|d| d.get("name").and_then(|n| n.as_str()))
            .collect();
        assert_eq!(defs.len(), 10);
        assert!(names.contains(&"time_deadline"));
        assert!(names.contains(&"time_schedule"));
        assert!(names.contains(&"time_sequence"));
        assert!(names.contains(&"time_decay"));
        assert!(names.contains(&"time_duration"));
        assert!(names.contains(&"time_core"));
        assert!(names.contains(&"time_workspace"));
        assert!(names.contains(&"time_exploration"));
        assert!(names.contains(&"time_protection"));
        assert!(names.contains(&"time_management"));
    }

    #[test]
    fn compact_deadline_operation_routes_to_legacy_name() {
        let (name, args) = normalize_compact_tool_call(
            "time_deadline",
            json!({
                "operation": "add",
                "params": {
                    "label": "Ship v1",
                    "due_at": "2030-01-01T00:00:00Z"
                }
            }),
        )
        .expect("deadline compact should map");
        assert_eq!(name, "time_deadline_add");
        assert_eq!(args["label"], "Ship v1");
    }

    #[test]
    fn compact_workspace_operation_routes_to_legacy_name() {
        let (name, args) = normalize_compact_tool_call(
            "time_workspace",
            json!({
                "operation": "session_resume",
                "params": { "limit": 3 }
            }),
        )
        .expect("workspace compact should map");
        assert_eq!(name, "time_session_resume");
        assert_eq!(args["limit"], 3);
    }

    #[test]
    fn compact_unknown_operation_errors() {
        let err = normalize_compact_tool_call(
            "time_core",
            json!({
                "operation": "does_not_exist"
            }),
        )
        .expect_err("unknown operation should fail");
        assert!(err.contains("Unknown time_core operation"));
    }
}
