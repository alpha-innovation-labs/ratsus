use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::conversation_picker::conversation_picker_item::ConversationPickerItem;
use crate::conversation_picker::conversation_picker_start_index::conversation_picker_start_index;

/// Builds styled picker body lines for the current query and filtered items.
pub fn conversation_picker_lines(
    query: &str,
    items: &[ConversationPickerItem],
    selected_position: usize,
    height: u16,
) -> Vec<Line<'static>> {
    let mut lines = picker_header_lines(query);
    if items.is_empty() {
        lines.push(Line::styled(
            "No conversations match this filter",
            Style::default().fg(Color::DarkGray),
        ));
        return lines;
    }

    let visible_height = usize::from(height).saturating_sub(lines.len());
    let start = conversation_picker_start_index(selected_position, items.len(), visible_height);
    let end = start.saturating_add(visible_height).min(items.len());

    for (position, item) in items[start..end].iter().enumerate() {
        let index = start + position;
        lines.push(conversation_picker_line(item, index == selected_position));
    }
    lines
}

/// Builds static header and help lines for the picker body.
fn picker_header_lines(query: &str) -> Vec<Line<'static>> {
    vec![
        Line::styled(
            format!("Filter: {}", query),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Line::styled(
            "Type to filter · ↑/↓ or Ctrl+n/p navigate · Enter open · Esc close",
            Style::default().fg(Color::DarkGray),
        ),
        Line::from(""),
    ]
}

/// Builds one visible picker row.
fn conversation_picker_line(item: &ConversationPickerItem, is_selected: bool) -> Line<'static> {
    let prefix = if is_selected { "›" } else { " " };
    let text = if item.is_folder() {
        format!("{} 📁 {}", prefix, item.title)
    } else {
        let marker = if item.is_active { "●" } else { " " };
        format!("{}   {} {} — {}", prefix, marker, item.title, item.subtitle)
    };
    Line::styled(text, conversation_picker_row_style(item, is_selected))
}

/// Returns the style for one picker result row.
fn conversation_picker_row_style(item: &ConversationPickerItem, is_selected: bool) -> Style {
    if is_selected {
        return Style::default().fg(Color::Black).bg(Color::Cyan);
    }
    if item.is_folder() {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(Color::White)
}
