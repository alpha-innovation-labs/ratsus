use crate::app::state::app_state::AppState;

/// Opens the command bar and resets transient query and selection state.
pub fn open_command_bar(app: &mut AppState) {
    app.command_bar.is_open = true;
    app.command_bar.query.clear();
    app.command_bar.is_filtering = true;
    app.command_bar.selected_position = 0;
    app.command_bar.pending_g = false;
}
