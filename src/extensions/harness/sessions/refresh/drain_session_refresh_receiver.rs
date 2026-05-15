use std::sync::mpsc::TryRecvError;

use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::refresh::apply_session_refreshes::apply_session_refreshes;

/// Applies a completed session refresh worker result when one is ready.
pub fn drain_session_refresh_receiver(app: &mut AppState) -> bool {
    let Some(receiver) = app.session_refresh_receiver.as_ref() else {
        return false;
    };
    match receiver.try_recv() {
        Ok(Ok(refreshed_sessions)) => {
            app.session_refresh_receiver = None;
            apply_session_refreshes(app, refreshed_sessions)
        }
        Ok(Err(_)) | Err(TryRecvError::Disconnected) => {
            app.session_refresh_receiver = None;
            false
        }
        Err(TryRecvError::Empty) => false,
    }
}
