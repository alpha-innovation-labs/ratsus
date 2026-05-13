use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::persisted_normal_terminal_session::PersistedNormalTerminalSession;

/// Builds in-memory session metadata from a persisted normal terminal entry.
pub fn nexus_session_from_persisted_normal_terminal_session(
    entry: PersistedNormalTerminalSession,
) -> NexusSession {
    NexusSession::new_with_created(
        entry.date,
        entry.created_at,
        entry.title,
        entry.id,
        entry.working_dir,
    )
}
