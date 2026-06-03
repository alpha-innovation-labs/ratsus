use crossterm::event::{MouseButton, MouseEventKind};

use crate::app::state::app_state::AppState;
use crate::extensions::plans::data::plan_list_row::PlanListRow;

/// Focuses and activates the grouped plan row under a left-click.
pub fn focus_clicked_plan(app: &mut AppState, mouse: ratkit::MouseEvent) {
    if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
        return;
    }
    if mouse.row < app.plan_list.last_area.y {
        return;
    }
    let visible_row = app.plan_list.scroll + usize::from(mouse.row - app.plan_list.last_area.y);
    let Some(row) = app.plan_list.visible_rows().get(visible_row).cloned() else {
        return;
    };
    app.plan_list.focused_row = visible_row;
    match row {
        PlanListRow::Folder { path, .. } => app.plan_list.toggle_folder(&path),
        PlanListRow::Plan { .. } => app.plan_list.activate_focused(),
    }
}
