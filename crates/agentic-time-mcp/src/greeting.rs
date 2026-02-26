//! Server greeting on startup.

/// Print the startup greeting to stderr.
pub fn print_greeting() {
    eprintln!("╔═══════════════════════════════════════════╗");
    eprintln!(
        "║  AgenticTime MCP Server v{}        ║",
        env!("CARGO_PKG_VERSION")
    );
    eprintln!("║  Temporal reasoning for AI agents         ║");
    eprintln!("║  🕐 Deadlines · Schedules · Sequences     ║");
    eprintln!("╚═══════════════════════════════════════════╝");
}
