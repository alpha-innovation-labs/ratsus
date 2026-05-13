use crate::expo::observation_preview_request::ObservationPreviewRequest;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds observation-preview requests for all session terminals.
pub fn observation_preview_requests(
    sessions: &[SessionTerminal],
) -> Vec<ObservationPreviewRequest> {
    sessions
        .iter()
        .map(|entry| {
            ObservationPreviewRequest::new(entry.session.id.clone(), entry.session.title.clone())
        })
        .collect()
}
