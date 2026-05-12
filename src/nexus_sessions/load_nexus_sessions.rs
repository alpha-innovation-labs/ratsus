use std::io;
use std::process::Command;

use crate::nexus_sessions::parse_nexus_sessions_json::parse_nexus_sessions_json;
use crate::nexus_sessions::session_info::NexusSession;

/// Loads every resumable Nexus session as JSON.
pub fn load_nexus_sessions() -> io::Result<Vec<NexusSession>> {
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
    parse_nexus_sessions_json(&stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
