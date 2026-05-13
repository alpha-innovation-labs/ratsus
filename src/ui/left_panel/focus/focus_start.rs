use crate::app::state::app_state::AppState;
use crate::ui::left_panel::focus::focus_row::focus_left_panel_row;

/// Moves left-panel focus to the first visible row.
pub fn focus_left_panel_start(app: &mut AppState) {
    focus_left_panel_row(app, 0);
}
