use ratkit::CoordinatorAction;

use crate::app::events::drain_delete_session_receiver::drain_delete_session_receiver;
use crate::app::events::redraw_action::redraw_action;
use crate::app::sessions::close_exited_sessions::close_exited_sessions;
use crate::app::sessions::drain_initial_sessions_receiver::drain_initial_sessions_receiver;
use crate::app::state::app_state::AppState;
use crate::extensions::expo::observations::drain_cache_receiver::drain_observation_cache_receiver;
use crate::extensions::expo::observations::poll_watcher::poll_observation_watcher;
use crate::extensions::file_viewer::tree::poll_watchers::poll_file_viewer_watchers;
use crate::extensions::harness::sessions::refresh::poll_session_watcher::poll_session_watcher;
use crate::ui::layout::resizable_grid::is_resizing::is_resizing_layout;
use crate::ui::left_panel::session::running_indicator_frame_changed::running_indicator_frame_changed;
use crate::ui::notifications::toast::show_failed_to_replace_chat::show_failed_to_replace_chat_toast;

/// Converts watcher changes and terminal redraw flags into a coordinator action.
pub fn handle_tick_event(app: &mut AppState, tick_count: u64) -> CoordinatorAction {
    let previous_loader_tick = app.loader_tick;
    app.loader_tick = tick_count;
    let initial_sessions_changed = match drain_initial_sessions_receiver(app) {
        Ok(changed) => changed,
        Err(error) => {
            show_failed_to_replace_chat_toast(&mut app.toast_manager, &error);
            true
        }
    };
    poll_observation_watcher(app);
    let observations_changed = drain_observation_cache_receiver(app);
    let delete_changed = drain_delete_session_receiver(app);
    let sessions_changed = poll_session_watcher(app);
    let file_viewer_changed = poll_file_viewer_watchers(app);
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
    let has_active_loader = app
        .session_terminals
        .iter()
        .any(|entry| entry.session.is_running)
        || app.delete_confirmation.is_deleting;
    let loader_changed =
        has_active_loader && running_indicator_frame_changed(previous_loader_tick, tick_count);
    redraw_action(
        sessions_changed
            || initial_sessions_changed
            || observations_changed
            || file_viewer_changed
            || delete_changed
            || exited_changed
            || scroll_changed
            || terminal_changed
            || loader_changed,
    )
}
