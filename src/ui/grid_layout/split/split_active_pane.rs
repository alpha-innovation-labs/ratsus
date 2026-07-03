use anyhow::{bail, Result};

use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::selection::focused_working_dir::focused_chat_session_working_dir;
use crate::ui::grid_layout::group::ensure_group_for_split::ensure_group_for_split;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::grid_layout::split::active_spawn_area::active_terminal_spawn_area;
use crate::ui::grid_layout::split::ensure_active_session::ensure_active_terminal_pane_session;
use crate::ui::grid_layout::split::set_active_session::set_active_terminal_pane_session;
use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;
use crate::ui::grid_layout::split::split_terminal_layout::split_terminal_layout;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Opens a new chat in a split adjacent to the active terminal pane.
pub fn split_active_terminal_pane(
    app: &mut AppState,
    direction: TerminalSplitDirection,
) -> Result<()> {
    ensure_active_terminal_pane_session(app);
    let working_dir = focused_chat_session_working_dir(app)?;
    let area = active_terminal_spawn_area(app);
    let session_terminal =
        app.chat_harness
            .spawn_new_chat(&working_dir, area.height.max(1), area.width.max(1))?;
    let session_id = session_terminal.session.id.clone();
    let source_pane_id = app.active_terminal_pane_id;
    let new_pane_id = split_terminal_layout(&mut app.terminal_layout, source_pane_id, direction);
    let Some(new_pane_id) = new_pane_id else {
        bail!("Failed to split active terminal pane");
    };

    ensure_group_for_split(
        &mut app.split_pane_session_groups,
        source_pane_id,
        new_pane_id,
    );
    app.session_terminals.push(session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    let new_index = app.session_terminals.len() - 1;
    app.active_terminal_pane_id = new_pane_id;
    set_active_terminal_pane_session(app, session_id);
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.keep_focused_session_visible();
    persist_multiplexer_state(app);
    Ok(())
}
