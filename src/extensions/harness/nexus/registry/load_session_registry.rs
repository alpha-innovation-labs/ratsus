use std::fs;
use std::io;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::nexus::config::cmux_session_registry_path::nexus_cmux_session_registry_path;
use crate::extensions::harness::nexus::process::is_alive::process_is_alive;
use crate::extensions::harness::nexus::process::session_file_is_active::session_file_is_active;
use crate::extensions::harness::nexus::registry::parse_session_registry::parse_nexus_session_registry;

/// Loads Nexus session metadata from the cmux registry JSON file.
pub fn load_nexus_session_registry() -> io::Result<Vec<ChatSession>> {
    let path = nexus_cmux_session_registry_path();
    let output = fs::read_to_string(path)?;
    parse_nexus_session_registry(&output, process_is_alive, session_file_is_active)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
