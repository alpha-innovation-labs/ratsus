use std::io;
use std::path::PathBuf;

use crate::app::state::app_state::AppState;

/// Returns the working directory for the currently focused chat session.
pub fn focused_chat_session_working_dir(app: &AppState) -> io::Result<PathBuf> {
    if let Some(entry) = app.session_terminals.get(app.focused_index) {
        return Ok(entry.session.working_dir.clone());
    }
    if let Some(entry) = app.session_terminals.get(app.active_index) {
        return Ok(entry.session.working_dir.clone());
    }
    std::env::current_dir()
}
