use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};
use ratkit::primitives::dialog::DialogBodyRenderer;

use crate::conversation_picker::conversation_picker_item::ConversationPickerItem;
use crate::conversation_picker::conversation_picker_lines::conversation_picker_lines;

/// Ratkit dialog body renderer for the conversation picker results.
pub struct ConversationPickerBody {
    query: String,
    items: Vec<ConversationPickerItem>,
    selected_position: usize,
}

impl ConversationPickerBody {
    /// Creates an owned dialog body renderer for the current picker snapshot.
    pub fn new(
        query: String,
        items: Vec<ConversationPickerItem>,
        selected_position: usize,
    ) -> Self {
        Self {
            query,
            items,
            selected_position,
        }
    }
}

impl DialogBodyRenderer for ConversationPickerBody {
    /// Renders the filter query, keyboard help, and visible conversation rows.
    fn render_body(&mut self, area: Rect, buf: &mut Buffer) {
        let lines = conversation_picker_lines(
            &self.query,
            &self.items,
            self.selected_position,
            area.height,
        );
        Paragraph::new(lines).render(area, buf);
    }
}
