use anyhow::Result;

use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::creation::start_new_chat_in_dir::start_new_chat_in_dir;
use crate::extensions::harness::sessions::selection::focused_working_dir::focused_chat_session_working_dir;

/// Starts a fresh chat beside the existing sessions and focuses it.
pub fn start_new_chat(app: &mut AppState) -> Result<()> {
    let working_dir = focused_chat_session_working_dir(app)?;
    start_new_chat_in_dir(app, &working_dir)
}
