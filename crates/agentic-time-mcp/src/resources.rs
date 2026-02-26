//! MCP resource definitions for AgenticTime.

/// Number of resource definitions.
pub const RESOURCE_COUNT: usize = 17;

/// A resource definition.
pub struct ResourceDefinition {
    /// Resource URI.
    pub uri: &'static str,
    /// Resource name.
    pub name: &'static str,
    /// Resource description.
    pub description: &'static str,
    /// MIME type.
    pub mime_type: &'static str,
}

/// All AgenticTime MCP resources.
pub const RESOURCES: &[ResourceDefinition] = &[
    ResourceDefinition {
        uri: "atime://deadline/",
        name: "All Deadlines",
        description: "List of all deadlines",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://deadline/overdue",
        name: "Overdue Deadlines",
        description: "List of overdue deadlines",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://deadline/upcoming",
        name: "Upcoming Deadlines",
        description: "Deadlines due in next 7 days",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://schedule/",
        name: "All Schedules",
        description: "List of all scheduled events",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://schedule/today",
        name: "Today's Schedule",
        description: "Schedule for today",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://schedule/week",
        name: "This Week's Schedule",
        description: "Schedule for this week",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://sequence/",
        name: "All Sequences",
        description: "List of all sequences",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://sequence/active",
        name: "Active Sequences",
        description: "Sequences currently in progress",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://decay/",
        name: "All Decay Models",
        description: "List of all decay models",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://decay/critical",
        name: "Critical Decays",
        description: "Decay models below 20% value",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://duration/",
        name: "All Estimates",
        description: "List all duration estimates",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "atime://stats",
        name: "Temporal Statistics",
        description: "Overall temporal statistics",
        mime_type: "application/json",
    },
];

/// List all resources.
pub fn list_resources() -> &'static [ResourceDefinition] {
    RESOURCES
}
