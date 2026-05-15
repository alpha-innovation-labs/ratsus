use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroupId;
use crate::ui::left_panel::session::activation::activate_split_group_child::activate_split_group_child;

/// Activates the currently active child in a split group, or the first child when none is active.
pub fn activate_split_group_parent(app: &mut AppState, group_id: SplitPaneSessionGroupId) {
    let Some((pane_id, index)) = child_target_for_group(app, group_id) else {
        return;
    };
    activate_split_group_child(app, pane_id, index);
}

/// Finds the child target that parent-row selection should activate.
fn child_target_for_group(
    app: &AppState,
    group_id: SplitPaneSessionGroupId,
) -> Option<(ratkit::primitives::resizable_grid::PaneId, usize)> {
    let group = app.split_pane_session_groups.groups.get(&group_id)?;
    let active_session_id = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.id.as_str());
    for pane_id in &group.panes {
        let Some(session_ids) = app.terminal_pane_session_bundles.get(pane_id) else {
            continue;
        };
        if let Some(active_session_id) = active_session_id {
            if session_ids
                .iter()
                .any(|candidate| candidate == active_session_id)
            {
                return session_index_for_id(app, *pane_id, active_session_id);
            }
        }
    }
    group.panes.iter().find_map(|pane_id| {
        app.terminal_pane_session_bundles
            .get(pane_id)
            .and_then(|session_ids| session_index_for_ids(app, *pane_id, session_ids))
    })
}

/// Resolves one session id to its pane and current session index.
fn session_index_for_id(
    app: &AppState,
    pane_id: ratkit::primitives::resizable_grid::PaneId,
    session_id: &str,
) -> Option<(ratkit::primitives::resizable_grid::PaneId, usize)> {
    app.session_terminals
        .iter()
        .position(|entry| entry.session.id == session_id)
        .map(|index| (pane_id, index))
}

/// Resolves the first valid session index in a pane bundle.
fn session_index_for_ids(
    app: &AppState,
    pane_id: ratkit::primitives::resizable_grid::PaneId,
    session_ids: &[String],
) -> Option<(ratkit::primitives::resizable_grid::PaneId, usize)> {
    session_ids.iter().find_map(|session_id| {
        app.session_terminals
            .iter()
            .position(|entry| entry.session.id == *session_id)
            .map(|index| (pane_id, index))
    })
}
