use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::app::deletion::open_delete_session_confirmation::open_delete_session_confirmation;
use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::actions::open_in_filter_mode::open_conversation_picker_in_filter_mode;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::focus::focus_adjacent_folder::focus_adjacent_folder;
use crate::ui::left_panel::focus::focus_end::focus_left_panel_end;
use crate::ui::left_panel::focus::focus_start::focus_left_panel_start;
use crate::ui::left_panel::folder::collapse_focused_project::collapse_focused_project;
use crate::ui::left_panel::folder::open_focused_project::open_focused_project;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::input::activate_focused_row::activate_focused_left_row;
use crate::ui::left_panel::input::toggle_focused_selection::toggle_focused_left_conversation_selection;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;
use crate::ui::left_panel::render::render_scrollbar::render_left_panel_scrollbar;
use crate::ui::left_panel::render::session_lines::session_lines;
use crate::ui::left_panel::session::focus_current_workspace_session_item::focus_current_workspace_session_item;

/// Adapts the left session pane to the shared left-pane content contract.
pub struct LeftPanelKeyBehavior<'a> {
    pub(crate) app: &'a mut AppState,
}

impl<'a> LeftPanelKeyBehavior<'a> {
    /// Creates a shared-key adapter for the left session pane.
    pub fn new(app: &'a mut AppState) -> Self {
        Self { app }
    }
}

impl LeftPaneContent for LeftPanelKeyBehavior<'_> {
    /// Handles one semantic action against chat/session left-pane state.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        match action {
            LeftPaneAction::MoveBy(direction) => self.app.select_relative_session(direction),
            LeftPaneAction::FocusFirst => focus_left_panel_start(self.app),
            LeftPaneAction::FocusLast => focus_left_panel_end(self.app),
            LeftPaneAction::FocusVisibleRow(row_index) => {
                focus_current_workspace_session_item(self.app, row_index)
            }
            LeftPaneAction::FocusAdjacentGroup(direction) => {
                focus_adjacent_folder(self.app, direction)
            }
            LeftPaneAction::Activate => activate_focused_left_row(self.app),
            LeftPaneAction::Collapse => collapse_focused_project(self.app),
            LeftPaneAction::Expand => open_focused_project(self.app),
            LeftPaneAction::Delete => open_delete_session_confirmation(self.app),
            LeftPaneAction::ToggleSelection => toggle_focused_left_conversation_selection(self.app),
            LeftPaneAction::StartFilter => open_conversation_picker_in_filter_mode(self.app),
            LeftPaneAction::Quit => return LeftPaneActionOutcome::Quit,
            LeftPaneAction::ReorderBy(_)
            | LeftPaneAction::InsertFilterCharacter(_)
            | LeftPaneAction::DeleteFilterCharacter => return LeftPaneActionOutcome::Continue,
        }
        LeftPaneActionOutcome::Handled
    }

    /// Returns the chat/session title for the shared left-pane shell.
    fn title(&self) -> String {
        " Sessions ".to_string()
    }

    /// Records the chat/session body area for scrolling and hit testing.
    fn prepare_body_area(&mut self, area: Rect) {
        self.app.last_session_list_area = area;
    }

    /// Renders chat/session rows inside the shared left-pane body area.
    fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(session_lines(self.app)), area);
        render_left_panel_scrollbar(self.app, frame);
    }

    /// Returns chat/session footer shortcuts for the shared left-pane shell.
    fn footer_items(&self) -> Vec<LeftPaneFooterItem> {
        vec![
            LeftPaneFooterItem::new("j/k", "move"),
            LeftPaneFooterItem::new("h/l", "fold"),
            LeftPaneFooterItem::new("gg/G", "edge"),
            LeftPaneFooterItem::new("/", "find"),
            LeftPaneFooterItem::new("Space", "select"),
            LeftPaneFooterItem::new("enter", "open"),
        ]
    }

    /// Returns whether the left session pane is waiting for a second `g`.
    fn has_pending_g(&self) -> bool {
        self.app.pending_left_g
    }

    /// Updates whether the left session pane is waiting for a second `g`.
    fn set_pending_g(&mut self, pending: bool) {
        self.app.pending_left_g = pending;
    }
}

#[cfg(test)]
mod tests {
    use super::LeftPanelKeyBehavior;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::left_panel::action::LeftPaneAction;
    use crate::ui::left_panel::content::LeftPaneContent;

    /// Verifies session-pane slash opens the conversation picker already filtering.
    #[test]
    fn start_filter_opens_picker_in_filter_mode() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/workspace/alpha")])?;
        let mut behavior = LeftPanelKeyBehavior::new(&mut app);

        behavior.handle_left_pane_action(LeftPaneAction::StartFilter);

        assert!(behavior.app.conversation_picker.is_open);
        assert!(behavior.app.conversation_picker.is_filtering);
        Ok(())
    }
}
