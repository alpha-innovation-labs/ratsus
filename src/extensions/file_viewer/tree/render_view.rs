use ratatui::text::Line;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::extensions::file_viewer::tree::render::file_entry_line::{
    file_entry_line, FileEntryLineConfig,
};
use crate::extensions::file_viewer::tree::render::file_entry_title::file_entry_title;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::file_viewer::tree::workspace_file_row::WorkspaceFileRow;

/// Renders the workspace-grouped file-system tree body inside the shared left-pane shell.
pub fn render_file_system_tree_view(view: &mut FileSystemTreeView, frame: &mut Frame, area: Rect) {
    view.set_tree_area(area);
    frame.render_widget(Paragraph::new(workspace_file_lines(view)), area);
}

/// Builds visible lines for the workspace-grouped file tree.
fn workspace_file_lines(view: &FileSystemTreeView) -> Vec<Line<'static>> {
    let rows = view.workspace_rows();
    if rows.is_empty() {
        return vec![Line::from("No files")];
    }
    rows.iter()
        .skip(view.workspace_scroll)
        .take(usize::from(view.last_tree_area.height))
        .enumerate()
        .map(|(offset, row)| workspace_file_line(view, row, view.workspace_scroll + offset))
        .collect()
}

/// Builds one grouped file-tree line with the original Ratkit file-tree colors.
fn workspace_file_line(
    view: &FileSystemTreeView,
    row: &WorkspaceFileRow,
    row_index: usize,
) -> Line<'static> {
    match row {
        WorkspaceFileRow::WorkspaceFolder { path } => file_entry_line(FileEntryLineConfig {
            name: &file_entry_title(path),
            depth: 0,
            is_dir: true,
            expanded: !view.workspace_collapsed_paths.contains(path),
            selected: row_index == view.workspace_focused_row,
            git_status: view.git_status_for_path(path, true),
            use_dark_theme: view.tree.config.use_dark_theme,
            dir_style: view.tree.config.dir_style,
            file_style: view.tree.config.file_style,
        }),
        WorkspaceFileRow::Entry {
            path,
            depth,
            is_dir,
        } => file_entry_line(FileEntryLineConfig {
            name: &file_entry_title(path),
            depth: *depth,
            is_dir: *is_dir,
            expanded: view.workspace_expanded_paths.contains(path),
            selected: row_index == view.workspace_focused_row,
            git_status: view.git_status_for_path(path, *is_dir),
            use_dark_theme: view.tree.config.use_dark_theme,
            dir_style: view.tree.config.dir_style,
            file_style: view.tree.config.file_style,
        }),
    }
}
