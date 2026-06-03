use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;

impl PlanListState {
    /// Moves focus by a signed visible-row delta and activates plans when focused.
    pub fn move_by(&mut self, direction: isize) {
        let rows = self.visible_rows();
        if rows.is_empty() {
            self.focused_row = 0;
            self.active_index = None;
            self.preview_state = None;
            return;
        }
        self.focused_row = wrapped_list_position(self.focused_row, direction, rows.len());
        self.activate_focused();
    }

    /// Moves focus to the first visible plan row.
    pub fn focus_first(&mut self) {
        self.focused_row = 0;
        self.activate_focused();
    }

    /// Moves focus to the last visible plan row.
    pub fn focus_last(&mut self) {
        let rows = self.visible_rows();
        self.focused_row = rows.len().saturating_sub(1);
        self.activate_focused();
    }

    /// Focuses the visible row that contains one plan index.
    pub(crate) fn focus_row_for_plan_index(&mut self, plan_index: usize) {
        if let Some(row) = self
            .visible_rows()
            .iter()
            .position(|row| matches!(row, PlanListRow::Plan { index } if *index == plan_index))
        {
            self.focused_row = row;
            self.keep_focused_visible();
        }
    }
}
