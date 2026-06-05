use crate::app::deletion::selected_delete_targets::selected_delete_targets;
use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItemKind;
use crate::extensions::history_modal::selection::selected_item::selected_history_modal_item;

/// Opens delete confirmation for selected picker sessions or the highlighted session.
pub fn open_selected_conversation_delete_confirmation(app: &mut AppState) {
    let selected_targets = selected_delete_targets(app);
    if !selected_targets.is_empty() {
        app.delete_confirmation.open_many(0, selected_targets);
        return;
    }
    open_highlighted_conversation_delete_confirmation(app);
}

/// Opens delete confirmation for the highlighted picker session.
fn open_highlighted_conversation_delete_confirmation(app: &mut AppState) {
    let Some(item) = selected_history_modal_item(app) else {
        return;
    };
    let HistoryModalItemKind::Session { index, .. } = item.kind else {
        return;
    };
    let Some(entry) = app.session_terminals.get(index) else {
        return;
    };
    app.delete_confirmation
        .open(index, entry.session.id.clone(), entry.session.title.clone());
}
