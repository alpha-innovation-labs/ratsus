use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Durable metadata for a normal shell terminal shown in the left pane.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PersistedNormalTerminalSession {
    pub date: String,
    pub created_at: String,
    pub title: String,
    pub id: String,
    pub working_dir: PathBuf,
}
