use std::path::PathBuf;

use ratatui::layout::Rect;

use crate::app::state::app_state::AppState;
use crate::ui::workspace_pane::workspace_box_height::workspace_box_height;
use crate::ui::workspace_pane::workspace_paths::workspace_paths;

/// Returns the workspace path rendered at a mouse row.
pub fn workspace_path_for_mouse_row(app: &AppState, row: u16, area: Rect) -> Option<PathBuf> {
    if row < area.y || row >= area.y.saturating_add(area.height) {
        return None;
    }
    let offset = usize::from(row.saturating_sub(area.y)) / workspace_box_height();
    workspace_paths(app)
        .get(app.workspace_scroll.saturating_add(offset))
        .cloned()
}
