use std::path::Path;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::left_panel::folder_blue_color::folder_blue_color;
use crate::left_panel::folder_compact_display_name::folder_compact_display_name;
use crate::left_panel::folder_display_name::folder_display_name;
use crate::left_panel::folder_icon::folder_icon;
use crate::left_panel::running_session_indicator::running_session_indicator;
use crate::left_panel::session_title_color::session_title_color;
use crate::left_panel::truncate_folder_path_to_width::truncate_folder_path_to_width;
use crate::left_panel::truncate_text_to_width::truncate_text_to_width;
use crate::rendering::default_border_color::default_border_color;
use crate::rendering::left_focused_border_color::left_focused_border_color;
use crate::session_panes::session_bundle_marker::SessionBundleMarker;

/// Values needed to build a folder row line.
pub struct FolderRowLineConfig<'a> {
    pub path: &'a Path,
    pub current_session_count: usize,
    pub total_session_count: usize,
    pub is_collapsed: bool,
    pub has_running_session: bool,
    pub loader_tick: u64,
    pub width: u16,
    pub show_full_path: bool,
    pub is_active_expo_folder: bool,
    pub is_selected: bool,
    pub is_dragging: bool,
}

/// Builds one styled line for a folder row using the shared left-panel visual design.
pub fn folder_row_line(config: FolderRowLineConfig<'_>) -> Line<'static> {
    let icon = folder_icon(config.is_collapsed);
    let marker = if config.has_running_session {
        format!("{} {}", icon, running_session_indicator(config.loader_tick))
    } else {
        icon.to_string()
    };
    let suffix = format!(
        " ({}/{})",
        config.current_session_count, config.total_session_count
    );
    let visible_path = visible_folder_path(&config, &marker, &suffix);
    let text = format!("{} {}{}", marker, visible_path, suffix);
    Line::styled(
        text,
        folder_line_style(
            config.is_selected,
            config.is_dragging,
            config.is_active_expo_folder,
        ),
    )
}

/// Returns the visible folder path for the configured row style.
fn visible_folder_path(config: &FolderRowLineConfig<'_>, marker: &str, suffix: &str) -> String {
    if !config.show_full_path {
        return folder_compact_display_name(config.path);
    }
    let folder_path = folder_display_name(config.path);
    let path_width = folder_path_width(config.width, marker, suffix);
    truncate_folder_path_to_width(&folder_path, path_width)
}

/// Returns the folder path width after reserving marker, spacing, and count suffix.
fn folder_path_width(width: u16, marker: &str, suffix: &str) -> usize {
    let reserved = marker.chars().count() + 1 + suffix.chars().count();
    usize::from(width).saturating_sub(reserved)
}

/// Values needed to build a Nexus session row line.
pub struct NexusSessionRowLineConfig<'a> {
    pub title: &'a str,
    pub age: &'a str,
    pub icon: &'a str,
    pub width: u16,
    pub is_selected: bool,
    pub is_active: bool,
    pub is_dragging: bool,
    pub is_toggled: bool,
    pub bundle_marker: Option<SessionBundleMarker>,
}

/// Builds one styled line for a Nexus session row using the shared left-panel visual design.
pub fn nexus_session_row_line(config: NexusSessionRowLineConfig<'_>) -> Line<'static> {
    let text_width = session_title_column_width(config.age, config.width);
    let text = truncate_text_to_width(
        &format!(
            "{}{}{} {}",
            session_branch_prefix(config.bundle_marker),
            selection_toggle_marker(config.is_toggled),
            config.icon,
            config.title
        ),
        text_width,
    );
    let padding = session_line_padding(&text, config.age, config.width);
    Line::from(vec![
        Span::styled(
            text,
            session_line_style(config.is_selected, config.is_active, config.is_dragging),
        ),
        Span::raw(padding),
        Span::styled(
            config.age.to_string(),
            Style::default().fg(session_title_color()),
        ),
    ])
}

/// Returns the marker for conversations selected for future bulk actions.
fn selection_toggle_marker(is_toggled: bool) -> &'static str {
    if is_toggled {
        "☑ "
    } else {
        ""
    }
}

/// Returns the branch prefix for bundled sessions in the left session tree.
fn session_branch_prefix(bundle_marker: Option<SessionBundleMarker>) -> &'static str {
    match bundle_marker {
        Some(SessionBundleMarker::First) => "  ┌─ ",
        Some(SessionBundleMarker::Middle) => "  ├─ ",
        Some(SessionBundleMarker::Last) => "  └─ ",
        None => "  ",
    }
}

/// Builds a styled folder-scoped row that opens older conversations.
pub fn folder_more_row_line(is_selected: bool) -> Line<'static> {
    Line::styled("  + more", more_line_style(is_selected))
}

/// Builds the separator shown below pinned active rows.
pub fn session_row_separator_line(width: u16) -> Line<'static> {
    Line::styled(
        "─".repeat(usize::from(width)),
        Style::default().fg(default_border_color()),
    )
}

/// Returns the left text column width after reserving room for the age column.
fn session_title_column_width(age: &str, width: u16) -> usize {
    usize::from(width).saturating_sub(age.chars().count() + 1)
}

/// Returns spacing that right-aligns the session age inside the row width.
fn session_line_padding(text: &str, age: &str, width: u16) -> String {
    let used_width = text.chars().count() + age.chars().count();
    let available_width = usize::from(width);
    let spaces = available_width.saturating_sub(used_width).max(1);
    " ".repeat(spaces)
}

/// Returns the style for a Nexus session row.
fn session_line_style(is_selected: bool, is_active: bool, is_dragging: bool) -> Style {
    if is_dragging {
        return Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
    }
    if is_selected {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    if is_active {
        return Style::default()
            .fg(left_focused_border_color())
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(session_title_color())
}

/// Returns the style for a folder row.
fn folder_line_style(is_selected: bool, is_dragging: bool, is_active_expo_folder: bool) -> Style {
    if is_dragging {
        return Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
    }
    if is_active_expo_folder {
        return Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    }
    if is_selected {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default()
        .fg(folder_blue_color())
        .add_modifier(Modifier::BOLD)
}

/// Returns the style for a folder-scoped more row.
fn more_line_style(is_selected: bool) -> Style {
    if is_selected {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(Color::Magenta)
}
