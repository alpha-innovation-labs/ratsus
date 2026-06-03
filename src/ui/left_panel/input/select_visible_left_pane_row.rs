use crate::app::state::app_state::AppState;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::active_content::ActiveLeftPaneContent;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Focuses the indexed visible row in whichever content the left pane is showing.
pub fn select_visible_left_pane_row(app: &mut AppState, row_index: usize) -> LeftPaneActionOutcome {
    let mut content = ActiveLeftPaneContent::for_app(app);
    content.handle_left_pane_action(LeftPaneAction::FocusVisibleRow(row_index))
}
