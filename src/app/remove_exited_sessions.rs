use std::path::PathBuf;

use crate::app::nexus_demo_state::NexusDemo;
use crate::terminal::is_chat_session::is_chat_session;

/// Summary of entries removed after their terminal process exited.
pub struct RemovedExitedSessions {
    pub removed_active: bool,
    pub removed_active_chat: bool,
    pub fallback_working_dir: PathBuf,
}

/// Removes exited session entries and records enough context to recover focus.
pub fn remove_exited_sessions(
    app: &mut NexusDemo,
    exited_indices: &[usize],
) -> RemovedExitedSessions {
    let fallback_working_dir = fallback_working_dir(app, exited_indices);
    let removed_active = exited_indices.binary_search(&app.active_index).is_ok();
    let removed_active_chat = removed_active
        && app
            .session_terminals
            .get(app.active_index)
            .is_some_and(|entry| is_chat_session(&entry.session));

    for index in exited_indices.iter().rev() {
        let removed = app.session_terminals.remove(*index);
        if is_chat_session(&removed.session) {
            app.closed_chat_session_ids.insert(removed.session.id);
        }
    }

    RemovedExitedSessions {
        removed_active,
        removed_active_chat,
        fallback_working_dir,
    }
}

/// Returns a working directory to use if a removed chat needs replacement.
fn fallback_working_dir(app: &NexusDemo, exited_indices: &[usize]) -> PathBuf {
    exited_indices
        .iter()
        .find_map(|index| app.session_terminals.get(*index))
        .map(|entry| entry.session.working_dir.clone())
        .or_else(|| {
            app.session_terminals
                .get(app.active_index)
                .map(|entry| entry.session.working_dir.clone())
        })
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}
