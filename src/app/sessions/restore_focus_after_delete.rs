use crate::app::sessions::clamp_session_index::clamp_session_index;
use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::bundle::prune_session_bundles::prune_terminal_pane_session_bundles;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Restores active, focused, and folder state after deleting one session.
pub fn restore_focus_after_delete(
    app: &mut AppState,
    deleted_index: usize,
    deleted_was_active: bool,
) {
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    prune_terminal_pane_session_bundles(app);
    if app.session_terminals.is_empty() {
        app.active_index = 0;
        app.focused_index = 0;
        app.focused_row = 0;
        return;
    }
    if deleted_was_active {
        app.active_index = clamp_session_index(deleted_index, app.session_terminals.len());
        app.focused_index = app.active_index;
        app.activate_focused_session();
        return;
    }
    if app.active_index > deleted_index {
        app.active_index = app.active_index.saturating_sub(1);
    }
    if app.focused_index > deleted_index {
        app.focused_index = app.focused_index.saturating_sub(1);
    }
    app.focused_index = clamp_session_index(app.focused_index, app.session_terminals.len());
    app.keep_focused_session_visible();
}
