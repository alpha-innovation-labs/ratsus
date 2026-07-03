use std::sync::mpsc::TryRecvError;

use anyhow::Result;

use crate::app::state::app_state::AppState;
use crate::extensions::expo::observations::preview_requests::observation_preview_requests;
use crate::extensions::expo::observations::spawn_cache_worker::spawn_observation_cache_worker;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::persistence::restore_multiplexer_state_into_app::restore_multiplexer_state_into_app;
use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;
use crate::ui::left_panel::focus::session_visible_row_index::session_visible_row_index;
use crate::ui::left_panel::order::apply_session_id_order::apply_session_id_order;
use crate::ui::left_panel::order::load_preferences::load_session_order_preferences;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;
use crate::ui::left_panel::session::sort_by_creation_date::sort_sessions_by_creation_date;
use crate::ui::left_panel::session::visible_rows::visible_session_rows_with_folders;

/// Applies startup sessions loaded by the background worker when ready.
pub fn drain_initial_sessions_receiver(app: &mut AppState) -> Result<bool> {
    let Some(receiver) = app.initial_sessions_receiver.as_ref() else {
        return Ok(false);
    };
    match receiver.try_recv() {
        Ok(Ok(sessions)) => {
            app.initial_sessions_receiver = None;
            apply_initial_sessions(app, sessions);
            Ok(true)
        }
        Ok(Err(error)) => {
            app.initial_sessions_receiver = None;
            Err(error)
        }
        Err(TryRecvError::Empty) => Ok(false),
        Err(TryRecvError::Disconnected) => {
            app.initial_sessions_receiver = None;
            Ok(false)
        }
    }
}

/// Merges loaded startup sessions into a shell that was shown before IO completed.
fn apply_initial_sessions(
    app: &mut AppState,
    sessions: Vec<crate::extensions::harness::core::chat_session::ChatSession>,
) {
    let preferences = load_session_order_preferences();
    let mut session_terminals = sessions
        .into_iter()
        .map(|session| SessionTerminal::dormant_with_harness(session, app.chat_harness.clone()))
        .collect::<Vec<_>>();
    sort_sessions_by_creation_date(&mut session_terminals);
    apply_session_id_order(&mut session_terminals, &preferences.session_ids);
    let active_index = preferences
        .active_session_id
        .as_deref()
        .and_then(|id| {
            session_terminals
                .iter()
                .position(|entry| entry.session.id == id)
        })
        .unwrap_or(0);
    app.session_terminals = session_terminals;
    app.active_index = active_index;
    app.focused_index = active_index;
    app.folder_order = sync_folder_order(&preferences.folder_paths, &app.session_terminals);
    app.observation_cache_receiver = Some(spawn_observation_cache_worker(
        app.chat_harness.clone(),
        observation_preview_requests(&app.session_terminals),
    ));
    app.terminal_pane_sessions.clear();
    app.terminal_pane_session_bundles.clear();
    if let Some(session) = app.session_terminals.get(active_index) {
        let pane_id = TERMINAL_PANE_ID;
        app.active_terminal_pane_id = pane_id;
        app.terminal_pane_sessions
            .insert(pane_id, session.session.id.clone());
        app.terminal_pane_session_bundles
            .insert(pane_id, vec![session.session.id.clone()]);
    }
    restore_multiplexer_state_into_app(app);
    app.focused_row = session_visible_row_index(
        &visible_session_rows_with_folders(
            &app.session_terminals,
            &app.collapsed_folders,
            &app.folder_order,
            Some(app.active_index),
            &app.split_pane_session_groups,
            &app.terminal_pane_session_bundles,
        ),
        app.active_index,
    )
    .unwrap_or(0);
}
