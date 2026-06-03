use ratatui::layout::Rect;

use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Stores the current render body area for hit testing and scrolling.
    pub fn set_area(&mut self, area: Rect) {
        self.last_area = area;
    }

    /// Scrolls the plan viewport without changing focused plan.
    pub fn scroll_by(&mut self, delta: isize) -> bool {
        let before = self.scroll;
        let max_scroll = self
            .visible_rows()
            .len()
            .saturating_sub(usize::from(self.last_area.height).max(1));
        self.scroll = self.scroll.saturating_add_signed(delta).min(max_scroll);
        self.scroll != before
    }

    /// Keeps focused row visible in the current viewport.
    pub fn keep_focused_visible(&mut self) {
        let height = usize::from(self.last_area.height).max(1);
        if self.focused_row < self.scroll {
            self.scroll = self.focused_row;
        } else if self.focused_row >= self.scroll + height {
            self.scroll = self.focused_row + 1 - height;
        }
    }
}
