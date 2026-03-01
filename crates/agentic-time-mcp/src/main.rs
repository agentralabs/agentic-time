//! AgenticTime MCP Server — temporal reasoning for AI agents.

mod ghost_bridge;
mod greeting;
mod invention_exploration;
mod invention_management;
mod invention_protection;
mod prompts;
mod resources;
mod server;
mod stdio;
mod tools;

use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("serve");

    match command {
        "serve" | "--stdio" => {
            if let Err(e) = server::run_server() {
                tracing::error!("Server error: {}", e);
                std::process::exit(1);
            }
        }
        "info" => {
            println!("AgenticTime MCP Server v{}", env!("CARGO_PKG_VERSION"));
            println!("Tool count: {}", tools::TOOLS.len());
            println!("Resource count: {}", resources::RESOURCE_COUNT);
            println!("Prompt count: {}", prompts::PROMPT_COUNT);
        }
        _ => {
            eprintln!("Usage: agentic-time-mcp [serve|info]");
            std::process::exit(1);
        }
    }
}
