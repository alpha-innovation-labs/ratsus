use std::path::Path;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::core::rendering::style::default_border_color::default_border_color;
use crate::ui::grid_layout::bundle::session_bundle_marker::SessionBundleMarker;
use crate::ui::left_panel::folder::blue_color::folder_blue_color;
use crate::ui::left_panel::folder::compact_display_name::folder_compact_display_name;
use crate::ui::left_panel::folder::display_name::folder_display_name;
use crate::ui::left_panel::folder::icon::folder_icon;
use crate::ui::left_panel::render::truncate_folder_path_to_width::truncate_folder_path_to_width;
use crate::ui::left_panel::render::truncate_text_to_width::truncate_text_to_width;
use crate::ui::left_panel::session::running_indicator::running_session_indicator;
use crate::ui::left_panel::session::title_color::session_title_color;

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
    let marker = if config.has_running_session {
        running_session_indicator(config.loader_tick).to_string()
    } else {
        folder_icon(config.is_collapsed).to_string()
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

/// Values needed to build a chat session row line.
pub struct ChatSessionRowLineConfig<'a> {
    pub title: &'a str,
    pub age: &'a str,
    pub icon: &'a str,
    pub width: u16,
    pub is_selected: bool,
    pub is_active: bool,
    pub is_running: bool,
    pub is_completed_unseen: bool,
    pub is_dragging: bool,
    pub is_toggled: bool,
    pub bundle_marker: Option<SessionBundleMarker>,
    pub tree_prefix: Option<&'a str>,
}

/// Builds one styled line for a chat session row using the shared left-panel visual design.
pub fn session_row_line(config: ChatSessionRowLineConfig<'_>) -> Line<'static> {
    let text_width = session_title_column_width(config.age, config.width);
    let prefix = format!(
        "{}{}",
        session_branch_prefix(config.bundle_marker, config.tree_prefix),
        selection_toggle_marker(config.is_toggled)
    );
    let title = truncated_session_title(&config, text_width, &prefix);
    let left_width =
        prefix.chars().count() + config.icon.chars().count() + 1 + title.chars().count();
    let padding = session_line_padding_width(left_width, config.age, config.width);
    Line::from(vec![
        Span::styled(prefix, session_decoration_style(&config)),
        Span::styled(config.icon.to_string(), session_icon_style(&config)),
        Span::raw(" "),
        Span::styled(title, session_title_style(&config)),
        Span::raw(padding),
        Span::styled(config.age.to_string(), session_age_style(&config)),
    ])
}

/// Returns the session title truncated to fit beside row decorations.
fn truncated_session_title(
    config: &ChatSessionRowLineConfig<'_>,
    text_width: usize,
    prefix: &str,
) -> String {
    let reserved_width = prefix.chars().count() + config.icon.chars().count() + 1;
    truncate_text_to_width(config.title, text_width.saturating_sub(reserved_width))
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
fn session_branch_prefix(
    bundle_marker: Option<SessionBundleMarker>,
    tree_prefix: Option<&str>,
) -> &str {
    if let Some(prefix) = tree_prefix {
        return prefix;
    }
    match bundle_marker {
        Some(SessionBundleMarker::First) => "  ┌─ ",
        Some(SessionBundleMarker::Middle) => "  ├─ ",
        Some(SessionBundleMarker::Last) => "  └─ ",
        None => "  ",
    }
}

const SPLIT_GROUP_LEFT_PADDING: &str = "  ";

/// Builds a styled split group parent row.
pub fn split_group_row_line(
    name: &str,
    child_count: usize,
    width: u16,
    is_selected: bool,
    is_active: bool,
) -> Line<'static> {
    let suffix = format!(" ({child_count})");
    let title_width = usize::from(width)
        .saturating_sub(SPLIT_GROUP_LEFT_PADDING.chars().count() + suffix.chars().count());
    let text = format!(
        "{}{}",
        SPLIT_GROUP_LEFT_PADDING,
        truncate_text_to_width(name, title_width)
    );
    let padding = usize::from(width)
        .saturating_sub(text.chars().count() + suffix.chars().count())
        .max(1);
    Line::from(vec![
        Span::styled(
            text,
            session_line_style(is_selected, is_active, false, false, false)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ".repeat(padding)),
        Span::styled(suffix, Style::default().fg(session_title_color())),
    ])
}

/// Builds a styled folder-scoped row that opens older conversations.
pub fn folder_more_row_line(is_selected: bool) -> Line<'static> {
    Line::styled("  + more", more_line_style(is_selected))
}

/// Builds the separator shown between non-interactive left-panel sections.
pub fn session_row_separator_line(width: u16, label: Option<&str>) -> Line<'static> {
    Line::styled(
        separator_text(width, label),
        Style::default().fg(default_border_color()),
    )
}

/// Returns separator text with an optional centered label.
fn separator_text(width: u16, label: Option<&str>) -> String {
    let width = usize::from(width);
    let Some(label) = label.filter(|label| !label.is_empty()) else {
        return "─".repeat(width);
    };
    let label = format!(" {label} ");
    let label_width = label.chars().count();
    if label_width >= width {
        return truncate_text_to_width(&label, width);
    }
    let side_width = width - label_width;
    let left_width = side_width / 2;
    let right_width = side_width - left_width;
    format!(
        "{}{}{}",
        "─".repeat(left_width),
        label,
        "─".repeat(right_width)
    )
}

/// Returns the left text column width after reserving room for the age column.
fn session_title_column_width(age: &str, width: u16) -> usize {
    usize::from(width).saturating_sub(age.chars().count() + 1)
}

/// Returns spacing that right-aligns the session age inside the row width.
fn session_line_padding_width(left_width: usize, age: &str, width: u16) -> String {
    let used_width = left_width + age.chars().count();
    let available_width = usize::from(width);
    let spaces = available_width.saturating_sub(used_width).max(1);
    " ".repeat(spaces)
}

/// Returns the style for a chat session row.
fn session_line_style(
    is_selected: bool,
    is_active: bool,
    is_dragging: bool,
    is_toggled: bool,
    is_completed_unseen: bool,
) -> Style {
    if is_dragging {
        return Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
    }
    if is_toggled || is_selected {
        return Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    }
    if is_active {
        return Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
    }
    if is_completed_unseen {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(session_title_color())
}

/// Returns the style for non-title session row decoration.
fn session_decoration_style(config: &ChatSessionRowLineConfig<'_>) -> Style {
    if config.is_dragging || config.is_toggled || config.is_selected || config.is_completed_unseen {
        return session_line_style(
            config.is_selected,
            config.is_active,
            config.is_dragging,
            config.is_toggled,
            config.is_completed_unseen,
        );
    }
    Style::default().fg(session_title_color())
}

/// Returns the style for a session row icon or working animation.
fn session_icon_style(config: &ChatSessionRowLineConfig<'_>) -> Style {
    if config.is_dragging || config.is_toggled || config.is_selected || config.is_completed_unseen {
        return session_line_style(
            config.is_selected,
            config.is_active,
            config.is_dragging,
            config.is_toggled,
            config.is_completed_unseen,
        );
    }
    if config.is_active && config.is_running {
        return Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(session_title_color())
}

/// Returns the style for a session row title.
fn session_title_style(config: &ChatSessionRowLineConfig<'_>) -> Style {
    session_line_style(
        config.is_selected,
        config.is_active,
        config.is_dragging,
        config.is_toggled,
        config.is_completed_unseen,
    )
}

/// Returns the age style for a chat session row.
fn session_age_style(config: &ChatSessionRowLineConfig<'_>) -> Style {
    if config.is_toggled || config.is_selected {
        return Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    }
    if config.is_completed_unseen {
        return Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(session_title_color())
}

/// Returns the style for a folder row.
fn folder_line_style(_is_selected: bool, is_dragging: bool, _is_active_expo_folder: bool) -> Style {
    if is_dragging {
        return Style::default()
            .fg(Color::Yellow)
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
