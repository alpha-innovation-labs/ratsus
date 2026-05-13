use ratatui::layout::Rect;

use crate::left_panel::rendered_left_panel_row::RenderedLeftPanelRow;
use crate::left_panel::session_list_row::SessionListRow;

/// Converts a mouse row into a rendered interactive session-list row.
pub fn session_row_for_rendered_click(
    row: u16,
    list_area: Rect,
    rendered_rows: &[RenderedLeftPanelRow],
) -> Option<(usize, SessionListRow)> {
    if row < list_area.y || row >= list_area.y.saturating_add(list_area.height) {
        return None;
    }
    let rendered_index = usize::from(row - list_area.y);
    match rendered_rows.get(rendered_index)? {
        RenderedLeftPanelRow::SessionListRow {
            source_row_index,
            row,
        } => Some((*source_row_index, row.clone())),
        RenderedLeftPanelRow::Separator => None,
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::session_row_for_rendered_click;
    use crate::left_panel::rendered_left_panel_row::RenderedLeftPanelRow;
    use crate::left_panel::session_list_row::SessionListRow;

    /// Verifies pinned rendered rows map to their source row, not scroll offset rows.
    #[test]
    fn maps_rendered_pinned_row_to_source_row() {
        let rendered = vec![RenderedLeftPanelRow::SessionListRow {
            source_row_index: 10,
            row: SessionListRow::Session { index: 4 },
        }];

        assert_eq!(
            session_row_for_rendered_click(2, Rect::new(0, 2, 20, 5), &rendered),
            Some((10, SessionListRow::Session { index: 4 }))
        );
    }

    /// Verifies separator rows are non-interactive.
    #[test]
    fn ignores_separator_rows() {
        let rendered = vec![RenderedLeftPanelRow::Separator];

        assert_eq!(
            session_row_for_rendered_click(2, Rect::new(0, 2, 20, 5), &rendered),
            None
        );
    }
}
