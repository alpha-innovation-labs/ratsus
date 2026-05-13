use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::conversation_picker::conversation_picker_item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::conversation_picker::conversation_picker_start_index::conversation_picker_start_index;
use crate::left_panel::running_session_indicator::running_session_indicator;
use crate::left_panel::session_row_line::{
    folder_row_line, session_row_line, ChatSessionRowLineConfig, FolderRowLineConfig,
};

/// Values needed to build conversation picker body lines.
pub struct ConversationPickerLinesConfig<'a> {
    pub query: &'a str,
    pub items: &'a [ConversationPickerItem],
    pub selected_position: usize,
    pub is_filtering: bool,
    pub height: u16,
    pub width: u16,
    pub loader_tick: u64,
    pub dragging_session_index: Option<usize>,
}

/// Builds styled picker body lines for the current query and filtered items.
pub fn conversation_picker_lines(config: ConversationPickerLinesConfig<'_>) -> Vec<Line<'static>> {
    if config.height == 0 {
        return Vec::new();
    }
    if config.height == 1 {
        return vec![picker_footer_line()];
    }

    let mut lines = picker_header_lines(config.query, config.is_filtering);
    let list_height = usize::from(config.height).saturating_sub(lines.len() + 1);
    lines.extend(picker_result_lines(
        config.items,
        config.selected_position,
        list_height,
        config.width,
        config.loader_tick,
        config.dragging_session_index,
    ));
    while lines.len() + 1 < usize::from(config.height) {
        lines.push(Line::from(""));
    }
    lines.push(picker_footer_line());
    lines
}

/// Builds static header lines for the picker body.
fn picker_header_lines(query: &str, is_filtering: bool) -> Vec<Line<'static>> {
    let label = if is_filtering {
        format!("Filter: {}", query)
    } else {
        "Press / to filter".to_string()
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

/// Builds the bottom-aligned keyboard help line for the picker body.
fn picker_footer_line() -> Line<'static> {
    Line::styled(
        "/ filter · ↑/↓ scroll · Shift+↑/↓ reorder · Space select · Enter open · Esc close",
        Style::default().fg(Color::DarkGray),
    )
}

/// Builds the visible result lines between the header and bottom hotkeys.
fn picker_result_lines(
    items: &[ConversationPickerItem],
    selected_position: usize,
    height: usize,
    width: u16,
    loader_tick: u64,
    dragging_session_index: Option<usize>,
) -> Vec<Line<'static>> {
    if height == 0 {
        return Vec::new();
    }
    if items.is_empty() {
        return vec![Line::styled(
            "No conversations match this filter",
            Style::default().fg(Color::DarkGray),
        )];
    }

    let start = conversation_picker_start_index(selected_position, items.len(), height);
    let end = start.saturating_add(height).min(items.len());
    items[start..end]
        .iter()
        .enumerate()
        .map(|(position, item)| {
            let index = start + position;
            conversation_picker_line(
                item,
                index == selected_position,
                width,
                loader_tick,
                dragging_session_index,
            )
        })
        .collect()
}

/// Builds one visible picker row with the same row renderer used by the left pane.
fn conversation_picker_line(
    item: &ConversationPickerItem,
    is_selected: bool,
    width: u16,
    loader_tick: u64,
    dragging_session_index: Option<usize>,
) -> Line<'static> {
    match &item.kind {
        ConversationPickerItemKind::Folder {
            path,
            current_session_count,
            total_session_count,
            is_collapsed,
            has_running_session,
        } => folder_row_line(FolderRowLineConfig {
            path,
            current_session_count: *current_session_count,
            total_session_count: *total_session_count,
            is_collapsed: *is_collapsed,
            has_running_session: *has_running_session,
            loader_tick,
            width,
            show_full_path: true,
            is_active_expo_folder: false,
            is_selected,
            is_dragging: false,
        }),
        ConversationPickerItemKind::Session {
            index,
            age,
            icon,
            is_running,
            ..
        } => {
            let visible_icon = if *is_running {
                running_session_indicator(loader_tick)
            } else {
                icon.as_str()
            };
            session_row_line(ChatSessionRowLineConfig {
                title: &item.title,
                age,
                icon: visible_icon,
                width,
                is_selected,
                is_active: item.is_active,
                is_dragging: dragging_session_index.is_some_and(|drag_index| drag_index == *index),
                is_toggled: item.is_toggled,
                bundle_marker: None,
            })
        }
    }
}
