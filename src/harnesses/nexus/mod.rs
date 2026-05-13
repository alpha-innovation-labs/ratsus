//! Nexus-backed chat harness implementation.

pub mod apply_session_refresh;
#[cfg(test)]
mod apply_session_refresh_tests;
pub mod delete_nexus_session;
pub mod is_new_nexus_chat_session;
pub mod load_chat_sessions;
pub mod load_nexus_session_registry;
pub mod merge_registry_session_metadata;
pub mod new_nexus_chat_session;
pub mod nexus_cmux_session_registry_path;
pub mod nexus_harness;
pub mod observations;
pub mod parse_chat_sessions;
pub mod parse_chat_sessions_json;
pub mod parse_nexus_session_registry;
pub mod process_is_alive;
pub mod sanitize_chat_sessions_json;
pub mod sanitize_json_control_characters;
pub mod session_file_is_active;
pub mod spawn_new_nexus_session_terminal;

pub use nexus_harness::NexusHarness;
