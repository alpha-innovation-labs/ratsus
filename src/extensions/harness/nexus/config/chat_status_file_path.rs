use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

static NEXUS_CHAT_STATUS_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Returns the cached Nexus chat status file path.
pub fn nexus_chat_status_file_path() -> PathBuf {
    NEXUS_CHAT_STATUS_FILE_PATH
        .get_or_init(resolve_nexus_chat_status_file_path)
        .clone()
}

/// Resolves the Nexus chat status file path from env, Nexus CLI, or the default user data path.
fn resolve_nexus_chat_status_file_path() -> PathBuf {
    if let Ok(path) = std::env::var("NEXUS_CHAT_STATUS_FILE") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }

    if let Ok(output) = Command::new("nexus")
        .arg("--chat-status-file-location")
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return PathBuf::from(path);
            }
        }
    }

    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local/share/nexus/agent/chat-status")
}
