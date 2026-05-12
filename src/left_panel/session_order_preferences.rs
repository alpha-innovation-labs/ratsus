use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Persisted left-panel ordering and collapsed-folder state.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SessionOrderPreferences {
    #[serde(default)]
    pub active_session_id: Option<String>,
    pub session_ids: Vec<String>,
    pub folder_paths: Vec<PathBuf>,
    pub collapsed_folder_paths: Vec<PathBuf>,
}
