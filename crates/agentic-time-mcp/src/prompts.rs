//! MCP prompt definitions for AgenticTime.

use std::collections::HashMap;

/// Number of prompts.
pub const PROMPT_COUNT: usize = 4;

/// A prompt definition.
pub struct PromptDefinition {
    /// Prompt name.
    pub name: &'static str,
    /// Prompt description.
    pub description: &'static str,
    /// Prompt arguments.
    pub arguments: &'static [PromptArgument],
}

/// A prompt argument.
pub struct PromptArgument {
    /// Argument name.
    pub name: &'static str,
    /// Argument description.
    pub description: &'static str,
    /// Whether this argument is required.
    pub required: bool,
}

/// All AgenticTime MCP prompts.
pub const PROMPTS: &[PromptDefinition] = &[
    PromptDefinition {
        name: "time_plan",
        description: "Plan a series of tasks with deadlines and dependencies",
        arguments: &[
            PromptArgument {
                name: "goal",
                description: "What you want to accomplish",
                required: true,
            },
            PromptArgument {
                name: "deadline",
                description: "Overall deadline (ISO 8601 date-time)",
                required: false,
            },
        ],
    },
    PromptDefinition {
        name: "time_review",
        description: "Review current temporal state: deadlines, schedules, decays",
        arguments: &[],
    },
    PromptDefinition {
        name: "time_estimate",
        description: "Estimate how long something will take using PERT model",
        arguments: &[PromptArgument {
            name: "task",
            description: "What you want to estimate",
            required: true,
        }],
    },
    PromptDefinition {
        name: "time_schedule_day",
        description: "Create a schedule for the day based on tasks and priorities",
        arguments: &[PromptArgument {
            name: "tasks",
            description: "Comma-separated list of tasks to schedule",
            required: true,
        }],
    },
];

/// Expand a prompt with arguments.
pub fn expand_prompt(name: &str, args: &HashMap<String, String>) -> Option<String> {
    match name {
        "time_plan" => {
            let goal = args.get("goal")?;
            let deadline_text = args.get("deadline").map(|d| format!(" with a deadline of {}", d)).unwrap_or_default();
            Some(format!(
                "I want to plan: {}{}\n\nPlease:\n1. Break this into concrete steps\n2. Estimate duration for each step (use time_duration_estimate)\n3. Identify dependencies between steps\n4. Create a sequence (use time_sequence_create)\n5. Set deadlines for critical milestones (use time_deadline_add)\n6. Check for conflicts with existing schedule",
                goal, deadline_text
            ))
        }
        "time_review" => Some(
            "Please review my current temporal state:\n\n1. Check for overdue deadlines (use time_deadline_overdue)\n2. Show today's schedule (read atime://schedule/today)\n3. List active sequences (read atime://sequence/active)\n4. Check for critical decays (read atime://decay/critical)\n5. Show overall stats (use time_stats)\n6. Highlight anything that needs immediate attention".to_string()
        ),
        "time_estimate" => {
            let task = args.get("task")?;
            Some(format!(
                "I need to estimate how long this will take: {}\n\nPlease:\n1. Consider optimistic, expected, and pessimistic scenarios\n2. Factor in potential blockers or dependencies\n3. Create the estimate (use time_duration_estimate)\n4. If this has subtasks, estimate each and aggregate (use time_duration_aggregate)",
                task
            ))
        }
        "time_schedule_day" => {
            let tasks = args.get("tasks")?;
            Some(format!(
                "I need to schedule these tasks today: {}\n\nPlease:\n1. Check what's already scheduled today (read atime://schedule/today)\n2. Find available slots (use time_schedule_available)\n3. Prioritize tasks by urgency and importance\n4. Account for context switching (add 15-min buffer between tasks)\n5. Leave buffer time for unexpected items\n6. Create the schedule (use time_schedule_create for each task)",
                tasks
            ))
        }
        _ => None,
    }
}
