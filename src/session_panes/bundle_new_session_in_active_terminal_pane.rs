use anyhow::Result;

use crate::app::app_state::AppState;
use crate::chat_sessions::focused_chat_session_working_dir::focused_chat_session_working_dir;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::session_panes::active_terminal_bundle_insert_index::active_terminal_bundle_insert_index;
use crate::session_panes::active_terminal_spawn_area::active_terminal_spawn_area;
use crate::session_panes::set_active_terminal_pane_bundle_session::set_active_terminal_pane_bundle_session;

/// Opens a new chat and bundles it into the active terminal split pane.
pub fn bundle_new_session_in_active_terminal_pane(app: &mut AppState) -> Result<()> {
    let working_dir = focused_chat_session_working_dir(app)?;
    let area = active_terminal_spawn_area(app);
    let session_terminal =
        app.chat_harness
            .spawn_new_chat(&working_dir, area.height.max(1), area.width.max(1))?;
    let session_id = session_terminal.session.id.clone();
    let insert_index = active_terminal_bundle_insert_index(app);
    let previous_session_id = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.id.clone());

    app.session_terminals.insert(insert_index, session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    app.active_index = insert_index;
    app.focused_index = insert_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = area;
    if let Some(previous_session_id) = previous_session_id {
        app.terminal_pane_session_bundles
            .entry(app.active_terminal_pane_id)
            .or_default()
            .push(previous_session_id);
    }
    set_active_terminal_pane_bundle_session(app, app.active_terminal_pane_id, session_id);
    app.keep_focused_session_visible();
    Ok(())
}
