use std::io;
use std::process::Command;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::nexus::parsing::parse_chat_sessions_json::parse_chat_sessions_json;
use crate::extensions::harness::nexus::status::load_chat_status_file::load_nexus_chat_status_file;

/// Loads every resumable Nexus session as JSON.
pub fn load_chat_sessions() -> io::Result<Vec<ChatSession>> {
    let output = Command::new("nexus")
        .args(["--sessions-all", "--json"])
        .output()?;
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    match parse_chat_sessions_json(&stdout) {
        Ok(sessions) => Ok(sessions),
        Err(_) => load_nexus_chat_status_file(),
    }
}
