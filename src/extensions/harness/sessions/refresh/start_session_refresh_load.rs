use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::refresh::spawn_session_refresh_worker::spawn_session_refresh_worker;

/// Starts one asynchronous session refresh unless a refresh is already in flight.
pub fn start_session_refresh_load(app: &mut AppState) {
    if app.session_refresh_receiver.is_some() {
        return;
    }
    app.session_refresh_receiver = Some(spawn_session_refresh_worker(app.chat_harness.clone()));
}
