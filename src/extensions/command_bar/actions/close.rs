use crate::app::state::app_state::AppState;

/// Closes the command bar and clears transient keyboard state.
pub fn close_command_bar(app: &mut AppState) {
    app.command_bar.is_open = false;
    app.command_bar.is_filtering = false;
    app.command_bar.pending_g = false;
}
