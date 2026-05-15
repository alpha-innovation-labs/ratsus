use std::collections::BTreeMap;

use ratkit::primitives::resizable_grid::PaneId;
use serde::{Deserialize, Serialize};

use crate::extensions::file_viewer::tree::persisted_file_system_tree_state::PersistedFileSystemTreeState;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::grid_layout::persistence::persisted_resizable_grid::PersistedResizableGrid;
use crate::ui::grid_layout::persistence::persisted_workspace_state::PersistedWorkspaceState;

/// Versioned on-disk state for restoring Nexus split-pane multiplexer groups.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedMultiplexerState {
    pub version: u8,
    pub terminal_layout: PersistedResizableGrid,
    pub terminal_pane_sessions: BTreeMap<PaneId, String>,
    pub terminal_pane_session_bundles: BTreeMap<PaneId, Vec<String>>,
    pub split_pane_session_groups: SplitPaneSessionGroupState,
    pub active_terminal_pane_id: PaneId,
    pub active_session_id: Option<String>,
    #[serde(default)]
    pub file_system_tree: PersistedFileSystemTreeState,
    #[serde(default)]
    pub workspace: PersistedWorkspaceState,
}
