use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::group::compact_split_pane_session_groups::compact_split_pane_session_groups;
use crate::ui::grid_layout::group::remove_pane_from_groups::remove_pane_from_groups;
use crate::ui::grid_layout::pane::activate_terminal_pane::activate_terminal_pane;
use crate::ui::grid_layout::pane::fallback_terminal_pane_id::fallback_terminal_pane_id;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;

/// Removes a split terminal pane from the layout and restores focus to a remaining pane.
pub fn close_terminal_pane(app: &mut AppState, pane_id: PaneId) -> bool {
    if app
        .terminal_layout
        .layout_panes(app.last_terminal_area)
        .len()
        <= 1
    {
        return false;
    }
    if !app.terminal_layout.remove_pane(pane_id) {
        return false;
    }
    app.terminal_pane_sessions.remove(&pane_id);
    app.terminal_pane_session_bundles.remove(&pane_id);
    app.terminal_pane_areas.remove(&pane_id);
    app.terminal_pane_close_buttons.remove(&pane_id);
    remove_pane_from_groups(&mut app.split_pane_session_groups, pane_id);
    compact_split_pane_session_groups(
        &mut app.split_pane_session_groups,
        &app.terminal_pane_session_bundles,
    );

    if app.active_terminal_pane_id == pane_id {
        if let Some(fallback_pane_id) = fallback_terminal_pane_id(app) {
            activate_terminal_pane(app, fallback_pane_id);
        }
    }
    persist_multiplexer_state(app);
    true
}

#[cfg(test)]
mod tests {
    use super::close_terminal_pane;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;

    /// Closing a split removes its pane state and leaves the remaining pane active.
    #[test]
    fn removes_split_pane_and_state() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha", "a", "/tmp/project"),
            dormant_session("Beta", "b", "/tmp/project"),
        ])?;
        let split_pane = app
            .terminal_layout
            .split_pane_vertically(TERMINAL_PANE_ID)
            .unwrap();
        app.terminal_pane_sessions
            .insert(TERMINAL_PANE_ID, "a".into());
        app.terminal_pane_sessions.insert(split_pane, "b".into());
        app.active_terminal_pane_id = split_pane;

        assert!(close_terminal_pane(&mut app, split_pane));

        assert_eq!(
            app.terminal_layout
                .layout_panes(app.last_terminal_area)
                .len(),
            1
        );
        assert!(!app.terminal_pane_sessions.contains_key(&split_pane));
        assert_eq!(app.active_terminal_pane_id, TERMINAL_PANE_ID);
        Ok(())
    }
}
