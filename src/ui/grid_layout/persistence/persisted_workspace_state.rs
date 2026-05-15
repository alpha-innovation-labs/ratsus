use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Persisted workspace-pane mode and ordering owned by the app.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedWorkspaceState {
    #[serde(default = "default_workspace_view_enabled")]
    pub workspace_view_enabled: bool,
    #[serde(default)]
    pub workspace_order: Vec<PathBuf>,
    #[serde(default)]
    pub selected_workspace_path: Option<PathBuf>,
}

impl Default for PersistedWorkspaceState {
    /// Builds default workspace state for old multiplexer files.
    fn default() -> Self {
        Self {
            workspace_view_enabled: default_workspace_view_enabled(),
            workspace_order: Vec::new(),
            selected_workspace_path: None,
        }
    }
}

/// Returns the default workspace-view mode for old persisted state files.
fn default_workspace_view_enabled() -> bool {
    true
}
