use ratatui::layout::Rect;

use crate::ui::left_panel::scroll::scrollbar_area::left_panel_scrollbar_area;

const LEFT_PANEL_SCROLLBAR_GAP: u16 = 1;

/// Returns the left-panel text width after reserving space and a gap for the scrollbar.
pub fn left_panel_text_width(session_list_area: Rect, total_rows: usize) -> u16 {
    let reserved_width = left_panel_scrollbar_area(session_list_area, total_rows)
        .map(|area| area.width.saturating_add(LEFT_PANEL_SCROLLBAR_GAP))
        .unwrap_or(0);
    session_list_area.width.saturating_sub(reserved_width)
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::left_panel_text_width;

    /// Verifies scrollable content reserves one column and one gap for the scrollbar.
    #[test]
    fn reserves_scrollbar_column_and_gap_when_scrollable() {
        assert_eq!(left_panel_text_width(Rect::new(1, 1, 10, 5), 12), 8);
    }

    /// Verifies content that fits can use the full left-panel width.
    #[test]
    fn keeps_full_width_when_not_scrollable() {
        assert_eq!(left_panel_text_width(Rect::new(1, 1, 10, 5), 5), 10);
    }
}
