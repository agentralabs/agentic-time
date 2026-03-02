//! Schedule CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ScheduleArgs {
    #[command(subcommand)]
    pub command: ScheduleCommand,
}

#[derive(Subcommand)]
pub enum ScheduleCommand {
    /// Create a scheduled event
    Create {
        /// Event label
        label: String,
        /// Start time (ISO 8601)
        #[arg(long)]
        start: String,
        /// Duration in minutes
        #[arg(long)]
        duration: i64,
    },
    /// List today's schedule
    Today,
}

pub fn run(args: ScheduleArgs, atime_path: &str) {
    let file = crate::open_or_create(atime_path);
    let mut engine = agentic_time::WriteEngine::new(file);

    match args.command {
        ScheduleCommand::Create {
            label,
            start,
            duration,
        } => match chrono::DateTime::parse_from_rfc3339(&start) {
            Ok(dt) => {
                let schedule = agentic_time::Schedule::new(
                    &label,
                    dt.with_timezone(&chrono::Utc),
                    duration * 60,
                );
                match engine.add_schedule(schedule) {
                    Ok(id) => println!("Created schedule: {}", id),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => crate::fail(&format!("Invalid date format: {}", e)),
        },
        ScheduleCommand::Today => {
            let now = chrono::Utc::now();
            let end = now + chrono::Duration::hours(24);
            let query = agentic_time::QueryEngine::new(engine.file());
            match query.schedules_in_range(now, end) {
                Ok(schedules) => {
                    for s in &schedules {
                        println!("  {} — {} ({})", s.id, s.label, s.start_at);
                    }
                    if schedules.is_empty() {
                        println!("Nothing scheduled today.");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
