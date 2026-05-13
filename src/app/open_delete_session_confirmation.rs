use crate::app::app_state::AppState;
use crate::app::selected_delete_targets::selected_delete_targets;
use crate::left_panel::focused_left_row::focused_left_row;
use crate::left_panel::session_list_row::SessionListRow;

/// Opens delete confirmation for selected sessions or the focused session row.
pub fn open_delete_session_confirmation(app: &mut AppState) {
    let selected_targets = selected_delete_targets(app);
    if !selected_targets.is_empty() {
        app.delete_confirmation.open_many(0, selected_targets);
        return;
    }
    open_focused_session_delete_confirmation(app);
}

/// Opens delete confirmation for the focused left-pane session row.
fn open_focused_session_delete_confirmation(app: &mut AppState) {
    let Some(SessionListRow::Session { index }) = focused_left_row(app) else {
        return;
    };
    let Some(entry) = app.session_terminals.get(index) else {
        return;
    };
    app.delete_confirmation
        .open(index, entry.session.id.clone(), entry.session.title.clone());
}
