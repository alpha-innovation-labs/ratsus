use std::collections::{BTreeMap, BTreeSet};

use ratkit::primitives::resizable_grid::types::LayoutNode;
use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::sync_workspace_root::restore_file_viewer_expansion;
use crate::extensions::plans::data::restore_active_plan_path::restore_active_plan_path;
use crate::ui::grid_layout::group::compact_split_pane_session_groups::compact_split_pane_session_groups;
use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroup;
use crate::ui::grid_layout::persistence::load_persisted_multiplexer_state::load_persisted_multiplexer_state;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;
use crate::ui::grid_layout::persistence::resizable_grid_from_persisted::resizable_grid_from_persisted;
use crate::ui::grid_layout::persistence::restore_workspace_state::restore_workspace_state;
use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Restores persisted split-pane multiplexer state into a fully loaded app.
pub fn restore_multiplexer_state_into_app(app: &mut AppState) {
    let Some(persisted) = load_persisted_multiplexer_state() else {
        return;
    };
    restore_persisted_multiplexer_state_into_app(app, persisted);
}

/// Restores one persisted split-pane multiplexer state into a fully loaded app.
pub fn restore_persisted_multiplexer_state_into_app(
    app: &mut AppState,
    persisted: PersistedMultiplexerState,
) {
    let previous_file_tree_root = app.file_system_tree_view.root_path().to_path_buf();
    restore_workspace_state(app, &persisted.workspace);
    app.left_pane_mode = persisted.left_pane_mode;
    app.active_main_pane_tab = persisted.active_main_pane_tab;
    app.selected_expo_folder = persisted.selected_expo_folder.clone();
    if app.left_pane_mode == LeftPaneMode::Plans {
        let _ = app.plan_list.sync_workspace_folders(&app.folder_order);
        if let Some(active_plan_path) = &persisted.active_plan_path {
            let _ = restore_active_plan_path(&mut app.plan_list, active_plan_path);
        }
    }
    if should_restore_file_tree_now(app.left_pane_mode, app.active_main_pane_tab) {
        app.file_system_tree_view
            .sync_workspace_roots(&app.folder_order);
        app.file_system_tree_expanded_paths =
            persisted.file_system_tree.expanded_paths_by_root.clone();
        app.file_system_tree_view.apply_workspace_open_state(
            &persisted.file_system_tree.workspace_expanded_paths,
            &persisted.file_system_tree.workspace_collapsed_paths,
        );
        restore_file_viewer_expansion_for_root(app, previous_file_tree_root);
        if let Some(selected_path) = &persisted.file_system_tree.selected_path {
            let _ = app
                .file_system_tree_view
                .restore_workspace_selected_path(selected_path);
        }
    }
    let Some(layout) = resizable_grid_from_persisted(&persisted.terminal_layout) else {
        return;
    };
    let pane_ids = layout_pane_ids(&layout.nodes);
    let valid_session_ids = app
        .session_terminals
        .iter()
        .map(|entry| entry.session.id.clone())
        .collect::<BTreeSet<_>>();
    let terminal_pane_session_bundles = valid_bundles(
        &persisted.terminal_pane_session_bundles,
        &pane_ids,
        &valid_session_ids,
    );
    if terminal_pane_session_bundles.is_empty() {
        return;
    }
    let terminal_pane_sessions = active_sessions_from_bundles(&terminal_pane_session_bundles);
    let active_terminal_pane_id = restored_active_pane(&persisted, &terminal_pane_sessions);
    let split_pane_session_groups =
        restored_groups(&persisted, &pane_ids, &terminal_pane_session_bundles);

    app.terminal_layout = layout;
    app.terminal_pane_sessions = terminal_pane_sessions;
    app.terminal_pane_session_bundles = terminal_pane_session_bundles;
    app.split_pane_session_groups = split_pane_session_groups;
    app.active_terminal_pane_id = active_terminal_pane_id;
    if let Some(active_index) = restored_active_index(app, &persisted) {
        app.active_index = active_index;
        app.focused_index = active_index;
    }
}

/// Returns whether persisted file-tree paths should be restored during startup.
fn should_restore_file_tree_now(left_mode: LeftPaneMode, main_tab: MainPaneTab) -> bool {
    left_mode == LeftPaneMode::Files || main_tab == MainPaneTab::Files
}

/// Restores expansion for the current root or a previously active saved root.
fn restore_file_viewer_expansion_for_root(app: &mut AppState, previous_root: std::path::PathBuf) {
    if app
        .file_system_tree_expanded_paths
        .contains_key(app.file_system_tree_view.root_path())
    {
        restore_file_viewer_expansion(app);
        return;
    }
    let Some(expanded_paths) = app
        .file_system_tree_expanded_paths
        .get(&previous_root)
        .cloned()
    else {
        restore_file_viewer_expansion(app);
        return;
    };
    let _ = app
        .file_system_tree_view
        .replace_root(previous_root, &expanded_paths);
}

/// Returns every pane id present in a layout tree.
fn layout_pane_ids(nodes: &[LayoutNode]) -> BTreeSet<PaneId> {
    nodes
        .iter()
        .filter_map(|node| match node {
            LayoutNode::Pane { id } => Some(*id),
            LayoutNode::Split { .. } => None,
        })
        .collect()
}

/// Returns pane bundles that reference existing panes and loaded sessions.
fn valid_bundles(
    bundles: &BTreeMap<PaneId, Vec<String>>,
    pane_ids: &BTreeSet<PaneId>,
    valid_session_ids: &BTreeSet<String>,
) -> BTreeMap<PaneId, Vec<String>> {
    bundles
        .iter()
        .filter(|(pane_id, _)| pane_ids.contains(pane_id))
        .filter_map(|(pane_id, session_ids)| {
            let valid_ids = session_ids
                .iter()
                .filter(|session_id| valid_session_ids.contains(*session_id))
                .cloned()
                .collect::<Vec<_>>();
            (!valid_ids.is_empty()).then_some((*pane_id, valid_ids))
        })
        .collect()
}

/// Uses each pane bundle's last session as that pane's active session.
fn active_sessions_from_bundles(
    bundles: &BTreeMap<PaneId, Vec<String>>,
) -> BTreeMap<PaneId, String> {
    bundles
        .iter()
        .filter_map(|(pane_id, session_ids)| {
            session_ids
                .last()
                .map(|session_id| (*pane_id, session_id.clone()))
        })
        .collect()
}

/// Returns the restored active pane, falling back to the first restored pane.
fn restored_active_pane(
    persisted: &PersistedMultiplexerState,
    terminal_pane_sessions: &BTreeMap<PaneId, String>,
) -> PaneId {
    if terminal_pane_sessions.contains_key(&persisted.active_terminal_pane_id) {
        return persisted.active_terminal_pane_id;
    }
    terminal_pane_sessions
        .keys()
        .next()
        .copied()
        .unwrap_or(TERMINAL_PANE_ID)
}

/// Returns restored split groups filtered to existing panes and valid sessions.
fn restored_groups(
    persisted: &PersistedMultiplexerState,
    pane_ids: &BTreeSet<PaneId>,
    terminal_pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
) -> crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState {
    let mut state = persisted.split_pane_session_groups.clone();
    state.groups = state
        .groups
        .into_iter()
        .filter_map(|(group_id, group)| restored_group(group_id, group, pane_ids))
        .collect();
    compact_split_pane_session_groups(&mut state, terminal_pane_session_bundles);
    state
}

/// Returns one restored group after dropping panes that no longer exist.
fn restored_group(
    group_id: u64,
    group: SplitPaneSessionGroup,
    pane_ids: &BTreeSet<PaneId>,
) -> Option<(u64, SplitPaneSessionGroup)> {
    let panes = group
        .panes
        .into_iter()
        .filter(|pane_id| pane_ids.contains(pane_id))
        .collect::<Vec<_>>();
    (!panes.is_empty()).then_some((group_id, SplitPaneSessionGroup { panes, ..group }))
}

/// Returns the restored active session index when its session still exists.
fn restored_active_index(app: &AppState, persisted: &PersistedMultiplexerState) -> Option<usize> {
    persisted
        .active_session_id
        .as_deref()
        .and_then(|session_id| {
            app.session_terminals
                .iter()
                .position(|entry| entry.session.id == session_id)
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use uuid::Uuid;

    use super::restore_persisted_multiplexer_state_into_app;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
    use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
    use crate::ui::grid_layout::persistence::capture_multiplexer_state::capture_multiplexer_state;
    use crate::ui::left_panel::action::LeftPaneAction;
    use crate::ui::left_panel::content::LeftPaneContent;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

    /// Restoring multiplexer state should restore saved file-viewer expansion state.
    #[test]
    fn restores_file_viewer_expanded_folders() -> anyhow::Result<()> {
        let root = temp_workspace()?;
        let nested = root.join("nested");
        fs::create_dir_all(&nested)?;
        let mut app = app_fixture(Vec::new())?;
        app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;
        let mut persisted = capture_multiplexer_state(&app);
        persisted.left_pane_mode = LeftPaneMode::Files;
        persisted.active_main_pane_tab = MainPaneTab::Chat;
        persisted
            .file_system_tree
            .expanded_paths_by_root
            .insert(root.clone(), vec![root.clone(), nested.clone()]);
        persisted.file_system_tree.workspace_expanded_paths = vec![nested.clone()];
        persisted.file_system_tree.workspace_collapsed_paths = vec![root.clone()];

        restore_persisted_multiplexer_state_into_app(&mut app, persisted);

        assert!(app
            .file_system_tree_expanded_paths
            .get(&root)
            .is_some_and(|paths| paths.contains(&nested)));
        assert!(app
            .file_system_tree_view
            .workspace_expanded_directory_paths()
            .contains(&nested));
        assert!(app
            .file_system_tree_view
            .workspace_collapsed_directory_paths()
            .contains(&root));
        assert!(app
            .file_system_tree_view
            .expanded_directory_paths()
            .contains(&nested));
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Restoring multiplexer state should restore file folder open and closed state changed through left-pane actions.
    #[test]
    fn restores_file_folder_open_state_from_left_pane_actions() -> anyhow::Result<()> {
        let root = temp_workspace()?;
        let nested = root.join("nested");
        fs::create_dir_all(&nested)?;
        let mut app = app_fixture(Vec::new())?;
        app.folder_order = vec![root.clone()];
        app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;
        app.file_system_tree_view
            .sync_workspace_roots(&app.folder_order);
        app.file_system_tree_view.select_workspace_row(1);
        app.file_system_tree_view
            .handle_left_pane_action(LeftPaneAction::Expand);
        app.file_system_tree_view.select_workspace_row(0);
        app.file_system_tree_view
            .handle_left_pane_action(LeftPaneAction::Collapse);
        let mut persisted = capture_multiplexer_state(&app);
        persisted.left_pane_mode = LeftPaneMode::Files;
        persisted.active_main_pane_tab = MainPaneTab::Chat;
        let mut restored_app = app_fixture(Vec::new())?;
        restored_app.folder_order = vec![root.clone()];
        restored_app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;

        restore_persisted_multiplexer_state_into_app(&mut restored_app, persisted);

        assert!(restored_app
            .file_system_tree_view
            .workspace_expanded_directory_paths()
            .contains(&nested));
        assert!(restored_app
            .file_system_tree_view
            .workspace_collapsed_directory_paths()
            .contains(&root));
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Restoring multiplexer state should restore workspace mode and known folder order.
    #[test]
    fn restores_workspace_mode_and_order() -> anyhow::Result<()> {
        let mut app = app_fixture(Vec::new())?;
        app.folder_order = vec!["/workspace/a".into(), "/workspace/b".into()];
        let mut persisted = capture_multiplexer_state(&app);
        persisted.workspace.workspace_view_enabled = false;
        persisted.workspace.workspace_order = vec!["/workspace/b".into(), "/workspace/a".into()];
        persisted.workspace.selected_workspace_path = Some("/workspace/b".into());

        restore_persisted_multiplexer_state_into_app(&mut app, persisted);

        assert!(!app.workspace_view_enabled);
        assert_eq!(
            app.folder_order,
            vec![
                std::path::PathBuf::from("/workspace/b"),
                std::path::PathBuf::from("/workspace/a")
            ]
        );
        assert_eq!(app.selected_workspace_path, Some("/workspace/b".into()));
        Ok(())
    }

    /// Restoring non-file and non-plan modes should not eagerly sync workspace file or plan state.
    #[test]
    fn skips_workspace_file_and_plan_sync_outside_their_modes() -> anyhow::Result<()> {
        let root = temp_workspace()?;
        fs::create_dir_all(root.join("plans"))?;
        fs::write(root.join("plans/alpha.md"), "# Alpha")?;
        fs::write(root.join("visible.txt"), "visible")?;
        let mut app = app_fixture(Vec::new())?;
        let original_file_roots = app.file_system_tree_view.workspace_roots.clone();
        let mut persisted = capture_multiplexer_state(&app);
        persisted.left_pane_mode = LeftPaneMode::Sessions;
        persisted.active_main_pane_tab = MainPaneTab::Chat;
        persisted.workspace.workspace_order = vec![root.clone()];
        persisted.active_plan_path = Some(root.join("plans/alpha.md"));
        persisted.file_system_tree.selected_path = Some(root.join("visible.txt"));

        restore_persisted_multiplexer_state_into_app(&mut app, persisted);

        assert!(app.plan_list.workspace_folders.is_empty());
        assert!(!app.file_system_tree_view.workspace_roots.contains(&root));
        assert!(!original_file_roots.contains(&root));
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Restoring multiplexer state should restore focused file selections when files are open.
    #[test]
    fn restores_focused_file_path_when_files_are_open() -> anyhow::Result<()> {
        let root = temp_workspace()?;
        let file_path = root.join("visible.txt");
        fs::write(&file_path, "visible")?;
        fs::create_dir_all(root.join("plans"))?;
        let plan_path = root.join("plans/alpha.md");
        fs::write(&plan_path, "# Alpha")?;
        let mut app = app_fixture(Vec::new())?;
        app.folder_order = vec![root.clone()];
        app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;
        let mut persisted = capture_multiplexer_state(&app);
        persisted.workspace.workspace_order = vec![root.clone()];
        persisted.workspace.selected_workspace_path = Some(root.clone());
        persisted.left_pane_mode = LeftPaneMode::Files;
        persisted.active_main_pane_tab = MainPaneTab::Files;
        persisted.selected_expo_folder = Some(root.clone());
        persisted.active_plan_path = Some(plan_path.clone());
        persisted.file_system_tree.selected_path = Some(file_path.clone());

        restore_persisted_multiplexer_state_into_app(&mut app, persisted);

        assert_eq!(app.left_pane_mode, LeftPaneMode::Files);
        assert_eq!(app.active_main_pane_tab, MainPaneTab::Files);
        assert_eq!(app.selected_expo_folder, Some(root.clone()));
        assert!(app.plan_list.active_plan().is_none());
        assert_eq!(
            app.file_system_tree_view.workspace_selected_status(),
            file_path.display().to_string()
        );
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Creates a test-owned workspace directory.
    fn temp_workspace() -> anyhow::Result<std::path::PathBuf> {
        let root = std::env::temp_dir().join(format!(
            "ratsus_restore_file_tree_{}_{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
