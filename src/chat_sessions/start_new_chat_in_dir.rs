use std::path::Path;

use anyhow::Result;

use crate::app::app_state::AppState;
use crate::chat_sessions::new_chat_insert_index::new_chat_insert_index;
use crate::chat_sessions::promote_new_chat_folder::promote_new_chat_folder;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::session_panes::active_terminal_spawn_area::active_terminal_spawn_area;
use crate::session_panes::set_active_terminal_pane_session::set_active_terminal_pane_session;

/// Starts a fresh chat in a specific working directory and focuses it.
pub fn start_new_chat_in_dir(app: &mut AppState, working_dir: &Path) -> Result<()> {
    let area = active_terminal_spawn_area(app);
    let rows = area.height.max(1);
    let cols = area.width.max(1);
    let session_terminal = app.chat_harness.spawn_new_chat(working_dir, rows, cols)?;
    let session_id = session_terminal.session.id.clone();

    let new_index = new_chat_insert_index(&app.session_terminals, working_dir);
    app.session_terminals.insert(new_index, session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    promote_new_chat_folder(&mut app.folder_order, working_dir);
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = area;
    set_active_terminal_pane_session(app, session_id);
    app.keep_focused_session_visible();
    persist_session_order_preferences(app);
    Ok(())
}
