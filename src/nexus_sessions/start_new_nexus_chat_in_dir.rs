use std::path::Path;

use anyhow::Result;

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::nexus_sessions::spawn_new_nexus_session_terminal::spawn_new_nexus_session_terminal;
use crate::session_panes::active_terminal_spawn_area::active_terminal_spawn_area;
use crate::session_panes::set_active_terminal_pane_session::set_active_terminal_pane_session;

/// Starts a fresh Nexus chat in a specific working directory and focuses it.
pub fn start_new_nexus_chat_in_dir(app: &mut NexusDemo, working_dir: &Path) -> Result<()> {
    let area = active_terminal_spawn_area(app);
    let rows = area.height.max(1);
    let cols = area.width.max(1);
    let session_terminal = spawn_new_nexus_session_terminal(working_dir, rows, cols)?;
    let session_id = session_terminal.session.id.clone();

    app.session_terminals.push(session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    let new_index = app.session_terminals.len() - 1;
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = area;
    set_active_terminal_pane_session(app, session_id);
    app.keep_focused_session_visible();
    Ok(())
}
