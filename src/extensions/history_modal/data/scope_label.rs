use crate::app::state::app_state::AppState;

const ACTIVE_SCOPE_MARKER: &str = "●";
const INACTIVE_SCOPE_MARKER: &str = "○";

/// Returns the visible picker scope label for the dialog header.
pub fn history_modal_scope_label(_app: &AppState) -> String {
    format!("{} All", all_scope_marker())
}

/// Returns the marker for the all picker option.
fn all_scope_marker() -> &'static str {
    ACTIVE_SCOPE_MARKER
}
