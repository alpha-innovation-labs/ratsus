use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::extensions::command_bar::data::command_item::CommandBarItem;
use crate::extensions::command_bar::layout::start_index::command_bar_start_index;
use crate::ui::left_panel::render::truncate_text_to_width::truncate_text_to_width;
use crate::ui::left_panel::session::title_color::session_title_color;

/// Values needed to build command bar body lines.
pub struct CommandBarLinesConfig<'a> {
    pub query: &'a str,
    pub items: &'a [CommandBarItem],
    pub selected_position: usize,
    pub is_filtering: bool,
    pub height: u16,
    pub width: u16,
}

/// Builds styled command bar body lines for the current query and selection.
pub fn command_bar_lines(config: CommandBarLinesConfig<'_>) -> Vec<Line<'static>> {
    if config.height == 0 {
        return Vec::new();
    }
    let mut lines = command_bar_header_lines(config.query, config.is_filtering);
    let list_height = usize::from(config.height).saturating_sub(lines.len());
    lines.extend(command_bar_result_lines(
        config.items,
        config.selected_position,
        list_height,
        config.width,
    ));
    while lines.len() < usize::from(config.height) {
        lines.push(Line::from(""));
    }
    lines
}

/// Builds static header lines for the command bar body.
fn command_bar_header_lines(query: &str, is_filtering: bool) -> Vec<Line<'static>> {
    let label = if is_filtering {
        format!("Filter commands: {}", query)
    } else {
        "Press / to filter commands".to_string()
    };
    vec![
        Line::styled(
            label,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
    ]
}

/// Builds visible command rows for the command bar body.
fn command_bar_result_lines(
    items: &[CommandBarItem],
    selected_position: usize,
    height: usize,
    width: u16,
) -> Vec<Line<'static>> {
    if height == 0 {
        return Vec::new();
    }
    if items.is_empty() {
        return vec![Line::styled(
            "No commands match this filter",
            Style::default().fg(Color::DarkGray),
        )];
    }
    let start = command_bar_start_index(selected_position, items.len(), height);
    let end = start.saturating_add(height).min(items.len());
    items[start..end]
        .iter()
        .enumerate()
        .map(|(position, item)| {
            command_bar_line(item, start + position == selected_position, width)
        })
        .collect()
}

/// Builds one visible command row using the same list color contract as search results.
fn command_bar_line(item: &CommandBarItem, is_selected: bool, width: u16) -> Line<'static> {
    let title_style = command_title_style(is_selected);
    let hotkey_style = command_hotkey_style(is_selected);
    let title = truncate_text_to_width(item.title, command_title_width(item.hotkey, width));
    let padding = command_row_padding(&title, item.hotkey, width);
    Line::from(vec![
        Span::styled("  ".to_string(), title_style),
        Span::styled(title, title_style),
        Span::raw(padding),
        Span::styled(item.hotkey.to_string(), hotkey_style),
    ])
}

/// Returns the title column width after reserving room for the hotkey column.
fn command_title_width(hotkey: &str, width: u16) -> usize {
    usize::from(width).saturating_sub(hotkey.chars().count() + 3)
}

/// Returns spacing that right-aligns the hotkey inside the row width.
fn command_row_padding(title: &str, hotkey: &str, width: u16) -> String {
    let used_width = 2 + title.chars().count() + hotkey.chars().count();
    " ".repeat(usize::from(width).saturating_sub(used_width).max(1))
}

/// Returns the command title style matching selected search rows.
fn command_title_style(is_selected: bool) -> Style {
    if is_selected {
        return Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    }
    Style::default().fg(session_title_color())
}

/// Returns the command hotkey style matching muted search metadata.
fn command_hotkey_style(is_selected: bool) -> Style {
    if is_selected {
        return Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    }
    Style::default().fg(Color::DarkGray)
}

#[cfg(test)]
mod tests {
    use super::{command_bar_lines, CommandBarLinesConfig};
    use crate::extensions::command_bar::data::filtered_commands::command_bar_items;

    /// Verifies command rows render titles and hotkeys without descriptions.
    #[test]
    fn renders_command_titles_and_hotkeys() {
        let items = command_bar_items();
        let lines = command_bar_lines(CommandBarLinesConfig {
            query: "",
            items: &items,
            selected_position: 0,
            is_filtering: false,
            height: 4,
            width: 60,
        });

        let rendered = format!("{:?}", lines);
        assert!(rendered.contains("Chat history"));
        assert!(rendered.contains("Ctrl+H"));
        assert!(!rendered.contains("Open the conversation picker"));
    }
}
