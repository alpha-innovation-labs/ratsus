use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};
use ratkit::primitives::dialog::DialogBodyRenderer;

use crate::extensions::history_modal::data::item::ConversationPickerItem;
use crate::extensions::history_modal::layout::lines::{
    conversation_picker_lines, ConversationPickerLinesConfig,
};
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Ratkit dialog body renderer for the conversation picker results.
pub struct ConversationPickerBody<'a> {
    query: String,
    items: Vec<ConversationPickerItem>,
    sessions: &'a [SessionTerminal],
    selected_position: usize,
    is_filtering: bool,
    loader_tick: u64,
    dragging_session_index: Option<usize>,
}

impl<'a> ConversationPickerBody<'a> {
    /// Creates an owned dialog body renderer for the current picker snapshot.
    pub fn new(
        query: String,
        items: Vec<ConversationPickerItem>,
        sessions: &'a [SessionTerminal],
        selected_position: usize,
        is_filtering: bool,
        loader_tick: u64,
        dragging_session_index: Option<usize>,
    ) -> Self {
        Self {
            query,
            items,
            sessions,
            selected_position,
            is_filtering,
            loader_tick,
            dragging_session_index,
        }
    }
}

impl<'a> DialogBodyRenderer for ConversationPickerBody<'a> {
    /// Renders the filter query and visible conversation rows.
    fn render_body(&mut self, area: Rect, buf: &mut Buffer) {
        let lines = conversation_picker_lines(ConversationPickerLinesConfig {
            query: &self.query,
            items: &self.items,
            sessions: self.sessions,
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
