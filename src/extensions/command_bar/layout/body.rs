use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};
use ratkit::primitives::dialog::DialogBodyRenderer;

use crate::extensions::command_bar::data::command_item::CommandBarItem;
use crate::extensions::command_bar::layout::lines::{command_bar_lines, CommandBarLinesConfig};

/// Ratkit dialog body renderer for the command bar command list.
pub struct CommandBarBody {
    query: String,
    items: Vec<CommandBarItem>,
    selected_position: usize,
    is_filtering: bool,
}

impl CommandBarBody {
    /// Creates an owned dialog body renderer for the current command bar snapshot.
    pub fn new(
        query: String,
        items: Vec<CommandBarItem>,
        selected_position: usize,
        is_filtering: bool,
    ) -> Self {
        Self {
            query,
            items,
            selected_position,
            is_filtering,
        }
    }
}

impl DialogBodyRenderer for CommandBarBody {
    /// Renders the filter query and visible command rows.
    fn render_body(&mut self, area: Rect, buf: &mut Buffer) {
        let lines = command_bar_lines(CommandBarLinesConfig {
            query: &self.query,
            items: &self.items,
            selected_position: self.selected_position,
            is_filtering: self.is_filtering,
            height: area.height,
            width: area.width,
        });
        Paragraph::new(lines).render(area, buf);
    }
}
