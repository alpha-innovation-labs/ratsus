use anyhow::{bail, Result};

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::nexus_sessions::focused_nexus_session_working_dir::focused_nexus_session_working_dir;
use crate::nexus_sessions::spawn_new_nexus_session_terminal::spawn_new_nexus_session_terminal;
use crate::session_panes::active_terminal_spawn_area::active_terminal_spawn_area;
use crate::session_panes::ensure_active_terminal_pane_session::ensure_active_terminal_pane_session;
use crate::session_panes::set_active_terminal_pane_session::set_active_terminal_pane_session;
use crate::session_panes::terminal_split_direction::TerminalSplitDirection;

/// Opens a new Nexus chat in a split adjacent to the active terminal pane.
pub fn split_active_terminal_pane(
    app: &mut NexusDemo,
    direction: TerminalSplitDirection,
) -> Result<()> {
    ensure_active_terminal_pane_session(app);
    let working_dir = focused_nexus_session_working_dir(app)?;
    let area = active_terminal_spawn_area(app);
    let session_terminal =
        spawn_new_nexus_session_terminal(&working_dir, area.height.max(1), area.width.max(1))?;
    let session_id = session_terminal.session.id.clone();
    let new_pane_id = match direction {
        TerminalSplitDirection::Right => app
            .terminal_layout
            .split_pane_vertically(app.active_terminal_pane_id),
        TerminalSplitDirection::Bottom => app
            .terminal_layout
            .split_pane_horizontally(app.active_terminal_pane_id),
    };
    let Some(new_pane_id) = new_pane_id else {
        bail!("Failed to split active terminal pane");
    };

    app.session_terminals.push(session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    let new_index = app.session_terminals.len() - 1;
    app.active_terminal_pane_id = new_pane_id;
    set_active_terminal_pane_session(app, session_id);
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.keep_focused_session_visible();
    Ok(())
}
