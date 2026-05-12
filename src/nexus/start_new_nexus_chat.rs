use anyhow::Result;

use crate::focused_nexus_session_working_dir::focused_nexus_session_working_dir;
use crate::focused_pane::FocusedPane;
use crate::nexus_demo_state::NexusDemo;
use crate::spawn_new_nexus_session_terminal::spawn_new_nexus_session_terminal;

/// Starts a fresh Nexus chat beside the existing sessions and focuses it.
pub fn start_new_nexus_chat(app: &mut NexusDemo) -> Result<()> {
    let working_dir = focused_nexus_session_working_dir(app)?;
    let rows = app.last_terminal_area.height.max(1);
    let cols = app.last_terminal_area.width.max(1);
    let session_terminal = spawn_new_nexus_session_terminal(&working_dir, rows, cols)?;

    app.session_terminals.push(session_terminal);
    let new_index = app.session_terminals.len() - 1;
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = app.last_terminal_area;
    app.keep_focused_session_visible();
    Ok(())
}
