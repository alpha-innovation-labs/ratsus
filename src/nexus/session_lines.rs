use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::focused_pane::FocusedPane;
use crate::folder_display_name::folder_display_name;
use crate::nexus_demo_state::NexusDemo;
use crate::session_list_row::SessionListRow;
use crate::visible_session_rows::visible_session_rows;

/// Builds the visible session list lines for the left pane.
pub fn session_lines(app: &NexusDemo) -> Vec<Line<'static>> {
    if app.session_terminals.is_empty() {
        return vec![Line::from("No Nexus sessions")];
    }

    let rows = visible_session_rows(&app.session_terminals, &app.collapsed_folders);
    let visible_height = usize::from(app.last_session_list_area.height);
    let end = (app.session_scroll + visible_height).min(rows.len());
    rows[app.session_scroll..end]
        .iter()
        .map(|row| session_tree_line(app, row))
        .collect()
}

/// Builds one styled line for a folder or session row.
fn session_tree_line(app: &NexusDemo, row: &SessionListRow) -> Line<'static> {
    match row {
        SessionListRow::Folder {
            path,
            session_count,
        } => folder_line(app, path, *session_count),
        SessionListRow::Session { index } => session_line(app, *index),
    }
}

/// Builds one styled line for a folder row.
fn folder_line(app: &NexusDemo, path: &std::path::Path, session_count: usize) -> Line<'static> {
    let marker = if app.collapsed_folders.contains(path) {
        "▸"
    } else {
        "▾"
    };
    let text = format!(
        "{} {} ({})",
        marker,
        folder_display_name(path),
        session_count
    );
    Line::styled(
        text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
}

/// Builds one styled line for a Nexus session row.
fn session_line(app: &NexusDemo, index: usize) -> Line<'static> {
    let entry = &app.session_terminals[index];
    let prefix = if index == app.active_index {
        "●"
    } else {
        " "
    };
    let text = format!("  {} {}", prefix, entry.session.title);
    Line::styled(text, session_line_style(app, index))
}

/// Returns the style for a Nexus session row.
fn session_line_style(app: &NexusDemo, index: usize) -> Style {
    if app
        .session_drag
        .is_some_and(|drag| drag.current_index == index)
    {
        return Style::default().fg(Color::Black).bg(Color::Yellow);
    }
    if index == app.focused_index && app.focused_pane == FocusedPane::Left {
        return Style::default().fg(Color::Black).bg(Color::Cyan);
    }
    if index == app.active_index {
        return Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);
    }
    Style::default()
}
