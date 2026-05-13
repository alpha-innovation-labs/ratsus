use chrono::Utc;
use ratatui::text::Line;

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::folder_has_running_session::folder_has_running_session;
use crate::left_panel::format_session_age::format_session_age;
use crate::left_panel::left_panel_text_width::left_panel_text_width;
use crate::left_panel::rendered_left_panel_row::RenderedLeftPanelRow;
use crate::left_panel::rendered_left_panel_rows::rendered_left_panel_rows_from_rows;
use crate::left_panel::running_session_indicator::running_session_indicator;
use crate::left_panel::session_icon::session_icon;
use crate::left_panel::session_is_active_chat_row::session_is_active_chat_row;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_row_line::{
    folder_more_row_line, folder_row_line, nexus_session_row_line, session_row_separator_line,
    FolderRowLineConfig, NexusSessionRowLineConfig,
};
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::session_panes::session_bundle_marker::session_bundle_marker;

/// Builds the visible session list lines for the left pane.
pub fn session_lines(app: &NexusDemo) -> Vec<Line<'static>> {
    if app.session_terminals.is_empty() {
        return vec![Line::from("No Nexus sessions")];
    }

    let rows = app.visible_rows();
    let line_width = left_panel_text_width(app.last_session_list_area, rows.len());
    rendered_left_panel_rows_from_rows(app, &rows, usize::from(app.last_session_list_area.height))
        .iter()
        .map(|rendered_row| rendered_session_line(app, rendered_row, line_width))
        .collect()
}

/// Builds one line from a rendered row, including separators.
fn rendered_session_line(
    app: &NexusDemo,
    rendered_row: &RenderedLeftPanelRow,
    line_width: u16,
) -> Line<'static> {
    match rendered_row {
        RenderedLeftPanelRow::SessionListRow {
            source_row_index,
            row,
        } => session_tree_line(app, row, *source_row_index, line_width),
        RenderedLeftPanelRow::Separator => session_row_separator_line(line_width),
    }
}

/// Builds one styled line for a folder or session row.
fn session_tree_line(
    app: &NexusDemo,
    row: &SessionListRow,
    row_index: usize,
    line_width: u16,
) -> Line<'static> {
    match row {
        SessionListRow::Folder {
            path,
            current_session_count,
            total_session_count,
        } => folder_row_line(FolderRowLineConfig {
            path,
            current_session_count: *current_session_count,
            total_session_count: *total_session_count,
            is_collapsed: app.collapsed_folders.contains(path),
            has_running_session: folder_has_running_session(path, &app.session_terminals),
            loader_tick: app.loader_tick,
            width: line_width,
            show_full_path: false,
            is_active_expo_folder: is_active_expo_folder(app, path),
            is_selected: is_selected(app, row_index),
            is_dragging: app.folder_drag.as_deref() == Some(path.as_path()),
        }),
        SessionListRow::Session { index } => session_line(app, *index, row_index, line_width),
        SessionListRow::FolderMore { .. } => folder_more_row_line(is_selected(app, row_index)),
    }
}

/// Builds one styled line for a Nexus session row.
fn session_line(app: &NexusDemo, index: usize, row_index: usize, line_width: u16) -> Line<'static> {
    let entry = &app.session_terminals[index];
    let age = format_session_age(&entry.session, Utc::now());
    let icon = if entry.session.is_running {
        running_session_indicator(app.loader_tick)
    } else {
        session_icon(&entry.session)
    };
    nexus_session_row_line(NexusSessionRowLineConfig {
        title: &entry.session.title,
        age: &age,
        icon,
        width: line_width,
        is_selected: is_selected(app, row_index),
        is_active: session_is_active_chat_row(app.active_main_pane_tab, index, app.active_index),
        is_dragging: app
            .session_drag
            .is_some_and(|drag| drag.current_index == index),
        is_toggled: app.selected_conversation_ids.contains(&entry.session.id),
        bundle_marker: session_bundle_marker(app, index),
    })
}

/// Returns whether a folder owns the active Expo view.
fn is_active_expo_folder(app: &NexusDemo, path: &std::path::Path) -> bool {
    app.active_main_pane_tab == MainPaneTab::Expo
        && app.selected_expo_folder.as_deref() == Some(path)
}

/// Returns whether the row has the left pane keyboard selection highlight.
fn is_selected(app: &NexusDemo, row_index: usize) -> bool {
    row_index == app.focused_row && app.focused_pane == FocusedPane::Left
}
