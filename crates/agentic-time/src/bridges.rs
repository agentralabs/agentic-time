//! Sister integration bridge traits for AgenticTime.
//!
//! Each bridge defines the interface for integrating with another Agentra sister.
//! Default implementations are no-ops, allowing gradual adoption.
//! Trait-based design ensures Hydra compatibility — swap implementors without refactoring.

/// Bridge to agentic-memory for persisting temporal events in memory.
pub trait MemoryBridge: Send + Sync {
    /// Store a temporal event as a memory node
    fn store_temporal_event(&self, event_type: &str, details: &str) -> Result<u64, String> {
        let _ = (event_type, details);
        Err("Memory bridge not connected".to_string())
    }

    /// Recall schedule history from memory
    fn recall_schedule_history(&self, topic: &str, max_results: usize) -> Vec<String> {
        let _ = (topic, max_results);
        Vec::new()
    }

    /// Link a deadline to a memory decision node
    fn link_deadline_to_decision(&self, deadline_id: &str, node_id: u64) -> Result<(), String> {
        let _ = (deadline_id, node_id);
        Err("Memory bridge not connected".to_string())
    }
}

/// Bridge to agentic-identity for verified temporal operations.
pub trait IdentityBridge: Send + Sync {
    /// Verify who created a deadline or schedule
    fn verify_creator(&self, entity_id: &str, agent_id: &str) -> bool {
        let _ = (entity_id, agent_id);
        true // Default: trust all
    }

    /// Sign a temporal entity for integrity
    fn sign_entity(&self, entity_id: &str, content_hash: &str) -> Result<String, String> {
        let _ = (entity_id, content_hash);
        Err("Identity bridge not connected".to_string())
    }

    /// Anchor a temporal action receipt
    fn anchor_receipt(&self, action: &str, entity_id: &str) -> Result<String, String> {
        let _ = (action, entity_id);
        Err("Identity bridge not connected".to_string())
    }
}

/// Bridge to agentic-contract for SLA and obligation enforcement.
pub trait ContractBridge: Send + Sync {
    /// Check if a temporal action is allowed by policies
    fn check_policy(&self, operation: &str, context: &str) -> Result<bool, String> {
        let _ = (operation, context);
        Ok(true) // Default: allow all
    }

    /// Link a deadline to a contract obligation
    fn link_to_obligation(&self, deadline_id: &str, obligation_id: &str) -> Result<(), String> {
        let _ = (deadline_id, obligation_id);
        Err("Contract bridge not connected".to_string())
    }

    /// Report a deadline breach as a contract violation
    fn report_deadline_breach(&self, deadline_id: &str, details: &str) -> Result<(), String> {
        let _ = (deadline_id, details);
        Err("Contract bridge not connected".to_string())
    }

    /// Enforce SLA timing constraints
    fn enforce_sla(&self, schedule_id: &str, latency_ms: u64) -> Result<(), String> {
        let _ = (schedule_id, latency_ms);
        Err("Contract bridge not connected".to_string())
    }
}

/// Bridge to agentic-codebase for code-aware temporal context.
pub trait CodebaseBridge: Send + Sync {
    /// Get temporal context for a code symbol (when was it last changed)
    fn symbol_temporal_context(&self, symbol: &str) -> Option<String> {
        let _ = symbol;
        None
    }

    /// Link a schedule to a code review or deployment
    fn link_schedule_to_code(&self, schedule_id: &str, symbol: &str) -> Result<(), String> {
        let _ = (schedule_id, symbol);
        Err("Codebase bridge not connected".to_string())
    }
}

/// Bridge to agentic-vision for visual temporal tracking.
pub trait VisionBridge: Send + Sync {
    /// Capture visual state at a temporal checkpoint
    fn capture_at_checkpoint(&self, description: &str) -> Result<u64, String> {
        let _ = description;
        Err("Vision bridge not connected".to_string())
    }

    /// Link a temporal entity to a visual capture
    fn link_entity_to_capture(&self, entity_id: &str, capture_id: u64) -> Result<(), String> {
        let _ = (entity_id, capture_id);
        Err("Vision bridge not connected".to_string())
    }
}

/// Bridge to agentic-comm for temporal messaging.
pub trait CommBridge: Send + Sync {
    /// Schedule a message for future delivery
    fn schedule_message(&self, channel_id: u64, content: &str, deliver_at: u64) -> Result<String, String> {
        let _ = (channel_id, content, deliver_at);
        Err("Comm bridge not connected".to_string())
    }

    /// Broadcast a deadline alert to a channel
    fn broadcast_deadline_alert(&self, deadline_id: &str, channel_id: u64) -> Result<(), String> {
        let _ = (deadline_id, channel_id);
        Err("Comm bridge not connected".to_string())
    }
}

/// No-op implementation of all bridges for standalone use.
#[derive(Debug, Clone, Default)]
pub struct NoOpBridges;

impl MemoryBridge for NoOpBridges {}
impl IdentityBridge for NoOpBridges {}
impl ContractBridge for NoOpBridges {}
impl CodebaseBridge for NoOpBridges {}
impl VisionBridge for NoOpBridges {}
impl CommBridge for NoOpBridges {}

/// Configuration for which bridges are active.
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    pub memory_enabled: bool,
    pub identity_enabled: bool,
    pub contract_enabled: bool,
    pub codebase_enabled: bool,
    pub vision_enabled: bool,
    pub comm_enabled: bool,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            memory_enabled: false,
            identity_enabled: false,
            contract_enabled: false,
            codebase_enabled: false,
            vision_enabled: false,
            comm_enabled: false,
        }
    }
}

/// Hydra adapter trait — future orchestrator discovery interface.
pub trait HydraAdapter: Send + Sync {
    fn adapter_id(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    fn handle_request(&self, method: &str, params: &str) -> Result<String, String>;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_bridges_implements_all_traits() {
        let b = NoOpBridges;
        let _: &dyn MemoryBridge = &b;
        let _: &dyn IdentityBridge = &b;
        let _: &dyn ContractBridge = &b;
        let _: &dyn CodebaseBridge = &b;
        let _: &dyn VisionBridge = &b;
        let _: &dyn CommBridge = &b;
    }

    #[test]
    fn memory_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.store_temporal_event("deadline", "details").is_err());
        assert!(b.recall_schedule_history("topic", 10).is_empty());
        assert!(b.link_deadline_to_decision("dl-1", 1).is_err());
    }

    #[test]
    fn identity_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.verify_creator("dl-1", "agent-1"));
        assert!(b.sign_entity("dl-1", "hash").is_err());
        assert!(b.anchor_receipt("create", "dl-1").is_err());
    }

    #[test]
    fn contract_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.check_policy("create_deadline", "ctx").unwrap());
        assert!(b.link_to_obligation("dl-1", "obl-1").is_err());
        assert!(b.report_deadline_breach("dl-1", "overdue").is_err());
        assert!(b.enforce_sla("sched-1", 100).is_err());
    }

    #[test]
    fn codebase_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.symbol_temporal_context("my_func").is_none());
        assert!(b.link_schedule_to_code("sched-1", "my_func").is_err());
    }

    #[test]
    fn vision_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.capture_at_checkpoint("screenshot").is_err());
        assert!(b.link_entity_to_capture("dl-1", 1).is_err());
    }

    #[test]
    fn comm_bridge_defaults() {
        let b = NoOpBridges;
        assert!(b.schedule_message(1, "hello", 1000).is_err());
        assert!(b.broadcast_deadline_alert("dl-1", 1).is_err());
    }

    #[test]
    fn bridge_config_defaults_all_false() {
        let cfg = BridgeConfig::default();
        assert!(!cfg.memory_enabled);
        assert!(!cfg.identity_enabled);
        assert!(!cfg.contract_enabled);
        assert!(!cfg.codebase_enabled);
        assert!(!cfg.vision_enabled);
        assert!(!cfg.comm_enabled);
    }

    #[test]
    fn noop_bridges_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NoOpBridges>();
    }

    #[test]
    fn noop_bridges_default_and_clone() {
        let b = NoOpBridges::default();
        let _b2 = b.clone();
    }
}
