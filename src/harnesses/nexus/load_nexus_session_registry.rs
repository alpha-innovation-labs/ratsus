use std::fs;
use std::io;

use crate::harness::chat_session::ChatSession;
use crate::harnesses::nexus::nexus_cmux_session_registry_path::nexus_cmux_session_registry_path;
use crate::harnesses::nexus::parse_nexus_session_registry::parse_nexus_session_registry;
use crate::harnesses::nexus::process_is_alive::process_is_alive;
use crate::harnesses::nexus::session_file_is_active::session_file_is_active;

/// Loads Nexus session metadata from the cmux registry JSON file.
pub fn load_nexus_session_registry() -> io::Result<Vec<ChatSession>> {
    let path = nexus_cmux_session_registry_path();
    let output = fs::read_to_string(path)?;
    parse_nexus_session_registry(&output, process_is_alive, session_file_is_active)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
