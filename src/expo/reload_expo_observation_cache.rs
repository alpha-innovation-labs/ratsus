use crate::app::app_state::AppState;
use crate::expo::expo_session_indices::expo_session_indices;
use crate::expo::observation_preview_request::ObservationPreviewRequest;

/// Reloads cached observation previews for the currently selected Expo folder once.
pub fn reload_expo_observation_cache(app: &mut AppState) {
    let requests = expo_session_indices(app)
        .into_iter()
        .filter_map(|index| app.session_terminals.get(index))
        .map(|entry| {
            ObservationPreviewRequest::new(entry.session.id.clone(), entry.session.title.clone())
        })
        .collect::<Vec<_>>();
    app.observation_previews = app
        .chat_harness
        .load_observation_previews(requests)
        .unwrap_or_default();
}
