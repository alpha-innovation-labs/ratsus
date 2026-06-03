use std::collections::BTreeMap;

use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Returns grouped folder and plan rows visible under current filter and collapse state.
    pub fn visible_rows(&self) -> Vec<PlanListRow> {
        let indexes_by_folder = visible_plan_indexes_by_folder(self);
        let totals_by_folder = total_plan_counts_by_folder(&self.plans);
        let mut rows = Vec::new();
        for folder in &self.folder_order {
            let indexes = indexes_by_folder.get(folder).cloned().unwrap_or_default();
            rows.push(PlanListRow::Folder {
                path: folder.clone(),
                current_plan_count: indexes.len(),
                total_plan_count: *totals_by_folder.get(folder).unwrap_or(&0),
            });
            if self.collapsed_folders.contains(folder) {
                continue;
            }
            rows.extend(indexes.into_iter().map(|index| PlanListRow::Plan { index }));
        }
        rows
    }
}

/// Groups visible plan indexes by folder while preserving list order.
fn visible_plan_indexes_by_folder(
    state: &PlanListState,
) -> BTreeMap<std::path::PathBuf, Vec<usize>> {
    let query = state.filter_query.to_ascii_lowercase();
    let mut folders = BTreeMap::new();
    for (index, plan) in state.plans.iter().enumerate() {
        if plan_matches_filter(plan, &query) {
            folders
                .entry(plan.folder.clone())
                .or_insert_with(Vec::new)
                .push(index);
        }
    }
    folders
}

/// Counts all plans per folder before filtering.
fn total_plan_counts_by_folder(plans: &[PlanEntry]) -> BTreeMap<std::path::PathBuf, usize> {
    let mut totals = BTreeMap::new();
    for plan in plans {
        *totals.entry(plan.folder.clone()).or_insert(0) += 1;
    }
    totals
}

/// Returns true when a plan matches the already-normalized query.
fn plan_matches_filter(plan: &PlanEntry, query: &str) -> bool {
    query.is_empty() || plan.title.to_ascii_lowercase().contains(query)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use ratatui::layout::Rect;

    use crate::extensions::plans::data::plan_entry::PlanEntry;
    use crate::extensions::plans::data::plan_list_row::PlanListRow;
    use crate::extensions::plans::data::plan_list_state::PlanListState;

    /// Builds plan state with one populated workspace and one empty workspace.
    fn state_with_empty_workspace() -> PlanListState {
        let folder_a = PathBuf::from("/workspace/folder-a");
        let folder_b = PathBuf::from("/workspace/folder-b");
        PlanListState {
            root_path: PathBuf::from("/workspace"),
            plans_dir: PathBuf::from("/workspace/plans"),
            workspace_folders: vec![folder_a.clone(), folder_b.clone()],
            plans: vec![PlanEntry::new(
                folder_a.join("plans/alpha.md"),
                folder_a.clone(),
                "alpha.md".to_string(),
            )],
            collapsed_folders: BTreeSet::new(),
            folder_order: vec![folder_a.clone(), folder_b.clone()],
            focused_row: 0,
            active_index: None,
            scroll: 0,
            filter_query: String::new(),
            filtering: false,
            pending_g: false,
            drag: None,
            last_area: Rect::default(),
            preview_state: None,
            selected_plan_watcher: None,
        }
    }

    /// Verifies workspace folders without Markdown plans remain visible as empty folders.
    #[test]
    fn visible_rows_include_empty_workspace_folders() {
        let rows = state_with_empty_workspace().visible_rows();

        assert!(matches!(
            rows.first(),
            Some(PlanListRow::Folder {
                current_plan_count: 1,
                total_plan_count: 1,
                ..
            })
        ));
        assert!(matches!(rows.get(1), Some(PlanListRow::Plan { index: 0 })));
        assert!(matches!(
            rows.get(2),
            Some(PlanListRow::Folder {
                current_plan_count: 0,
                total_plan_count: 0,
                ..
            })
        ));
    }
}
