use ratatui::{layout::Rect, Frame};

use crate::extensions::file_viewer::tree::render_view::render_file_system_tree_view;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl LeftPaneContent for FileSystemTreeView {
    /// Handles one semantic left-pane action against the file tree.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        let outcome = match action {
            LeftPaneAction::MoveBy(direction) => {
                self.move_by(direction);
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusFirst => {
                self.focus_first();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusLast => {
                self.focus_last();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusAdjacentGroup(_) => LeftPaneActionOutcome::Continue,
            LeftPaneAction::Collapse => {
                self.collapse_selected();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::Expand | LeftPaneAction::Activate => {
                self.expand_or_enter_child();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::StartFilter => {
                self.start_filtering();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::InsertFilterCharacter(character) => {
                self.push_filter_character(character);
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::DeleteFilterCharacter => {
                self.pop_filter_character();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::Quit => self.quit_or_close_filter(),
            LeftPaneAction::Delete
            | LeftPaneAction::ToggleSelection
            | LeftPaneAction::ReorderBy(_) => LeftPaneActionOutcome::Continue,
        };
        self.refresh_selection(outcome)
    }

    /// Returns the file-tree title for the shared left-pane shell.
    fn title(&self) -> String {
        " files ".to_string()
    }

    /// Records the file-tree body area for rendering and hit testing.
    fn prepare_body_area(&mut self, area: Rect) {
        self.set_tree_area(area);
    }

    /// Renders the file tree body inside the shared left-pane body area.
    fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        render_file_system_tree_view(self, frame, area);
    }

    /// Returns file-tree footer shortcuts for the shared left-pane shell.
    fn footer_items(&self) -> Vec<LeftPaneFooterItem> {
        vec![
            LeftPaneFooterItem::new("j/k", "move"),
            LeftPaneFooterItem::new("h/l", "fold"),
            LeftPaneFooterItem::new("gg/G", "edge"),
            LeftPaneFooterItem::new("/", "filter"),
            LeftPaneFooterItem::new("enter", "open"),
            LeftPaneFooterItem::new("q", "quit"),
        ]
    }

    /// Returns the selected path status for the shared left-pane footer.
    fn footer_status(&self) -> Option<String> {
        Some(self.selected_status())
    }

    /// Returns whether the file tree is accepting filter text.
    fn is_filtering(&self) -> bool {
        FileSystemTreeView::is_filtering(self)
    }

    /// Returns whether the file tree is waiting for a second `g`.
    fn has_pending_g(&self) -> bool {
        self.pending_g()
    }

    /// Updates whether the file tree is waiting for a second `g`.
    fn set_pending_g(&mut self, pending: bool) {
        self.set_pending_g(pending);
    }
}
