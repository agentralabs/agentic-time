//! AgenticTime CLI — `atime` command-line interface.

mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "atime", version, about = "Temporal reasoning for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the .atime file
    #[arg(long, global = true, env = "ATIME_PATH")]
    path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage deadlines
    Deadline(commands::deadline::DeadlineArgs),
    /// Manage schedules
    Schedule(commands::schedule::ScheduleArgs),
    /// Manage sequences
    Sequence(commands::sequence::SequenceArgs),
    /// Manage decay models
    Decay(commands::decay::DecayArgs),
    /// Show temporal statistics
    Stats,
    /// Refresh all statuses
    Refresh,
    /// Show file info
    Info {
        /// File path
        path: Option<String>,
    },
    /// Install MCP configuration
    Install {
        /// Install profile
        #[arg(long, default_value = "desktop")]
        profile: String,
    },
    /// Start MCP server
    Serve,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let atime_path = cli.path.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.agentic/time.atime", home)
    });

    match cli.command {
        Commands::Stats => {
            let file = open_or_create(&atime_path);
            let engine = agentic_time::WriteEngine::new(file);
            let query = agentic_time::QueryEngine::new(engine.file());
            match query.stats() {
                Ok(stats) => println!("{}", serde_json::to_string_pretty(&stats).unwrap()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Refresh => {
            let file = open_or_create(&atime_path);
            let mut engine = agentic_time::WriteEngine::new(file);
            match engine.update_all_statuses() {
                Ok(report) => {
                    println!(
                        "Refreshed: {} deadlines updated, {} decays updated, {} new overdue",
                        report.deadlines_updated,
                        report.decays_updated,
                        report.new_overdue.len()
                    );
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Info { path } => {
            let p = path.as_deref().unwrap_or(&atime_path);
            match agentic_time::TimeFile::open(p) {
                Ok(f) => {
                    println!("File: {}", p);
                    println!("Entities: {}", f.entity_count());
                    println!("Version: {}", f.header.version);
                }
                Err(e) => eprintln!("Error opening {}: {}", p, e),
            }
        }
        Commands::Install { profile } => {
            println!("Installing AgenticTime for profile: {}", profile);
            println!("TODO: implement install command");
        }
        Commands::Serve => {
            println!("Starting MCP server... Use agentic-time-mcp serve instead.");
        }
        Commands::Deadline(args) => commands::deadline::run(args, &atime_path),
        Commands::Schedule(args) => commands::schedule::run(args, &atime_path),
        Commands::Sequence(args) => commands::sequence::run(args, &atime_path),
        Commands::Decay(args) => commands::decay::run(args, &atime_path),
    }
}

fn open_or_create(path: &str) -> agentic_time::TimeFile {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::path::Path::new(path).exists() {
        agentic_time::TimeFile::open(path).expect("Failed to open .atime file")
    } else {
        agentic_time::TimeFile::create(path).expect("Failed to create .atime file")
    }
}
