//! Harness-backed chat session capabilities and backend adapters.

use std::sync::Arc;

pub mod conversation_picker;
pub mod core;
pub mod nexus;
pub mod observations;
pub mod sessions;
pub mod stub;

pub use core::{ChatHarness, ChatSession, ChatSessionKind};

/// Builds the real Nexus-backed chat harness.
pub fn nexus_harness() -> Arc<dyn ChatHarness> {
    Arc::new(nexus::NexusHarness)
}
