use anyhow::Result;

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::nexus_sessions::focused_nexus_session_working_dir::focused_nexus_session_working_dir;
use crate::terminal::spawn_normal_terminal_session::spawn_normal_terminal_session;

/// Starts a normal shell terminal in the focused session working directory.
pub fn start_new_normal_terminal(app: &mut NexusDemo) -> Result<()> {
    let working_dir = focused_nexus_session_working_dir(app)?;
    let rows = app.last_terminal_area.height.max(1);
    let cols = app.last_terminal_area.width.max(1);
    let session_terminal = spawn_normal_terminal_session(&working_dir, rows, cols)?;

    app.session_terminals.push(session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    let new_index = app.session_terminals.len() - 1;
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = app.last_terminal_area;
    app.keep_focused_session_visible();
    Ok(())
}
