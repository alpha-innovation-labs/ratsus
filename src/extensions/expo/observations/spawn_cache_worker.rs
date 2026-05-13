use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
use crate::extensions::expo::observations::preview_request::ObservationPreviewRequest;
use crate::extensions::harness::core::chat_harness::ChatHarness;

/// Spawns a background observation-cache load for requested conversation previews.
pub fn spawn_observation_cache_worker(
    chat_harness: Arc<dyn ChatHarness>,
    requests: Vec<ObservationPreviewRequest>,
) -> Receiver<HashMap<String, ConversationObservationPreview>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let previews = chat_harness
            .load_observation_previews(requests)
            .unwrap_or_default();
        let _ = sender.send(previews);
    });
    receiver
}
