use std::collections::BTreeSet;
use std::path::PathBuf;

use anyhow::Result;
use ratatui::layout::Rect;
use ratkit::services::file_watcher::FileWatcher;

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::plans::data::plan_drag_state::PlanDragState;
use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::fs::load_markdown_plans::load_markdown_plans;
use crate::extensions::plans::fs::load_workspace_markdown_plans::load_workspace_markdown_plans;
use crate::extensions::plans::watch::start_selected_plan_watcher::start_selected_plan_watcher;

/// Stateful plan list hosted in the shared left-pane shell.
pub struct PlanListState {
    pub root_path: PathBuf,
    pub plans_dir: PathBuf,
    pub workspace_folders: Vec<PathBuf>,
    pub plans: Vec<PlanEntry>,
    pub collapsed_folders: BTreeSet<PathBuf>,
    pub folder_order: Vec<PathBuf>,
    pub focused_row: usize,
    pub active_index: Option<usize>,
    pub scroll: usize,
    pub filter_query: String,
    pub filtering: bool,
    pub pending_g: bool,
    pub drag: Option<PlanDragState>,
    pub last_area: Rect,
    pub preview_state: Option<FilePreviewState>,
    pub(crate) selected_plan_watcher: Option<FileWatcher>,
}

impl PlanListState {
    /// Builds plan state from the current working directory.
    pub fn new() -> Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_root(root)
    }

    /// Builds plan state from an explicit project root.
    pub fn with_root(root_path: PathBuf) -> Result<Self> {
        let plans_dir = root_path.join("plans");
        let plans = load_markdown_plans(&plans_dir)?;
        let folder_order = plan_folder_order(&plans);
        let active_index = (!plans.is_empty()).then_some(0);
        let preview_state =
            active_index.map(|index| preview_state_for_path(&plans[index].path, false));
        let selected_plan_watcher =
            active_index.and_then(|index| start_selected_plan_watcher(&plans[index].path));
        Ok(Self {
            root_path,
            plans_dir,
            workspace_folders: Vec::new(),
            plans,
            collapsed_folders: BTreeSet::new(),
            folder_order,
            focused_row: 0,
            active_index,
            scroll: 0,
            filter_query: String::new(),
            filtering: false,
            pending_g: false,
            drag: None,
            last_area: Rect::default(),
            preview_state,
            selected_plan_watcher,
        })
    }

    /// Reloads plans when the app workspace folder list changes.
    pub fn sync_workspace_folders(&mut self, workspace_folders: &[PathBuf]) -> Result<()> {
        if self.workspace_folders == workspace_folders {
            return Ok(());
        }
        self.workspace_folders = workspace_folders.to_vec();
        self.plans = load_workspace_markdown_plans(workspace_folders)?;
        self.folder_order = workspace_folders.to_vec();
        self.focused_row = 0;
        self.active_index = (!self.plans.is_empty()).then_some(0);
        self.preview_state = self
            .active_index
            .map(|index| preview_state_for_path(&self.plans[index].path, false));
        self.selected_plan_watcher = self
            .active_index
            .and_then(|index| start_selected_plan_watcher(&self.plans[index].path));
        Ok(())
    }
}

/// Returns the first-seen folder order for loaded plans.
fn plan_folder_order(plans: &[PlanEntry]) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut folders = Vec::new();
    for plan in plans {
        if seen.insert(plan.folder.clone()) {
            folders.push(plan.folder.clone());
        }
    }
    folders
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::PlanListState;

    /// Plan state should watch only the active plan file when a plan is open.
    #[test]
    fn starts_selected_plan_watcher_only_when_plan_is_open() -> anyhow::Result<()> {
        let root = temp_root("selected-plan-watcher")?;
        let plans_dir = root.join("plans");
        fs::create_dir_all(&plans_dir)?;
        fs::write(plans_dir.join("alpha.md"), "# Alpha")?;

        let state = PlanListState::with_root(root.clone())?;

        assert!(state.selected_plan_watcher.is_some());
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Plan state should not watch anything when no plan file is open.
    #[test]
    fn has_no_plan_watcher_without_open_plan() -> anyhow::Result<()> {
        let root = temp_root("no-plan-watcher")?;

        let state = PlanListState::with_root(root.clone())?;

        assert!(state.selected_plan_watcher.is_none());
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Creates an isolated temporary root for plan-state tests.
    fn temp_root(name: &str) -> anyhow::Result<std::path::PathBuf> {
        let root = std::env::temp_dir().join(format!(
            "ratsus_plan_state_{name}_{}_{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
