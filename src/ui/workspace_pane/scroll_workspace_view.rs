use crate::app::state::app_state::AppState;
use crate::ui::workspace_pane::workspace_box_height::workspace_box_height;

/// Scrolls the workspace pane by a signed workspace delta.
pub fn scroll_workspace_view(app: &mut AppState, delta: isize) -> bool {
    let row_count = app.folder_order.len();
    let visible_boxes =
        (usize::from(app.last_workspace_list_area.height) / workspace_box_height()).max(1);
    let max_scroll = row_count.saturating_sub(visible_boxes);
    let next = app
        .workspace_scroll
        .saturating_add_signed(delta)
        .min(max_scroll);
    let changed = next != app.workspace_scroll;
    app.workspace_scroll = next;
    changed
}
