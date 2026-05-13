use std::collections::BTreeSet;

use crate::app::deletion::session_index_after_delete::session_index_after_delete;
use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::bundle::prune_session_bundles::prune_terminal_pane_session_bundles;
use crate::ui::left_panel::focus::session_visible_row_index::session_visible_row_index;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;
use crate::ui::left_panel::session::visible_rows::visible_session_rows;

/// Restores active, focused, and folder state after deleting multiple sessions.
pub fn restore_focus_after_bulk_delete(app: &mut AppState, preferred_row: usize) {
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    prune_terminal_pane_session_bundles(app);
    if app.session_terminals.is_empty() {
        clear_focus(app);
        return;
    }
    let expanded_rows = visible_session_rows(
        &app.session_terminals,
        &BTreeSet::new(),
        &app.folder_order,
        None,
    );
    let Some(next_index) = session_index_after_delete(&expanded_rows, preferred_row) else {
        return;
    };
    open_next_session_folder(app, next_index);
    focus_next_session(app, next_index);
}

/// Clears focus when no sessions remain.
fn clear_focus(app: &mut AppState) {
    app.active_index = 0;
    app.focused_index = 0;
    app.focused_row = 0;
}

/// Opens the folder containing the next focused session.
fn open_next_session_folder(app: &mut AppState, next_index: usize) {
    if let Some(entry) = app.session_terminals.get(next_index) {
        app.collapsed_folders.remove(&entry.session.working_dir);
    }
}

/// Focuses and activates the next session after deletion.
fn focus_next_session(app: &mut AppState, next_index: usize) {
    let rows = visible_session_rows(
        &app.session_terminals,
        &app.collapsed_folders,
        &app.folder_order,
        Some(next_index),
    );
    app.focused_index = next_index;
    app.active_index = next_index;
    app.focused_row = session_visible_row_index(&rows, next_index).unwrap_or(0);
    app.activate_focused_session();
}
