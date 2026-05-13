use crate::app::state::app_state::AppState;
use crate::ui::left_panel::focus::focus_row::focus_left_panel_row;

/// Moves left-panel focus to the last visible row.
pub fn focus_left_panel_end(app: &mut AppState) {
    let last_index = app.visible_rows().len().saturating_sub(1);
    focus_left_panel_row(app, last_index);
}
