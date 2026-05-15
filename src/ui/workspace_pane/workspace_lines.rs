use ratatui::text::Line;

use crate::app::state::app_state::AppState;
use crate::ui::workspace_pane::workspace_box_height::workspace_box_height;
use crate::ui::workspace_pane::workspace_box_lines::workspace_box_lines;
use crate::ui::workspace_pane::workspace_paths::workspace_paths;

/// Builds visible workspace pane lines from the scrolled workspace order.
pub fn workspace_lines(app: &AppState) -> Vec<Line<'static>> {
    let paths = workspace_paths(app);
    if paths.is_empty() {
        return vec![Line::from("No workspaces")];
    }
    let visible_boxes =
        (usize::from(app.last_workspace_list_area.height) / workspace_box_height()).max(1);
    let start = app.workspace_scroll.min(paths.len());
    let end = start.saturating_add(visible_boxes).min(paths.len());
    paths[start..end]
        .iter()
        .flat_map(|path| workspace_box_lines(app, path, app.last_workspace_list_area.width))
        .collect()
}
