use std::path::PathBuf;

/// One visible row in the grouped plan tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanListRow {
    /// Folder containing one or more visible Markdown plans.
    Folder {
        path: PathBuf,
        current_plan_count: usize,
        total_plan_count: usize,
    },
    /// Markdown plan entry by index in `PlanListState::plans`.
    Plan { index: usize },
}
