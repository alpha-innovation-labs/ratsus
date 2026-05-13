use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::observation_preview_requests::observation_preview_requests;
use crate::expo::spawn_observation_cache_worker::spawn_observation_cache_worker;

/// Starts one background observation-cache load for every known Nexus session.
pub fn start_observation_cache_load(app: &mut NexusDemo) {
    let requests = observation_preview_requests(&app.session_terminals);
    app.observation_cache_receiver = Some(spawn_observation_cache_worker(requests));
}
