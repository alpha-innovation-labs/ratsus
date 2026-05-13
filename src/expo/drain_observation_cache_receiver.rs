use std::sync::mpsc::TryRecvError;

use crate::app::nexus_demo_state::NexusDemo;

/// Applies a completed background observation-cache load when one is ready.
pub fn drain_observation_cache_receiver(app: &mut NexusDemo) -> bool {
    let Some(receiver) = app.observation_cache_receiver.as_ref() else {
        return false;
    };
    match receiver.try_recv() {
        Ok(previews) => {
            app.observation_previews = previews;
            app.observation_cache_receiver = None;
            true
        }
        Err(TryRecvError::Empty) => false,
        Err(TryRecvError::Disconnected) => {
            app.observation_cache_receiver = None;
            false
        }
    }
}
