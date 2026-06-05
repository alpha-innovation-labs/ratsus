use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};
use ratkit::primitives::dialog::DialogBodyRenderer;

use crate::extensions::history_modal::data::item::ConversationPickerItem;
use crate::extensions::history_modal::layout::lines::{
    conversation_picker_lines, ConversationPickerLinesConfig,
};

/// Ratkit dialog body renderer for the conversation picker results.
pub struct ConversationPickerBody {
    query: String,
    items: Vec<ConversationPickerItem>,
    selected_position: usize,
    is_filtering: bool,
    loader_tick: u64,
    dragging_session_index: Option<usize>,
}

impl ConversationPickerBody {
    /// Creates an owned dialog body renderer for the current picker snapshot.
    pub fn new(
        query: String,
        items: Vec<ConversationPickerItem>,
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

impl DialogBodyRenderer for ConversationPickerBody {
    /// Renders the filter query and visible conversation rows.
    fn render_body(&mut self, area: Rect, buf: &mut Buffer) {
        let lines = conversation_picker_lines(ConversationPickerLinesConfig {
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
