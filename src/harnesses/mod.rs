//! Concrete chat harness implementations.

use std::sync::Arc;

use crate::harness::ChatHarness;

pub mod nexus;
pub mod stub;

/// Builds the real Nexus-backed chat harness.
pub fn nexus_harness() -> Arc<dyn ChatHarness> {
    Arc::new(nexus::NexusHarness)
}
