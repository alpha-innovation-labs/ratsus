use std::io;
use std::path::PathBuf;

use crate::nexus_demo_state::NexusDemo;

/// Returns the working directory for the currently focused Nexus session.
pub fn focused_nexus_session_working_dir(app: &NexusDemo) -> io::Result<PathBuf> {
    if let Some(entry) = app.session_terminals.get(app.focused_index) {
        return Ok(entry.session.working_dir.clone());
    }
    if let Some(entry) = app.session_terminals.get(app.active_index) {
        return Ok(entry.session.working_dir.clone());
    }
    std::env::current_dir()
}
