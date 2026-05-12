use ratatui::text::Line;

use crate::main_pane::main_pane_tab::MainPaneTab;

/// Builds the main pane title with only the Chat label visible.
pub fn main_pane_title_line(_selected: MainPaneTab) -> Line<'static> {
    Line::from("Chat")
}
