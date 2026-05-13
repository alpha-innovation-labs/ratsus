//! Nexus CLI session discovery, refresh, and session creation.

pub mod apply_session_refresh;
#[cfg(test)]
mod apply_session_refresh_tests;
pub mod delete_nexus_session;
pub mod drain_session_refreshes;
pub mod focused_nexus_session_working_dir;
pub mod is_new_nexus_chat_session;
pub mod load_nexus_session_registry;
pub mod load_nexus_sessions;
pub mod merge_registry_session_metadata;
pub mod new_nexus_chat_session;
pub mod nexus_cmux_session_registry_path;
pub mod parse_nexus_session_registry;
pub mod parse_nexus_sessions;
pub mod parse_nexus_sessions_json;
pub mod process_is_alive;
pub mod sanitize_json_control_characters;
pub mod sanitize_nexus_sessions_json;
pub mod session_file_is_active;
pub mod session_info;
pub mod spawn_new_nexus_session_terminal;
pub mod spawn_session_refresh_worker;
pub mod start_new_nexus_chat;
pub mod start_new_nexus_chat_in_dir;
