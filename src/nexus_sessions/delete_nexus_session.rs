use std::io;
use std::process::Command;

/// Deletes a persisted Nexus chat session through the Nexus CLI.
pub fn delete_nexus_session(session_id: &str) -> io::Result<()> {
    let output = Command::new("nexus")
        .args(["--delete-session", session_id])
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}
