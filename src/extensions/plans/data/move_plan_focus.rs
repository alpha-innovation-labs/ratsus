use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;

impl PlanListState {
    /// Moves focus by a signed visible-row delta and activates the target plan.
    pub fn move_by(&mut self, direction: isize) {
        let visible = self.visible_indices();
        if visible.is_empty() {
            self.focused_row = 0;
            self.active_index = None;
            self.preview_state = None;
            return;
        }
        self.focused_row = wrapped_list_position(self.focused_row, direction, visible.len());
        self.activate_focused();
    }

    /// Moves focus to the first visible plan.
    pub fn focus_first(&mut self) {
        self.focused_row = 0;
        self.activate_focused();
    }

    /// Moves focus to the last visible plan.
    pub fn focus_last(&mut self) {
        let visible = self.visible_indices();
        self.focused_row = visible.len().saturating_sub(1);
        self.activate_focused();
    }

    /// Focuses the visible row that contains one plan index.
    pub(crate) fn focus_row_for_plan_index(&mut self, plan_index: usize) {
        if let Some(row) = self
            .visible_indices()
            .iter()
            .position(|index| *index == plan_index)
        {
            self.focused_row = row;
            self.keep_focused_visible();
        }
    }
}
