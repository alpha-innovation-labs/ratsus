use ratkit::CoordinatorAction;

use crate::app::close_exited_sessions::close_exited_sessions;
use crate::app::drain_delete_session_receiver::drain_delete_session_receiver;
use crate::app::nexus_demo_state::NexusDemo;
use crate::app::redraw_action::redraw_action;
use crate::expo::drain_observation_cache_receiver::drain_observation_cache_receiver;
use crate::expo::poll_observation_watcher::poll_observation_watcher;
use crate::layout::is_resizing_layout::is_resizing_layout;
use crate::nexus_sessions::drain_session_refreshes::drain_session_refreshes;
use crate::notifications::show_failed_to_replace_chat_toast::show_failed_to_replace_chat_toast;

/// Converts completed background refreshes and terminal redraw flags into a coordinator action.
pub fn handle_tick_event(app: &mut NexusDemo, tick_count: u64) -> CoordinatorAction {
    app.loader_tick = tick_count;
    poll_observation_watcher(app);
    let observations_changed = drain_observation_cache_receiver(app);
    let delete_changed = drain_delete_session_receiver(app);
    let sessions_changed = drain_session_refreshes(app);
    let exited_changed = match close_exited_sessions(app) {
        Ok(changed) => changed,
        Err(error) => {
            show_failed_to_replace_chat_toast(&mut app.toast_manager, &error);
            true
        }
    };
    if is_resizing_layout(&app.layout_widget_state) {
        return redraw_action(sessions_changed);
    }

    let scroll_changed = app.pending_left_scroll_redraw;
    app.pending_left_scroll_redraw = false;
    let terminal_changed = app
        .session_terminals
        .iter()
        .filter_map(|entry| entry.terminal.as_ref())
        .any(|terminal| terminal.take_needs_redraw());
    let loader_changed = app
        .session_terminals
        .iter()
        .any(|entry| entry.session.is_running)
        || app.delete_confirmation.is_deleting;
    redraw_action(
        sessions_changed
            || observations_changed
            || delete_changed
            || exited_changed
            || scroll_changed
            || terminal_changed
            || loader_changed,
    )
}
