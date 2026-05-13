use std::path::PathBuf;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::core::chat_session::ChatSessionKind;

/// Builds one fake chat session metadata value.
pub fn stub_session(
    date: &str,
    title: &str,
    id: &str,
    working_dir: impl Into<PathBuf>,
    is_running: bool,
) -> ChatSession {
    ChatSession::new(date, title, id, working_dir)
        .with_running(is_running)
        .with_kind(ChatSessionKind::Chat)
}
