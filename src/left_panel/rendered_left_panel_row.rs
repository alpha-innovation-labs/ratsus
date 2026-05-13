use crate::left_panel::session_list_row::SessionListRow;

/// One rendered row in the left panel, including non-interactive separators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderedLeftPanelRow {
    SessionListRow {
        source_row_index: usize,
        row: SessionListRow,
    },
    Separator,
}
