use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::load_observation_previews::load_observation_previews;
use crate::expo::observation_preview_request::ObservationPreviewRequest;

/// Spawns a background observation-cache load for requested conversation previews.
pub fn spawn_observation_cache_worker(
    requests: Vec<ObservationPreviewRequest>,
) -> Receiver<HashMap<String, ConversationObservationPreview>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let previews = load_observation_previews(requests).unwrap_or_default();
        let _ = sender.send(previews);
    });
    receiver
}
