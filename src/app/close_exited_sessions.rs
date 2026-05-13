use anyhow::Result;

use crate::app::app_state::AppState;
use crate::app::exited_session_indices::exited_session_indices;
use crate::app::exited_session_pane_ids::exited_session_pane_ids;
use crate::app::remove_exited_sessions::remove_exited_sessions;
use crate::app::restore_focus_after_removals::restore_focus_after_removals;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::session_panes::close_exited_terminal_panes::close_exited_terminal_panes;
use crate::terminal::persist_normal_terminal_sessions::persist_normal_terminal_sessions;

/// Closes entries whose backing terminal process has exited.
pub fn close_exited_sessions(app: &mut AppState) -> Result<bool> {
    let exited_indices = exited_session_indices(&mut app.session_terminals);
    if exited_indices.is_empty() {
        return Ok(false);
    }

    let pane_ids = exited_session_pane_ids(app, &exited_indices);
    let removed = remove_exited_sessions(app, &exited_indices);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    let closed_exited_pane = close_exited_terminal_panes(app, &pane_ids);
    if app.chat_harness.persist_normal_terminals() {
        persist_normal_terminal_sessions(&app.session_terminals);
    }
    restore_focus_after_removals(app, &exited_indices, removed, closed_exited_pane)?;
    Ok(true)
}
