use std::fs;
use std::path::{Path, PathBuf};

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::file_viewer::tree::workspace_file_row::WorkspaceFileRow;

impl FileSystemTreeView {
    /// Returns visible rows for the workspace-grouped file tree.
    pub(crate) fn workspace_rows(&self) -> Vec<WorkspaceFileRow> {
        let mut rows = Vec::new();
        for root in &self.workspace_roots {
            let before = rows.len();
            rows.push(WorkspaceFileRow::WorkspaceFolder { path: root.clone() });
            if !self.workspace_collapsed_paths.contains(root) {
                append_visible_entries(self, root, 1, &mut rows);
            }
            if self.workspace_filtering && rows.len() == before + 1 {
                rows.pop();
            }
        }
        rows
    }
}

/// Appends visible entries under one directory.
fn append_visible_entries(
    view: &FileSystemTreeView,
    dir: &Path,
    depth: usize,
    rows: &mut Vec<WorkspaceFileRow>,
) {
    let Ok(entries) = sorted_child_paths(dir) else {
        return;
    };
    for path in entries {
        let is_dir = path.is_dir();
        let matches_filter = workspace_file_matches_filter(view, &path);
        if !view.workspace_filtering || matches_filter || child_matches_filter(view, &path) {
            rows.push(WorkspaceFileRow::Entry {
                path: path.clone(),
                depth,
                is_dir,
            });
        }
        if is_dir
            && view.workspace_expanded_paths.contains(&path)
            && (!view.workspace_filtering || child_matches_filter(view, &path))
        {
            append_visible_entries(view, &path, depth + 1, rows);
        }
    }
}

/// Returns sorted direct children of a directory with directories first.
fn sorted_child_paths(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    paths.sort_by(|left, right| {
        right
            .is_dir()
            .cmp(&left.is_dir())
            .then_with(|| display_name(left).cmp(&display_name(right)))
    });
    Ok(paths)
}

/// Returns whether a path or one descendant matches the file filter.
fn child_matches_filter(view: &FileSystemTreeView, path: &Path) -> bool {
    if workspace_file_matches_filter(view, path) {
        return true;
    }
    if !path.is_dir() {
        return false;
    }
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };
    entries
        .filter_map(Result::ok)
        .any(|entry| child_matches_filter(view, &entry.path()))
}

/// Returns whether one path matches the active file filter text.
fn workspace_file_matches_filter(view: &FileSystemTreeView, path: &Path) -> bool {
    if view.workspace_filter_query.is_empty() {
        return true;
    }
    display_name(path)
        .to_ascii_lowercase()
        .contains(&view.workspace_filter_query.to_ascii_lowercase())
}

/// Returns a stable display name for one path.
fn display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}
