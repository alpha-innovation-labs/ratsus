/// Tracks an in-progress left-pane session drag operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionDragState {
    pub source_index: usize,
    pub current_index: usize,
}

impl SessionDragState {
    /// Creates drag state for the selected source session index.
    pub fn new(source_index: usize) -> Self {
        Self {
            source_index,
            current_index: source_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SessionDragState;

    /// Verifies drag state starts and remains over its source row.
    #[test]
    fn starts_at_source_index() {
        assert_eq!(
            SessionDragState::new(2),
            SessionDragState {
                source_index: 2,
                current_index: 2
            }
        );
    }
}
