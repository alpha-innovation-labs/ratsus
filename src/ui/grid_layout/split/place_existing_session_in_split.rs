use anyhow::{bail, Result};

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::group::ensure_group_for_split::ensure_group_for_split;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::grid_layout::split::ensure_active_session::ensure_active_terminal_pane_session;
use crate::ui::grid_layout::split::set_active_session::set_active_terminal_pane_session;
use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;
use crate::ui::grid_layout::split::split_terminal_layout::split_terminal_layout;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Places an existing session in a newly split terminal pane and focuses that pane.
pub fn place_existing_session_in_terminal_split(
    app: &mut AppState,
    index: usize,
    direction: TerminalSplitDirection,
) -> Result<()> {
    let Some(session_id) = app
        .session_terminals
        .get(index)
        .map(|entry| entry.session.id.clone())
    else {
        return Ok(());
    };
    ensure_active_terminal_pane_session(app);
    let source_pane_id = app.active_terminal_pane_id;
    let new_pane_id = split_terminal_layout(&mut app.terminal_layout, source_pane_id, direction);
    let Some(new_pane_id) = new_pane_id else {
        bail!("Failed to split active terminal pane for existing session");
    };

    ensure_group_for_split(
        &mut app.split_pane_session_groups,
        source_pane_id,
        new_pane_id,
    );
    app.active_terminal_pane_id = new_pane_id;
    set_active_terminal_pane_session(app, session_id);
    app.active_index = index;
    app.focused_index = index;
    app.focused_pane = FocusedPane::Terminal;
    app.keep_focused_session_visible();
    persist_multiplexer_state(app);
    Ok(())
}
