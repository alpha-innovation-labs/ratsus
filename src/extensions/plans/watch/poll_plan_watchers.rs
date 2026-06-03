use crate::app::state::app_state::AppState;

/// Polls plan file watchers and reports whether visible plan state changed.
pub fn poll_plan_watchers(app: &mut AppState) -> bool {
    app.plan_list.poll_watchers()
}
