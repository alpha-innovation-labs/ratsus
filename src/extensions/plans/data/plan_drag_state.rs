/// Tracks an active plan drag operation by current plan index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanDragState {
    pub source_index: usize,
    pub current_index: usize,
}

impl PlanDragState {
    /// Creates drag state for the plan index where the drag started.
    pub fn new(source_index: usize) -> Self {
        Self {
            source_index,
            current_index: source_index,
        }
    }
}
