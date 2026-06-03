use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl PlanListState {
    /// Starts text filtering for the plan list.
    pub fn start_filtering(&mut self) {
        self.filtering = true;
    }

    /// Adds one character to the active filter query.
    pub fn push_filter_character(&mut self, character: char) {
        self.filter_query.push(character);
        self.focus_first_plan_row();
        self.activate_focused();
    }

    /// Removes one character from the active filter query.
    pub fn pop_filter_character(&mut self) {
        self.filter_query.pop();
        self.focus_first_plan_row();
        self.activate_focused();
    }

    /// Closes filter mode or requests app quit when no filter is active.
    pub fn quit_or_close_filter(&mut self) -> LeftPaneActionOutcome {
        if self.filtering {
            self.filtering = false;
            return LeftPaneActionOutcome::Handled;
        }
        LeftPaneActionOutcome::Quit
    }

    /// Focuses the first visible plan row after filter changes.
    fn focus_first_plan_row(&mut self) {
        self.focused_row = self
            .visible_rows()
            .iter()
            .position(|row| matches!(row, PlanListRow::Plan { .. }))
            .unwrap_or(0);
    }
}
