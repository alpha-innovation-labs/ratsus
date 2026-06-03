use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::plans::render::plan_lines::plan_lines;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl LeftPaneContent for PlanListState {
    /// Handles one semantic left-pane action against plan-list state.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        match action {
            LeftPaneAction::MoveBy(direction) => self.move_by(direction),
            LeftPaneAction::FocusFirst => self.focus_first(),
            LeftPaneAction::FocusLast => self.focus_last(),
            LeftPaneAction::FocusVisibleRow(row_index) => self.focus_visible_row(row_index),
            LeftPaneAction::Activate => self.activate_focused(),
            LeftPaneAction::Collapse => self.collapse_focused_folder(),
            LeftPaneAction::Expand => self.expand_focused_folder(),
            LeftPaneAction::StartFilter => self.start_filtering(),
            LeftPaneAction::InsertFilterCharacter(character) => {
                self.push_filter_character(character)
            }
            LeftPaneAction::DeleteFilterCharacter => self.pop_filter_character(),
            LeftPaneAction::ReorderBy(direction) => self.reorder_focused_by(direction),
            LeftPaneAction::Quit => return self.quit_or_close_filter(),
            LeftPaneAction::FocusAdjacentGroup(_)
            | LeftPaneAction::Delete
            | LeftPaneAction::ToggleSelection => return LeftPaneActionOutcome::Continue,
        }
        LeftPaneActionOutcome::Handled
    }

    /// Returns the plan-list title for fallback shell rendering.
    fn title(&self) -> String {
        " Plans ".to_string()
    }

    /// Records the plan-list body area for rendering and hit testing.
    fn prepare_body_area(&mut self, area: Rect) {
        self.set_area(area);
    }

    /// Renders Markdown plan rows inside the shared left-pane body area.
    fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(plan_lines(self)), area);
    }

    /// Returns plan-list footer shortcuts for the shared left-pane shell.
    fn footer_items(&self) -> Vec<LeftPaneFooterItem> {
        vec![
            LeftPaneFooterItem::new("j/k", "move"),
            LeftPaneFooterItem::new("gg/G", "edge"),
            LeftPaneFooterItem::new("/", "filter"),
            LeftPaneFooterItem::new("S-Up", "raise"),
            LeftPaneFooterItem::new("S-Down", "lower"),
            LeftPaneFooterItem::new("enter", "open"),
        ]
    }

    /// Returns active filter text while plan filtering is enabled.
    fn footer_status(&self) -> Option<String> {
        self.filtering
            .then(|| format!("filter: {}", self.filter_query))
    }

    /// Returns whether the plan list is accepting filter text.
    fn is_filtering(&self) -> bool {
        self.filtering
    }

    /// Returns whether the plan list is waiting for a second `g`.
    fn has_pending_g(&self) -> bool {
        self.pending_g
    }

    /// Updates whether the plan list is waiting for a second `g`.
    fn set_pending_g(&mut self, pending: bool) {
        self.pending_g = pending;
    }
}
