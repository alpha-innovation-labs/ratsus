use crate::app::app_state::AppState;
use crate::app::confirm_delete_session::delete_target_indices;
use crate::app::restore_focus_after_bulk_delete::restore_focus_after_bulk_delete;
use crate::terminal::is_chat_session::is_chat_session;
use crate::terminal::persist_normal_terminal_sessions::persist_normal_terminal_sessions;

/// Removes successfully deleted sessions from app state and restores focus.
pub fn complete_delete_sessions(app: &mut AppState, session_ids: &[String]) {
    let preferred_row = app.delete_confirmation.preferred_focus_row;
    let deleted_indices = delete_target_indices(app, session_ids);
    for index in deleted_indices.iter().rev().copied() {
        remove_session_at_index(app, index);
    }
    app.delete_confirmation.close();
    app.delete_session_receiver = None;
    if app.chat_harness.persist_normal_terminals() {
        persist_normal_terminal_sessions(&app.session_terminals);
    }
    if !deleted_indices.is_empty() {
        restore_focus_after_bulk_delete(app, preferred_row);
    }
}

/// Removes one session entry and updates related local state.
fn remove_session_at_index(app: &mut AppState, index: usize) {
    let session_id = app.session_terminals[index].session.id.clone();
    if is_chat_session(&app.session_terminals[index].session) {
        app.closed_chat_session_ids.insert(session_id.clone());
    } else if let Some(terminal) = app.session_terminals[index].terminal.as_mut() {
        terminal.kill();
    }
    app.selected_conversation_ids.remove(&session_id);
    app.session_terminals.remove(index);
}
