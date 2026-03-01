//! Ghost Writer — auto-syncs temporal context to AI coding tools.
//!
//! Writes formatted markdown to:
//! - Claude: `~/.claude/memory/TIME_CONTEXT.md`
//! - Cursor: `~/.cursor/memory/agentic-time.md`
//! - Windsurf: `~/.windsurf/memory/agentic-time.md`
//! - Cody: `~/.sourcegraph/cody/memory/agentic-time.md`

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// Client targets for ghost writing.
const CLIENTS: &[(&str, &str, &str)] = &[
    ("Claude", ".claude/memory", "TIME_CONTEXT.md"),
    ("Cursor", ".cursor/memory", "agentic-time.md"),
    ("Windsurf", ".windsurf/memory", "agentic-time.md"),
    ("Cody", ".sourcegraph/cody/memory", "agentic-time.md"),
];

/// Detect all available AI coding tool memory directories.
pub fn detect_all_memory_dirs() -> Vec<(String, PathBuf)> {
    let home = match std::env::var("HOME") {
        Ok(h) => PathBuf::from(h),
        Err(_) => return Vec::new(),
    };

    let mut dirs = Vec::new();
    for (name, rel_path, filename) in CLIENTS {
        let dir = home.join(rel_path);
        if dir.exists() || std::fs::create_dir_all(&dir).is_ok() {
            dirs.push((name.to_string(), dir.join(filename)));
        }
    }
    dirs
}

/// Format temporal context as markdown for ghost writing.
pub fn format_time_context(
    overdue_deadlines: &[String],
    today_schedule: &[String],
    active_sequences: &[String],
    critical_decays: &[String],
) -> String {
    let mut out = String::new();
    out.push_str("# AgenticTime Context\n\n");
    out.push_str(&format!(
        "_Last updated: {}_\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    if !overdue_deadlines.is_empty() {
        out.push_str("## ⚠️ Overdue Deadlines\n\n");
        for d in overdue_deadlines {
            out.push_str(&format!("- {}\n", d));
        }
        out.push('\n');
    }

    if !today_schedule.is_empty() {
        out.push_str("## 📅 Today's Schedule\n\n");
        for s in today_schedule {
            out.push_str(&format!("- {}\n", s));
        }
        out.push('\n');
    }

    if !active_sequences.is_empty() {
        out.push_str("## 🔄 Active Sequences\n\n");
        for s in active_sequences {
            out.push_str(&format!("- {}\n", s));
        }
        out.push('\n');
    }

    if !critical_decays.is_empty() {
        out.push_str("## 📉 Critical Decays\n\n");
        for d in critical_decays {
            out.push_str(&format!("- {}\n", d));
        }
        out.push('\n');
    }

    if overdue_deadlines.is_empty()
        && today_schedule.is_empty()
        && active_sequences.is_empty()
        && critical_decays.is_empty()
    {
        out.push_str("_No active temporal items._\n");
    }

    out
}

/// Write context to all detected client directories (atomic write).
fn write_to_clients(content: &str, targets: &[(String, PathBuf)]) {
    for (name, path) in targets {
        let tmp_path = path.with_extension("md.tmp");
        match std::fs::write(&tmp_path, content) {
            Ok(_) => match std::fs::rename(&tmp_path, path) {
                Ok(_) => tracing::trace!("Ghost writer: updated {}", name),
                Err(e) => tracing::warn!("Ghost writer: rename failed for {}: {}", name, e),
            },
            Err(e) => tracing::warn!("Ghost writer: write failed for {}: {}", name, e),
        }
    }
}

/// Spawn the ghost writer background thread (5-second sync interval).
pub fn spawn_ghost_writer() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let targets = detect_all_memory_dirs();
        if targets.is_empty() {
            tracing::info!("Ghost writer: no AI tool directories detected, stopping");
            return;
        }

        tracing::info!(
            "Ghost writer: syncing to {} clients every 5s",
            targets.len()
        );

        loop {
            thread::sleep(Duration::from_secs(5));

            // TODO: Read actual data from engine when wired up
            let content = format_time_context(&[], &[], &[], &[]);
            write_to_clients(&content, &targets);
        }
    })
}
