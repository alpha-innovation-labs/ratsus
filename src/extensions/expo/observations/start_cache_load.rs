use crate::app::state::app_state::AppState;
use crate::extensions::expo::observations::preview_requests::observation_preview_requests;
use crate::extensions::expo::observations::spawn_cache_worker::spawn_observation_cache_worker;

/// Starts one background observation-cache load for every known chat session.
pub fn start_observation_cache_load(app: &mut AppState) {
    let requests = observation_preview_requests(&app.session_terminals);
    app.observation_cache_receiver = Some(spawn_observation_cache_worker(
        app.chat_harness.clone(),
        requests,
    ));
}
