use anyhow::Result;

use crate::app::sessions::normal_terminal_insert_index::normal_terminal_insert_index;
use crate::app::sessions::normal_terminal_working_dir::normal_terminal_working_dir;
use crate::app::state::app_state::AppState;
use crate::extensions::terminal::persistence::persist_normal_terminal_sessions::persist_normal_terminal_sessions;
use crate::extensions::terminal::process::spawn_normal_terminal_session::spawn_normal_terminal_session;
use crate::ui::grid_layout::split::active_spawn_area::active_terminal_spawn_area;
use crate::ui::grid_layout::split::set_active_session::set_active_terminal_pane_session;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Starts a normal shell terminal in the active selected session working directory.
pub fn start_new_normal_terminal(app: &mut AppState) -> Result<()> {
    let working_dir = normal_terminal_working_dir(
        &app.session_terminals,
        app.active_index,
        app.focused_index,
        None,
    )?;
    let area = active_terminal_spawn_area(app);
    let rows = area.height.max(1);
    let cols = area.width.max(1);
    let session_terminal = match app
        .chat_harness
        .spawn_normal_terminal(&working_dir, rows, cols)
    {
        Some(result) => result?,
        None => spawn_normal_terminal_session(&working_dir, rows, cols)?,
    };
    let session_id = session_terminal.session.id.clone();

    let new_index = normal_terminal_insert_index(app.session_terminals.len(), app.focused_index);
    app.session_terminals.insert(new_index, session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = area;
    if app.chat_harness.persist_normal_terminals() {
        persist_normal_terminal_sessions(&app.session_terminals);
    }
    set_active_terminal_pane_session(app, session_id);
    app.keep_focused_session_visible();
    Ok(())
}
