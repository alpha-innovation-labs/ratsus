use std::collections::BTreeMap;

use ratatui::layout::Rect;

use crate::app::test_support::app_fixture::app_fixture;
use crate::app::test_support::dormant_session::dormant_session;
use crate::ui::grid_layout::group::ensure_group_for_split::ensure_group_for_split;
use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;

/// Selecting a standalone session should suspend, not mutate, the grouped layout.
#[test]
fn activating_standalone_session_suspends_group_layout() -> anyhow::Result<()> {
    let mut app = grouped_app_fixture()?;

    app.focused_index = 2;
    app.activate_focused_session();

    assert_eq!(current_pane_count(&app), 1);
    assert_eq!(
        app.terminal_pane_sessions.get(&TERMINAL_PANE_ID),
        Some(&"c".to_string())
    );
    assert!(app.suspended_multiplexer_state.is_some());
    Ok(())
}

/// Selecting a grouped session while standalone should restore the full group layout.
#[test]
fn activating_grouped_session_restores_group_layout() -> anyhow::Result<()> {
    let mut app = grouped_app_fixture()?;
    app.focused_index = 2;
    app.activate_focused_session();

    app.focused_index = 0;
    app.activate_focused_session();

    assert_eq!(current_pane_count(&app), 2);
    assert_eq!(app.active_index, 0);
    assert_eq!(app.split_pane_session_groups.groups.len(), 1);
    Ok(())
}

/// Builds an app with two sessions in one split group and one standalone session.
fn grouped_app_fixture() -> anyhow::Result<crate::app::state::app_state::AppState> {
    let mut app = app_fixture(vec![
        dormant_session("Alpha", "a", "/tmp/project"),
        dormant_session("Beta", "b", "/tmp/project"),
        dormant_session("Gamma", "c", "/tmp/project"),
    ])?;
    app.last_terminal_area = Rect::new(0, 0, 120, 40);
    let right_pane = app
        .terminal_layout
        .split_pane_vertically(TERMINAL_PANE_ID)
        .expect("split pane exists");
    app.terminal_pane_sessions = BTreeMap::from([
        (TERMINAL_PANE_ID, "a".to_string()),
        (right_pane, "b".to_string()),
    ]);
    app.terminal_pane_session_bundles = BTreeMap::from([
        (TERMINAL_PANE_ID, vec!["a".to_string()]),
        (right_pane, vec!["b".to_string()]),
    ]);
    ensure_group_for_split(
        &mut app.split_pane_session_groups,
        TERMINAL_PANE_ID,
        right_pane,
    );
    Ok(app)
}

/// Returns the number of panes currently in the terminal layout.
fn current_pane_count(app: &crate::app::state::app_state::AppState) -> usize {
    app.terminal_layout
        .layout_panes(app.last_terminal_area)
        .len()
}
