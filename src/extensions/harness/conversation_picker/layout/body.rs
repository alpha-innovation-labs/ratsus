use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};
use ratkit::primitives::dialog::DialogBodyRenderer;

use crate::extensions::history_modal::data::item::HistoryModalItem;
use crate::extensions::history_modal::layout::lines::{
    history_modal_lines, HistoryModalLinesConfig,
};

/// Ratkit dialog body renderer for the conversation picker results.
pub struct HistoryModalBody {
    query: String,
    items: Vec<HistoryModalItem>,
    selected_position: usize,
    is_filtering: bool,
    loader_tick: u64,
    dragging_session_index: Option<usize>,
}

impl HistoryModalBody {
    /// Creates an owned dialog body renderer for the current picker snapshot.
    pub fn new(
        query: String,
        items: Vec<HistoryModalItem>,
        selected_position: usize,
        is_filtering: bool,
        loader_tick: u64,
        dragging_session_index: Option<usize>,
    ) -> Self {
        Self {
            query,
            items,
            selected_position,
            is_filtering,
            loader_tick,
            dragging_session_index,
        }
    }
}

impl DialogBodyRenderer for HistoryModalBody {
    /// Renders the filter query and visible conversation rows.
    fn render_body(&mut self, area: Rect, buf: &mut Buffer) {
        let lines = history_modal_lines(HistoryModalLinesConfig {
            query: &self.query,
            items: &self.items,
            selected_position: self.selected_position,
            is_filtering: self.is_filtering,
            height: area.height,
            width: area.width,
            loader_tick: self.loader_tick,
            dragging_session_index: self.dragging_session_index,
        });
        Paragraph::new(lines).render(area, buf);
    }
}
