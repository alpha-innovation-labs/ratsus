use std::collections::BTreeMap;
use std::path::PathBuf;

use ratkit::primitives::resizable_grid::PaneId;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroup;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::visible_session_rows;

/// Caches the left-panel visible row tree until its row-shaping inputs change.
#[derive(Default)]
pub struct VisibleSessionRowsCache {
    signature: Option<VisibleSessionRowsSignature>,
    rows: Vec<SessionListRow>,
    rebuild_count: usize,
}

impl VisibleSessionRowsCache {
    /// Builds an empty visible rows cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns cached rows, rebuilding only when session, folder, or collapse inputs change.
    pub fn rows(
        &mut self,
        session_terminals: &[SessionTerminal],
        collapsed_folders: &std::collections::BTreeSet<PathBuf>,
        folder_order: &[PathBuf],
        pinned_session_index: Option<usize>,
        split_groups: &SplitPaneSessionGroupState,
        pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
    ) -> &[SessionListRow] {
        let signature = VisibleSessionRowsSignature::new(
            session_terminals,
            collapsed_folders,
            folder_order,
            pinned_session_index,
            split_groups,
            pane_session_bundles,
        );
        if self.signature.as_ref() != Some(&signature) {
            self.rows = visible_session_rows(
                session_terminals,
                collapsed_folders,
                folder_order,
                pinned_session_index,
                split_groups,
                pane_session_bundles,
            );
            self.signature = Some(signature);
            self.rebuild_count += 1;
        }
        &self.rows
    }

    /// Returns the cached visible row count for the current row-shaping inputs.
    pub fn row_count(
        &mut self,
        session_terminals: &[SessionTerminal],
        collapsed_folders: &std::collections::BTreeSet<PathBuf>,
        folder_order: &[PathBuf],
        pinned_session_index: Option<usize>,
        split_groups: &SplitPaneSessionGroupState,
        pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
    ) -> usize {
        self.rows(
            session_terminals,
            collapsed_folders,
            folder_order,
            pinned_session_index,
            split_groups,
            pane_session_bundles,
        )
        .len()
    }

    /// Returns how many times this cache rebuilt its visible rows.
    #[cfg(test)]
    pub fn rebuild_count(&self) -> usize {
        self.rebuild_count
    }
}

/// Identifies the inputs that can change the visible folder/session row tree.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VisibleSessionRowsSignature {
    sessions: Vec<VisibleSessionSignature>,
    collapsed_folders: Vec<PathBuf>,
    folder_order: Vec<PathBuf>,
    pinned_session_index: Option<usize>,
    split_groups: Vec<SplitGroupSignature>,
    pane_session_bundles: Vec<(PaneId, Vec<String>)>,
}

impl VisibleSessionRowsSignature {
    /// Builds a signature from every input that affects visible rows and row indexes.
    fn new(
        session_terminals: &[SessionTerminal],
        collapsed_folders: &std::collections::BTreeSet<PathBuf>,
        folder_order: &[PathBuf],
        pinned_session_index: Option<usize>,
        split_groups: &SplitPaneSessionGroupState,
        pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
    ) -> Self {
        Self {
            sessions: session_terminals
                .iter()
                .map(VisibleSessionSignature::new)
                .collect(),
            collapsed_folders: collapsed_folders.iter().cloned().collect(),
            folder_order: folder_order.to_vec(),
            pinned_session_index,
            split_groups: split_groups
                .groups
                .values()
                .map(SplitGroupSignature::new)
                .collect(),
            pane_session_bundles: pane_session_bundles
                .iter()
                .map(|(pane_id, session_ids)| (*pane_id, session_ids.clone()))
                .collect(),
        }
    }
}

/// Identifies one session's row-affecting data and current vector position.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VisibleSessionSignature {
    id: String,
    working_dir: PathBuf,
    is_running: bool,
}

impl VisibleSessionSignature {
    /// Builds a signature for one session entry.
    fn new(entry: &SessionTerminal) -> Self {
        Self {
            id: entry.session.id.clone(),
            working_dir: entry.session.working_dir.clone(),
            is_running: entry.session.is_running,
        }
    }
}

/// Identifies one split group's row-affecting identity, label, and pane order.
#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitGroupSignature {
    id: u64,
    name: String,
    panes: Vec<PaneId>,
}

impl SplitGroupSignature {
    /// Builds a row cache signature for one split-pane session group.
    fn new(group: &SplitPaneSessionGroup) -> Self {
        Self {
            id: group.id,
            name: group.name.clone(),
            panes: group.panes.clone(),
        }
    }
}
