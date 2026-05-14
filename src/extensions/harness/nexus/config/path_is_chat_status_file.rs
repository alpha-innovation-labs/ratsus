use std::path::Path;

use crate::extensions::harness::nexus::config::chat_status_file_path::nexus_chat_status_file_path;

/// Returns whether a watched path is the Nexus chat status file.
pub fn path_is_chat_status_file(path: &Path) -> bool {
    let status_path = nexus_chat_status_file_path();
    path == status_path || canonical_paths_match(path, &status_path)
}

/// Returns whether two existing paths resolve to the same filesystem target.
fn canonical_paths_match(left: &Path, right: &Path) -> bool {
    let Ok(left) = left.canonicalize() else {
        return false;
    };
    let Ok(right) = right.canonicalize() else {
        return false;
    };
    left == right
}

#[cfg(test)]
mod tests {
    use super::path_is_chat_status_file;
    use crate::extensions::harness::nexus::config::chat_status_file_path::nexus_chat_status_file_path;

    /// Verifies the configured chat status path is recognized as a session update source.
    #[test]
    fn detects_configured_chat_status_path() {
        assert!(path_is_chat_status_file(&nexus_chat_status_file_path()));
    }

    /// Verifies unrelated paths do not trigger session refreshes.
    #[test]
    fn rejects_unrelated_path() {
        assert!(!path_is_chat_status_file(std::path::Path::new(
            "/tmp/unrelated-chat-status"
        )));
    }
}
