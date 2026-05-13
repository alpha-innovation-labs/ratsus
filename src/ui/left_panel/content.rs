use ratatui::{layout::Rect, Frame};

use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Contract implemented by content hosted inside the shared left-pane shell.
pub trait LeftPaneContent {
    /// Handles one semantic left-pane action in the content domain.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome;

    /// Returns the title shown by the shared left-pane shell.
    fn title(&self) -> String {
        String::new()
    }

    /// Records the body area used by this content before rendering.
    fn prepare_body_area(&mut self, _area: Rect) {}

    /// Renders this content inside the shared left-pane body area.
    fn render_body(&mut self, _frame: &mut Frame, _area: Rect) {}

    /// Returns footer shortcuts for the shared left-pane footer renderer.
    fn footer_items(&self) -> Vec<LeftPaneFooterItem>;

    /// Returns optional status text shown by the shared left-pane footer.
    fn footer_status(&self) -> Option<String> {
        None
    }

    /// Returns true when typed characters should edit a content filter.
    fn is_filtering(&self) -> bool {
        false
    }

    /// Returns whether this content is waiting for a second `g` key.
    fn has_pending_g(&self) -> bool;

    /// Updates whether this content is waiting for a second `g` key.
    fn set_pending_g(&mut self, pending: bool);
}
