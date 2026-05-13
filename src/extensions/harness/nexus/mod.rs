//! Nexus-backed chat harness implementation.

pub mod adapter;
pub mod config;
pub mod parsing;
pub mod process;
pub mod refresh;
pub mod registry;
pub mod sessions;

pub use adapter::nexus_harness::NexusHarness;
