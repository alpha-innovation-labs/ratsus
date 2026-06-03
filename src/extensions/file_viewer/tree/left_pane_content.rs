use ratatui::{layout::Rect, Frame};

use crate::extensions::file_viewer::tree::render_view::render_file_system_tree_view;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl LeftPaneContent for FileSystemTreeView {
    /// Handles one semantic left-pane action against the grouped file tree.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        match action {
            LeftPaneAction::MoveBy(direction) => {
                self.move_workspace_by(direction);
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusFirst => {
                self.focus_workspace_first();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusLast => {
                self.focus_workspace_last();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusVisibleRow(row_index) => {
                self.focus_current_workspace_file_item(row_index);
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::FocusAdjacentGroup(_) => LeftPaneActionOutcome::Continue,
            LeftPaneAction::Collapse => {
                self.collapse_workspace_selected();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::Expand | LeftPaneAction::Activate => {
                self.expand_or_enter_workspace_child();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::StartFilter => {
                self.start_workspace_filtering();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::InsertFilterCharacter(character) => {
                self.push_workspace_filter_character(character);
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::DeleteFilterCharacter => {
                self.pop_workspace_filter_character();
                LeftPaneActionOutcome::Handled
            }
            LeftPaneAction::Quit => self.quit_or_close_workspace_filter(),
            LeftPaneAction::Delete
            | LeftPaneAction::ToggleSelection
            | LeftPaneAction::ReorderBy(_) => LeftPaneActionOutcome::Continue,
        }
    }

    /// Returns the file-tree title for the shared left-pane shell.
    fn title(&self) -> String {
        " files ".to_string()
    }

    /// Records the file-tree body area for rendering and hit testing.
    fn prepare_body_area(&mut self, area: Rect) {
        self.set_tree_area(area);
    }

    /// Renders the grouped workspace file tree inside the shared left-pane body area.
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
        Some(self.workspace_selected_status())
    }

    /// Returns whether the grouped file tree is accepting filter text.
    fn is_filtering(&self) -> bool {
        self.workspace_filtering
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
