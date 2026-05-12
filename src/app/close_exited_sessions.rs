use anyhow::Result;

use crate::app::exited_session_indices::exited_session_indices;
use crate::app::nexus_demo_state::NexusDemo;
use crate::app::remove_exited_sessions::remove_exited_sessions;
use crate::app::restore_focus_after_removals::restore_focus_after_removals;
use crate::left_panel::sync_folder_order::sync_folder_order;

/// Closes entries whose backing terminal process has exited.
pub fn close_exited_sessions(app: &mut NexusDemo) -> Result<bool> {
    let exited_indices = exited_session_indices(&mut app.session_terminals);
    if exited_indices.is_empty() {
        return Ok(false);
    }

    let removed = remove_exited_sessions(app, &exited_indices);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    restore_focus_after_removals(app, &exited_indices, removed)?;
    Ok(true)
}
