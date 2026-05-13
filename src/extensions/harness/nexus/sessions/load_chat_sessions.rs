use std::io;
use std::process::Command;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::nexus::parsing::parse_chat_sessions_json::parse_chat_sessions_json;

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
    parse_chat_sessions_json(&stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
