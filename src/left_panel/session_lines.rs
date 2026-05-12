use chrono::Utc;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::folder_display_name::folder_display_name;
use crate::left_panel::folder_has_running_session::folder_has_running_session;
use crate::left_panel::folder_icon::folder_icon;
use crate::left_panel::format_session_age::format_session_age;
use crate::left_panel::running_session_indicator::running_session_indicator;
use crate::left_panel::session_icon::session_icon;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::truncate_text_to_width::truncate_text_to_width;
use crate::left_panel::visible_session_rows::visible_session_rows;

/// Builds the visible session list lines for the left pane.
pub fn session_lines(app: &NexusDemo) -> Vec<Line<'static>> {
    if app.session_terminals.is_empty() {
        return vec![Line::from("No Nexus sessions")];
    }

    let rows = visible_session_rows(
        &app.session_terminals,
        &app.collapsed_folders,
        &app.folder_order,
    );
    let visible_height = usize::from(app.last_session_list_area.height);
    let end = (app.session_scroll + visible_height).min(rows.len());
    rows[app.session_scroll..end]
        .iter()
        .enumerate()
        .map(|(offset, row)| session_tree_line(app, row, app.session_scroll + offset))
        .collect()
}

/// Builds one styled line for a folder or session row.
fn session_tree_line(app: &NexusDemo, row: &SessionListRow, row_index: usize) -> Line<'static> {
    match row {
        SessionListRow::Folder {
            path,
            current_session_count,
            total_session_count,
        } => folder_line(
            app,
            path,
            *current_session_count,
            *total_session_count,
            row_index,
        ),
        SessionListRow::Session { index } => session_line(app, *index, row_index),
        SessionListRow::FolderMore { .. } => more_line(app, row_index),
    }
}

/// Builds one styled line for a folder row.
fn folder_line(
    app: &NexusDemo,
    path: &std::path::Path,
    current_session_count: usize,
    total_session_count: usize,
    row_index: usize,
) -> Line<'static> {
    let is_collapsed = app.collapsed_folders.contains(path);
    let icon = folder_icon(is_collapsed);
    let marker = if folder_has_running_session(path, &app.session_terminals) {
        format!("{} {}", icon, running_session_indicator(app.loader_tick))
    } else {
        icon.to_string()
    };
    let text = format!(
        "{} {} ({}/{})",
        marker,
        folder_display_name(path),
        current_session_count,
        total_session_count
    );
    Line::styled(text, folder_line_style(app, path, row_index))
}

/// Builds one styled line for a Nexus session row.
fn session_line(app: &NexusDemo, index: usize, row_index: usize) -> Line<'static> {
    let entry = &app.session_terminals[index];
    let age = format_session_age(&entry.session, Utc::now());
    let text_width = session_title_column_width(&age, app.last_session_list_area.width);
    let icon = if entry.session.is_running {
        running_session_indicator(app.loader_tick)
    } else {
        session_icon(&entry.session)
    };
    let text = truncate_text_to_width(&format!("  {} {}", icon, entry.session.title), text_width);
    let padding = session_line_padding(&text, &age, app.last_session_list_area.width);
    Line::from(vec![
        Span::styled(text, session_line_style(app, index, row_index)),
        Span::raw(padding),
        Span::styled(age, Style::default().fg(Color::DarkGray)),
    ])
}

/// Returns the left text column width after reserving room for the age column.
fn session_title_column_width(age: &str, width: u16) -> usize {
    usize::from(width).saturating_sub(age.chars().count() + 1)
}

/// Returns spacing that right-aligns the session age inside the left pane.
fn session_line_padding(text: &str, age: &str, width: u16) -> String {
    let used_width = text.chars().count() + age.chars().count();
    let available_width = usize::from(width);
    let spaces = available_width.saturating_sub(used_width).max(1);
    " ".repeat(spaces)
}

/// Builds a folder-scoped row that opens older conversations.
fn more_line(app: &NexusDemo, row_index: usize) -> Line<'static> {
    Line::styled("  + more", more_line_style(app, row_index))
}

/// Returns the style for a Nexus session row.
fn session_line_style(app: &NexusDemo, index: usize, row_index: usize) -> Style {
    if app
        .session_drag
        .is_some_and(|drag| drag.current_index == index)
    {
        return Style::default().fg(Color::Black).bg(Color::Yellow);
    }
    if row_index == app.focused_row && app.focused_pane == FocusedPane::Left {
        return Style::default().fg(Color::Black).bg(Color::Cyan);
    }
    if index == app.active_index {
        return Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);
    }
    Style::default()
}

/// Returns the style for a folder row.
fn folder_line_style(app: &NexusDemo, path: &std::path::Path, row_index: usize) -> Style {
    if app.folder_drag.as_deref() == Some(path) {
        return Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
    }
    if row_index == app.focused_row && app.focused_pane == FocusedPane::Left {
        return Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
    }
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

/// Returns the style for a folder-scoped more row.
fn more_line_style(app: &NexusDemo, row_index: usize) -> Style {
    if row_index == app.focused_row && app.focused_pane == FocusedPane::Left {
        return Style::default().fg(Color::Black).bg(Color::Cyan);
    }
    Style::default().fg(Color::Magenta)
}
