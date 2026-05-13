use std::sync::mpsc::TryRecvError;

use crate::app::deletion::complete_delete_sessions::complete_delete_sessions;
use crate::app::deletion::delete_sessions_result::DeleteSessionsResult;
use crate::app::deletion::removable_delete_session_ids::removable_delete_session_ids;
use crate::app::state::app_state::AppState;
use crate::ui::notifications::toast::show_failed_to_delete_session::show_failed_to_delete_session_toast;

/// Drains the background delete worker result when available.
pub fn drain_delete_session_receiver(app: &mut AppState) -> bool {
    let Some(receiver) = app.delete_session_receiver.as_ref() else {
        return false;
    };
    match receiver.try_recv() {
        Ok(result) => handle_delete_result(app, result),
        Err(TryRecvError::Empty) => false,
        Err(TryRecvError::Disconnected) => {
            app.delete_session_receiver = None;
            app.delete_confirmation.stop_deleting();
            true
        }
    }
}

/// Applies a completed delete worker result to app state.
fn handle_delete_result(app: &mut AppState, result: DeleteSessionsResult) -> bool {
    let removable_ids = removable_delete_session_ids(app, &result.deleted_chat_ids);
    if result.has_errors() {
        show_failed_to_delete_session_toast(
            &mut app.toast_manager,
            &anyhow::anyhow!(result.error_message()),
        );
    }
    complete_delete_sessions(app, &removable_ids);
    true
}
