//! MCP server — handles JSON-RPC messages over stdio.

use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;

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
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
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
    let ghost_file = if atime_path.exists() {
        agentic_time::TimeFile::open(&atime_path)?
    } else {
        agentic_time::TimeFile::create(&atime_path)?
    };
    let ghost_engine = Arc::new(Mutex::new(agentic_time::WriteEngine::new(ghost_file)));
    let _ghost_handle = ghost_bridge::spawn_ghost_writer(ghost_engine);

    tracing::info!("AgenticTime MCP server started");

    loop {
        if let Err(e) = runtime.enforce_storage_budget(&atime_path) {
            tracing::warn!("storage budget check failed: {}", e);
        }

        let msg = match transport.read_message() {
            Ok(m) => m,
            Err(TransportError::Io(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                tracing::info!("Client disconnected");
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
                let tool_list: Vec<Value> = tools::TOOLS
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
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "tools": tool_list }
                })
            }
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(json!({}));
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                // Try invention modules first, then fall back to core tools
                let result = if let Some(r) =
                    invention_exploration::try_handle(tool_name, args.clone(), &mut engine)
                {
                    r
                } else if let Some(r) =
                    invention_protection::try_handle(tool_name, args.clone(), &mut engine)
                {
                    r
                } else if let Some(r) =
                    invention_management::try_handle(tool_name, args.clone(), &mut engine)
                {
                    r
                } else {
                    tools::handle_tool_call(tool_name, args, &mut engine).await
                };

                match result {
                    Ok(result) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{
                                "type": "text",
                                "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                            }]
                        }
                    }),
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
