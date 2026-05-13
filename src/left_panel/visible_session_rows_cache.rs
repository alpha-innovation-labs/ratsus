use std::path::PathBuf;

use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::visible_session_rows::visible_session_rows;
use crate::terminal::session_terminal::SessionTerminal;

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
    ) -> &[SessionListRow] {
        let signature = VisibleSessionRowsSignature::new(
            session_terminals,
            collapsed_folders,
            folder_order,
            pinned_session_index,
        );
        if self.signature.as_ref() != Some(&signature) {
            self.rows = visible_session_rows(
                session_terminals,
                collapsed_folders,
                folder_order,
                pinned_session_index,
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
    ) -> usize {
        self.rows(
            session_terminals,
            collapsed_folders,
            folder_order,
            pinned_session_index,
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
}

impl VisibleSessionRowsSignature {
    /// Builds a signature from every input that affects visible rows and row indexes.
    fn new(
        session_terminals: &[SessionTerminal],
        collapsed_folders: &std::collections::BTreeSet<PathBuf>,
        folder_order: &[PathBuf],
        pinned_session_index: Option<usize>,
    ) -> Self {
        Self {
            sessions: session_terminals
                .iter()
                .map(VisibleSessionSignature::new)
                .collect(),
            collapsed_folders: collapsed_folders.iter().cloned().collect(),
            folder_order: folder_order.to_vec(),
            pinned_session_index,
        }
    }
}

/// Identifies one session's row-affecting data and current vector position.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VisibleSessionSignature {
    id: String,
    working_dir: PathBuf,
}

impl VisibleSessionSignature {
    /// Builds a signature for one session entry.
    fn new(entry: &SessionTerminal) -> Self {
        Self {
            id: entry.session.id.clone(),
            working_dir: entry.session.working_dir.clone(),
        }
    }
}
