//! Sequence CLI commands.

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct SequenceArgs {
    #[command(subcommand)]
    pub command: SequenceCommand,
}

#[derive(Subcommand)]
pub enum SequenceCommand {
    /// Show status of a sequence
    Status {
        /// Sequence ID
        id: String,
    },
    /// Advance a sequence
    Advance {
        /// Sequence ID
        id: String,
    },
    /// List sequences
    List,
}

pub fn run(args: SequenceArgs, atime_path: &str) {
    let file = crate::open_or_create(atime_path);
    let mut engine = agentic_time::WriteEngine::new(file);

    match args.command {
        SequenceCommand::Status { id } => match id.parse::<agentic_time::TemporalId>() {
            Ok(tid) => {
                let query = agentic_time::QueryEngine::new(engine.file());
                match query.sequence(&tid) {
                    Ok(Some(seq)) => {
                        println!("Sequence: {} ({:?})", seq.label, seq.status);
                        println!("Step {}/{}", seq.current_step + 1, seq.steps.len());
                        for step in &seq.steps {
                            println!("  {} — {:?}", step.label, step.status);
                        }
                    }
                    Ok(None) => eprintln!("Sequence not found: {}", id),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => eprintln!("Invalid ID: {}", e),
        },
        SequenceCommand::Advance { id } => match id.parse::<agentic_time::TemporalId>() {
            Ok(tid) => match engine.advance_sequence(&tid) {
                Ok(_) => println!("Sequence advanced"),
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Invalid ID: {}", e),
        },
        SequenceCommand::List => {
            let query = agentic_time::QueryEngine::new(engine.file());
            match query.active_sequences() {
                Ok(seqs) => {
                    for s in &seqs {
                        println!(
                            "  {} — {} (step {}/{})",
                            s.id,
                            s.label,
                            s.current_step + 1,
                            s.steps.len()
                        );
                    }
                    if seqs.is_empty() {
                        println!("No active sequences.");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
