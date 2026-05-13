use crate::app::app_state::AppState;

/// Visual position for a session inside a terminal pane bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionBundleMarker {
    /// First row in a bundled group.
    First,
    /// Middle row in a bundled group.
    Middle,
    /// Last row in a bundled group.
    Last,
}

/// Returns the bundle marker for a session row when it belongs to a multi-session pane.
pub fn session_bundle_marker(app: &AppState, session_index: usize) -> Option<SessionBundleMarker> {
    let session_id = &app.session_terminals.get(session_index)?.session.id;
    app.terminal_pane_session_bundles
        .values()
        .filter(|bundle| bundle.len() > 1)
        .find_map(|bundle| marker_in_bundle(bundle, session_id))
}

/// Returns a row marker for one session id inside a bundle.
fn marker_in_bundle(bundle: &[String], session_id: &str) -> Option<SessionBundleMarker> {
    let position = bundle
        .iter()
        .position(|candidate| candidate == session_id)?;
    if position == 0 {
        return Some(SessionBundleMarker::First);
    }
    if position + 1 == bundle.len() {
        return Some(SessionBundleMarker::Last);
    }
    Some(SessionBundleMarker::Middle)
}

#[cfg(test)]
mod tests {
    use super::{marker_in_bundle, SessionBundleMarker};

    /// The first item receives the first branch marker.
    #[test]
    fn marks_first_bundle_entry() {
        let bundle = vec!["a".to_string(), "b".to_string()];

        assert_eq!(
            marker_in_bundle(&bundle, "a"),
            Some(SessionBundleMarker::First)
        );
    }

    /// The final item receives the last branch marker.
    #[test]
    fn marks_last_bundle_entry() {
        let bundle = vec!["a".to_string(), "b".to_string()];

        assert_eq!(
            marker_in_bundle(&bundle, "b"),
            Some(SessionBundleMarker::Last)
        );
    }
}
