use ratkit::primitives::resizable_grid::PaneId;

/// One visible session child that belongs to a split-pane group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupedSessionChild {
    pub pane_id: PaneId,
    pub session_index: usize,
}
