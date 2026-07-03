use chrono::Utc;
use ratatui::text::Line;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::ui::grid_layout::bundle::session_bundle_marker::session_bundle_marker;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::folder::has_running_session::folder_has_running_session;
use crate::ui::left_panel::render::rendered_row::RenderedLeftPanelRow;
use crate::ui::left_panel::render::rendered_rows::rendered_left_panel_rows_from_rows;
use crate::ui::left_panel::render::session_row_line::{
    folder_row_line, session_row_line, session_row_separator_line, split_group_row_line,
    ChatSessionRowLineConfig, FolderRowLineConfig,
};
use crate::ui::left_panel::render::text_width::left_panel_text_width;
use crate::ui::left_panel::session::format_age::format_session_age;
use crate::ui::left_panel::session::icon::session_icon;
use crate::ui::left_panel::session::is_active_chat_row::session_is_active_chat_row;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::running_indicator::running_session_indicator;

/// Builds the visible session list lines for the left pane.
pub fn session_lines(app: &AppState) -> Vec<Line<'static>> {
    if app.session_terminals.is_empty() {
        return vec![Line::from(format!(
            "No {} sessions",
            app.chat_harness.display_name()
        ))];
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
    app: &AppState,
    rendered_row: &RenderedLeftPanelRow,
    line_width: u16,
) -> Line<'static> {
    match rendered_row {
        RenderedLeftPanelRow::SessionListRow {
            source_row_index,
            row,
        } => session_tree_line(app, row, *source_row_index, line_width),
        RenderedLeftPanelRow::Separator { label } => {
            session_row_separator_line(line_width, label.as_deref())
        }
    }
}

/// Builds one styled line for a folder or session row.
fn session_tree_line(
    app: &AppState,
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
        SessionListRow::SplitGroup {
            group_id,
            name,
            child_count,
        } => split_group_row_line(
            name,
            *child_count,
            line_width,
            is_selected(app, row_index),
            split_group_is_active(app, *group_id),
        ),
        SessionListRow::SplitGroupChild { index, is_last, .. } => {
            split_group_child_line(app, *index, *is_last, row_index, line_width)
        }
        SessionListRow::Session { index } => session_line(app, *index, row_index, line_width),
    }
}

/// Builds one styled line for a chat session row.
fn session_line(app: &AppState, index: usize, row_index: usize, line_width: u16) -> Line<'static> {
    let entry = &app.session_terminals[index];
    let age = format_session_age(&entry.session, Utc::now());
    let icon = if entry.session.is_running {
        running_session_indicator(app.loader_tick)
    } else {
        session_icon(&entry.session)
    };
    session_row_line(ChatSessionRowLineConfig {
        title: &entry.session.title,
        age: &age,
        icon,
        width: line_width,
        is_selected: is_selected(app, row_index),
        is_active: session_is_active_chat_row(app.active_main_pane_tab, index, app.active_index),
        is_running: entry.session.is_running,
        is_completed_unseen: app.completed_unseen_session_ids.contains(&entry.session.id),
        is_dragging: app
            .session_drag
            .is_some_and(|drag| drag.current_index == index),
        is_toggled: app.selected_conversation_ids.contains(&entry.session.id),
        bundle_marker: session_bundle_marker(app, index),
        tree_prefix: None,
    })
}

/// Builds one styled row for a session nested under a split group parent.
fn split_group_child_line(
    app: &AppState,
    index: usize,
    is_last: bool,
    row_index: usize,
    line_width: u16,
) -> Line<'static> {
    let entry = &app.session_terminals[index];
    let age = format_session_age(&entry.session, Utc::now());
    let icon = if entry.session.is_running {
        running_session_indicator(app.loader_tick)
    } else {
        session_icon(&entry.session)
    };
    session_row_line(ChatSessionRowLineConfig {
        title: &entry.session.title,
        age: &age,
        icon,
        width: line_width,
        is_selected: is_selected(app, row_index),
        is_active: session_is_active_chat_row(app.active_main_pane_tab, index, app.active_index),
        is_running: entry.session.is_running,
        is_completed_unseen: app.completed_unseen_session_ids.contains(&entry.session.id),
        is_dragging: app
            .session_drag
            .is_some_and(|drag| drag.current_index == index),
        is_toggled: app.selected_conversation_ids.contains(&entry.session.id),
        bundle_marker: None,
        tree_prefix: Some(if is_last { "  └─ " } else { "  ├─ " }),
    })
}

/// Returns whether any child of a split group is the active chat row.
fn split_group_is_active(app: &AppState, group_id: u64) -> bool {
    let Some(group) = app.split_pane_session_groups.groups.get(&group_id) else {
        return false;
    };
    let Some(active_session_id) = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.id.as_str())
    else {
        return false;
    };
    group.panes.iter().any(|pane_id| {
        app.terminal_pane_session_bundles
            .get(pane_id)
            .is_some_and(|session_ids| {
                session_ids
                    .iter()
                    .any(|candidate| candidate == active_session_id)
            })
    })
}

/// Returns whether a folder owns the active Expo view.
fn is_active_expo_folder(app: &AppState, path: &std::path::Path) -> bool {
    app.active_main_pane_tab == MainPaneTab::Expo
        && app.selected_expo_folder.as_deref() == Some(path)
}

/// Returns whether the row has the left pane keyboard selection highlight.
fn is_selected(app: &AppState, row_index: usize) -> bool {
    row_index == app.focused_row && app.focused_pane == FocusedPane::Left
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use ratatui::layout::Rect;

    use super::session_lines;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::harness::core::chat_session::ChatSession;
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;

    /// Verifies workspace-scoped regular session rows are not indented under the folder.
    #[test]
    fn workspace_session_rows_are_left_aligned() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.last_session_list_area = Rect::new(0, 0, 40, 10);

        let lines = session_lines(&app);
        let session_line = lines
            .iter()
            .map(line_text)
            .find(|text| text.contains("Alpha"))
            .expect("session row");

        assert!(!session_line.starts_with(' '));
        Ok(())
    }

    /// Verifies session rows are separated by day-group labels in the consolidated view.
    #[test]
    fn session_rows_show_day_group_separators() -> anyhow::Result<()> {
        let now = Utc::now();
        let mut app = app_fixture(vec![
            session_with_date("Today", "today", (now - Duration::hours(2)).to_rfc3339()),
            session_with_date(
                "Yesterday",
                "one-day",
                (now - Duration::days(1)).to_rfc3339(),
            ),
            session_with_date("Older", "two-days", (now - Duration::days(2)).to_rfc3339()),
        ])?;
        app.last_session_list_area = Rect::new(0, 0, 60, 10);
        app.folder_order = vec!["/tmp/project".into()];

        let texts = session_lines(&app)
            .iter()
            .map(line_text)
            .collect::<Vec<_>>();
        let separators = texts
            .iter()
            .filter(|text| text.contains('─'))
            .collect::<Vec<_>>();

        assert_eq!(separators.len(), 3);
        assert!(separators.iter().any(|text| text.contains(" today ")));
        assert!(separators.iter().any(|text| text.contains(" 1d ")));
        assert!(separators.iter().any(|text| text.contains(" 2d ")));
        Ok(())
    }

    /// Builds a dormant session entry with an explicit modified date.
    fn session_with_date(title: &str, id: &str, date: String) -> SessionTerminal {
        SessionTerminal::dormant(ChatSession::new(date, title, id, "/tmp/project"))
    }

    /// Returns the concatenated visible text for a rendered line.
    fn line_text(line: &ratatui::text::Line<'_>) -> String {
        line.spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect()
    }
}
