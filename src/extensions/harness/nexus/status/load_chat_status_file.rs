use std::fs;
use std::io;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::nexus::config::chat_status_file_path::nexus_chat_status_file_path;
use crate::extensions::harness::nexus::process::is_alive::process_is_alive;
use crate::extensions::harness::nexus::process::session_file_is_active::session_file_is_active;
use crate::extensions::harness::nexus::status::parse_chat_status_file::parse_nexus_chat_status_file;

/// Loads Nexus session status metadata from the chat status JSON file.
pub fn load_nexus_chat_status_file() -> io::Result<Vec<ChatSession>> {
    let path = nexus_chat_status_file_path();
    let output = fs::read_to_string(path)?;
    parse_nexus_chat_status_file(&output, process_is_alive, session_file_is_active)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
