use anyhow::Result;

use crate::app::nexus_demo_state::NexusDemo;
use crate::nexus_sessions::focused_nexus_session_working_dir::focused_nexus_session_working_dir;
use crate::nexus_sessions::start_new_nexus_chat_in_dir::start_new_nexus_chat_in_dir;

/// Starts a fresh Nexus chat beside the existing sessions and focuses it.
pub fn start_new_nexus_chat(app: &mut NexusDemo) -> Result<()> {
    let working_dir = focused_nexus_session_working_dir(app)?;
    start_new_nexus_chat_in_dir(app, &working_dir)
}
