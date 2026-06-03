use std::path::PathBuf;

use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::plans::watch::start_selected_plan_watcher::start_selected_plan_watcher;

impl PlanListState {
    /// Polls the active plan-file watcher and refreshes the open preview.
    pub(crate) fn poll_watchers(&mut self) -> bool {
        self.poll_selected_plan_watcher()
    }

    /// Reloads the active plan preview when its file changes externally.
    pub(crate) fn refresh_active_preview_for_changed_paths(
        &mut self,
        changed_paths: &[PathBuf],
    ) -> bool {
        let Some(active_path) = self.active_plan().map(|plan| plan.path.clone()) else {
            return false;
        };
        if !changed_paths
            .iter()
            .any(|changed_path| changed_path == &active_path)
        {
            return false;
        }
        self.preview_state = Some(preview_state_for_path(&active_path, false));
        self.selected_plan_watcher = start_selected_plan_watcher(&active_path);
        true
    }

    /// Polls the active plan file watcher and reloads preview content.
    fn poll_selected_plan_watcher(&mut self) -> bool {
        let Some(watcher) = self.selected_plan_watcher.as_mut() else {
            return false;
        };
        if !watcher.check_for_changes() {
            return false;
        }
        let changed_paths = watcher.get_changed_paths();
        self.refresh_active_preview_for_changed_paths(&changed_paths)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::extensions::plans::data::plan_list_state::PlanListState;

    /// Active plan file changes should refresh the existing preview selection.
    #[test]
    fn refreshes_active_plan_preview_for_external_content_changes() -> anyhow::Result<()> {
        let root = temp_root("plan-preview")?;
        let plans_dir = root.join("plans");
        fs::create_dir_all(&plans_dir)?;
        let plan_path = plans_dir.join("alpha.md");
        fs::write(&plan_path, "# Before")?;
        let mut state = PlanListState::with_root(root.clone())?;
        fs::write(&plan_path, "# After")?;

        let changed =
            state.refresh_active_preview_for_changed_paths(std::slice::from_ref(&plan_path));

        assert!(changed);
        assert!(state.preview_state.is_some());
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Returns a unique temporary root for plan watcher tests.
    fn temp_root(label: &str) -> anyhow::Result<std::path::PathBuf> {
        let root = std::env::temp_dir().join(format!("ratsus-{label}-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
