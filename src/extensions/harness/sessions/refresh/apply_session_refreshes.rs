use crate::app::state::app_state::AppState;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Applies refreshed session metadata and updates dependent visible ordering state.
pub fn apply_session_refreshes(app: &mut AppState, refreshed_sessions: Vec<ChatSession>) -> bool {
    let refreshed_sessions = refreshed_sessions
        .into_iter()
        .filter(|session| !app.closed_chat_session_ids.contains(&session.id))
        .collect();
    let changed = app
        .chat_harness
        .merge_session_refresh(&mut app.session_terminals, refreshed_sessions);
    if changed {
        app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
        app.keep_focused_session_visible();
    }
    changed
}
