use std::path::Path;

use uuid::Uuid;

use crate::extensions::harness::core::chat_session::{ChatSession, ChatSessionKind};

/// Creates metadata for a normal shell terminal session.
pub fn normal_terminal_session_info(working_dir: &Path) -> ChatSession {
    ChatSession::new(
        "now",
        "Terminal",
        format!("terminal-{}", Uuid::new_v4()),
        working_dir.to_path_buf(),
    )
    .with_kind(ChatSessionKind::NormalTerminal)
}

#[cfg(test)]
mod tests {
    use super::normal_terminal_session_info;

    /// Normal terminal metadata should be identifiable and grouped by working directory.
    #[test]
    fn builds_normal_terminal_metadata() {
        let session = normal_terminal_session_info("/tmp/project".as_ref());

        assert_eq!(session.date, "now");
        assert_eq!(session.title, "Terminal");
        assert!(session.id.starts_with("terminal-"));
        assert_eq!(
            session.working_dir,
            std::path::PathBuf::from("/tmp/project")
        );
    }
}
