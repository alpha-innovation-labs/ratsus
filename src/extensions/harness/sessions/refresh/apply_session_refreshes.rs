use std::collections::BTreeMap;

use crate::app::state::app_state::AppState;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Applies refreshed session metadata and updates dependent visible ordering state.
pub fn apply_session_refreshes(app: &mut AppState, refreshed_sessions: Vec<ChatSession>) -> bool {
    let refreshed_sessions = refreshed_sessions
        .into_iter()
        .filter(|session| !app.closed_chat_session_ids.contains(&session.id))
        .collect();
    let running_before = running_status_by_id(app);
    let active_session_id = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.id.clone());
    let changed = app
        .chat_harness
        .merge_session_refresh(&mut app.session_terminals, refreshed_sessions);
    if changed {
        update_completed_unseen_sessions(app, &running_before, active_session_id.as_deref());
        app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
        app.keep_focused_session_visible();
    }
    changed
}

/// Captures the running state for every session before refresh metadata is applied.
fn running_status_by_id(app: &AppState) -> BTreeMap<String, bool> {
    app.session_terminals
        .iter()
        .map(|entry| (entry.session.id.clone(), entry.session.is_running))
        .collect()
}

/// Marks inactive sessions that just completed work as unseen until the user opens them.
fn update_completed_unseen_sessions(
    app: &mut AppState,
    running_before: &BTreeMap<String, bool>,
    active_session_id: Option<&str>,
) {
    for entry in &app.session_terminals {
        let session_id = entry.session.id.as_str();
        if Some(session_id) == active_session_id || entry.session.is_running {
            app.completed_unseen_session_ids.remove(session_id);
            continue;
        }
        if running_before.get(session_id).copied().unwrap_or(false) {
            app.completed_unseen_session_ids
                .insert(entry.session.id.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::harness::core::chat_session::ChatSession;
    use crate::extensions::harness::sessions::refresh::apply_session_refreshes::apply_session_refreshes;

    /// Verifies completed inactive sessions stay visually marked until opened.
    #[test]
    fn marks_completed_inactive_session_until_activation() {
        let mut app = app_fixture(vec![
            dormant_session("active", "active-id", "/tmp/project"),
            dormant_session("done", "done-id", "/tmp/project"),
        ])
        .expect("app fixture");
        app.active_index = 0;
        app.focused_index = 0;
        app.session_terminals[1].session.is_running = true;

        apply_session_refreshes(
            &mut app,
            vec![
                ChatSession::new("now", "active", "active-id", "/tmp/project"),
                ChatSession::new("now", "done", "done-id", "/tmp/project"),
            ],
        );

        assert!(app.completed_unseen_session_ids.contains("done-id"));
        app.focused_index = 1;
        app.activate_focused_session();
        assert!(!app.completed_unseen_session_ids.contains("done-id"));
    }
}
