use ratatui::layout::Rect;

use crate::main_pane::main_pane_tab::MainPaneTab;

/// Returns the main-pane tab under a mouse position in the pane title row.
pub fn main_pane_tab_at_position(area: Rect, column: u16, row: u16) -> Option<MainPaneTab> {
    if row != area.y || column <= area.x || column >= area.x.saturating_add(area.width) {
        return None;
    }

    let title_start = area.x.saturating_add(1);
    if is_inside_label(column, title_start, "Chat") {
        return Some(MainPaneTab::Chat);
    }
    None
}

/// Returns true when a column is inside a title label range.
fn is_inside_label(column: u16, start: u16, label: &str) -> bool {
    let end = start.saturating_add(label.len() as u16);
    column >= start && column < end
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::main_pane_tab_at_position;
    use crate::main_pane::main_pane_tab::MainPaneTab;

    /// Verifies that the Chat label can be clicked in the pane title row.
    #[test]
    fn returns_chat_for_chat_label_position() {
        assert_eq!(
            main_pane_tab_at_position(Rect::new(10, 3, 80, 20), 12, 3),
            Some(MainPaneTab::Chat)
        );
    }

    /// Verifies that the hidden Files label is no longer clickable.
    #[test]
    fn ignores_files_label_position() {
        assert_eq!(
            main_pane_tab_at_position(Rect::new(10, 3, 80, 20), 19, 3),
            None
        );
    }

    /// Verifies that the hidden Diff label is no longer clickable.
    #[test]
    fn ignores_diff_label_position() {
        assert_eq!(
            main_pane_tab_at_position(Rect::new(10, 3, 80, 20), 27, 3),
            None
        );
    }

    /// Verifies that clicks outside the title row are ignored.
    #[test]
    fn ignores_positions_outside_title_row() {
        assert_eq!(
            main_pane_tab_at_position(Rect::new(10, 3, 80, 20), 12, 4),
            None
        );
    }
}
