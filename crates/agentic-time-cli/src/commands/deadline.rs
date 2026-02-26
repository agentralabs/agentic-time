//! Deadline CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct DeadlineArgs {
    #[command(subcommand)]
    pub command: DeadlineCommand,
}

#[derive(Subcommand)]
pub enum DeadlineCommand {
    /// Add a new deadline
    Add {
        /// Deadline label
        label: String,
        /// Due date (ISO 8601)
        #[arg(long)]
        due: String,
        /// Warning date (ISO 8601)
        #[arg(long)]
        warn: Option<String>,
        /// Deadline type
        #[arg(long, default_value = "soft")]
        r#type: String,
    },
    /// Complete a deadline
    Complete {
        /// Deadline ID
        id: String,
    },
    /// Cancel a deadline
    Cancel {
        /// Deadline ID
        id: String,
    },
    /// List deadlines
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },
    /// Show overdue deadlines
    Overdue,
}

pub fn run(args: DeadlineArgs, atime_path: &str) {
    let file = crate::open_or_create(atime_path);
    let mut engine = agentic_time::WriteEngine::new(file);

    match args.command {
        DeadlineCommand::Add { label, due, .. } => {
            match chrono::DateTime::parse_from_rfc3339(&due) {
                Ok(dt) => {
                    let deadline =
                        agentic_time::Deadline::new(&label, dt.with_timezone(&chrono::Utc));
                    match engine.add_deadline(deadline) {
                        Ok(id) => println!("Created deadline: {}", id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Err(e) => eprintln!(
                    "Invalid date format: {}. Use ISO 8601 (e.g. 2026-03-01T00:00:00Z)",
                    e
                ),
            }
        }
        DeadlineCommand::Complete { id } => match id.parse::<agentic_time::TemporalId>() {
            Ok(tid) => match engine.complete_deadline(&tid) {
                Ok(_) => println!("Deadline completed"),
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Invalid ID: {}", e),
        },
        DeadlineCommand::Cancel { id } => match id.parse::<agentic_time::TemporalId>() {
            Ok(tid) => match engine.cancel_deadline(&tid) {
                Ok(_) => println!("Deadline cancelled"),
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Invalid ID: {}", e),
        },
        DeadlineCommand::List { .. } => {
            let query = agentic_time::QueryEngine::new(engine.file());
            match query.deadlines_by_status(agentic_time::DeadlineStatus::Pending) {
                Ok(deadlines) => {
                    for d in &deadlines {
                        println!("  {} — {} (due: {})", d.id, d.label, d.due_at);
                    }
                    if deadlines.is_empty() {
                        println!("No pending deadlines.");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        DeadlineCommand::Overdue => {
            let query = agentic_time::QueryEngine::new(engine.file());
            match query.overdue_deadlines() {
                Ok(deadlines) => {
                    for d in &deadlines {
                        println!("  ⚠ {} — {} (was due: {})", d.id, d.label, d.due_at);
                    }
                    if deadlines.is_empty() {
                        println!("No overdue deadlines.");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
