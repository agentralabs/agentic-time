//! Decay CLI commands.

use clap::{Args, Subcommand};

fn fail(msg: &str) -> ! {
    eprintln!("{msg}");
    std::process::exit(1);
}

#[derive(Args)]
pub struct DecayArgs {
    #[command(subcommand)]
    pub command: DecayCommand,
}

#[derive(Subcommand)]
pub enum DecayCommand {
    /// Create a decay model
    Create {
        /// Label
        label: String,
        /// Initial value
        #[arg(long)]
        initial: f64,
        /// Decay type
        #[arg(long, default_value = "exponential")]
        r#type: String,
        /// Rate
        #[arg(long, default_value = "0.01")]
        rate: f64,
    },
    /// Get current value
    Value {
        /// Decay model ID
        id: String,
    },
    /// List all decay models
    List,
}

pub fn run(args: DecayArgs, atime_path: &str) {
    let file = crate::open_or_create(atime_path);
    let mut engine = agentic_time::WriteEngine::new(file);

    match args.command {
        DecayCommand::Create {
            label,
            initial,
            r#type,
            rate,
        } => {
            let decay_type = match r#type.as_str() {
                "linear" => agentic_time::DecayType::Linear { rate },
                "exponential" => agentic_time::DecayType::Exponential { lambda: rate },
                "half_life" => agentic_time::DecayType::HalfLife {
                    half_life_secs: (rate * 3600.0) as i64,
                },
                _ => {
                    fail(&format!(
                        "Unknown decay type: {}. Use linear, exponential, or half_life",
                        r#type
                    ));
                }
            };
            let decay = agentic_time::DecayModel::new(&label, initial, decay_type);
            match engine.add_decay(decay) {
                Ok(id) => println!("Created decay model: {}", id),
                Err(e) => fail(&format!("Error: {}", e)),
            }
        }
        DecayCommand::Value { id } => match id.parse::<agentic_time::TemporalId>() {
            Ok(tid) => match engine.refresh_decay(&tid) {
                Ok(val) => println!("Current value: {:.4}", val),
                Err(e) => fail(&format!("Error: {}", e)),
            },
            Err(e) => fail(&format!("Invalid ID: {}", e)),
        },
        DecayCommand::List => {
            let query = agentic_time::QueryEngine::new(engine.file());
            match query.decays_below_threshold(f64::MAX) {
                Ok(decays) => {
                    for d in &decays {
                        println!("  {} — {} (value: {:.4})", d.id, d.label, d.current_value);
                    }
                    if decays.is_empty() {
                        println!("No decay models.");
                    }
                }
                Err(e) => fail(&format!("Error: {}", e)),
            }
        }
    }
}
