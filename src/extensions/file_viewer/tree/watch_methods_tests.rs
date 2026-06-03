use std::fs;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Selected file changes should refresh preview content from disk.
#[test]
fn refreshes_preview_for_changed_selected_file() -> anyhow::Result<()> {
    let root = temp_root("selected_file_refresh")?;
    let file_path = root.join("alpha.txt");
    fs::write(&file_path, "before")?;
    let mut view = FileSystemTreeView::with_root(root.clone())?;
    view.select_path(vec![0, 0]);
    let _ = view.refresh_selection(LeftPaneActionOutcome::Handled);
    fs::write(&file_path, "after")?;

    let changed = view.refresh_preview_for_changed_paths(std::slice::from_ref(&file_path));
    wait_for_file_viewer_load(&mut view);

    assert!(changed);
    assert_eq!(view.code_preview_content(), Some("after"));
    let _ = fs::remove_dir_all(root);
    Ok(())
}

/// Root-level file changes should rebuild visible root rows.
#[test]
fn rebuilds_tree_for_root_level_changes() -> anyhow::Result<()> {
    let root = temp_root("root_tree_refresh")?;
    fs::write(root.join("alpha.txt"), "alpha")?;
    let mut view = FileSystemTreeView::with_root(root.clone())?;
    fs::write(root.join("beta.txt"), "beta")?;

    let changed = view.refresh_tree_for_changed_paths(&[root.join("beta.txt")]);
    wait_for_file_viewer_load(&mut view);

    assert!(changed);
    assert!(view.root_child_names().contains(&"beta.txt".to_string()));
    let _ = fs::remove_dir_all(root);
    Ok(())
}

/// Nested file changes should not rebuild root rows.
#[test]
fn ignores_nested_changes_for_root_rows() -> anyhow::Result<()> {
    let root = temp_root("nested_tree_refresh")?;
    fs::create_dir_all(root.join("nested"))?;
    let nested_file = root.join("nested").join("leaf.txt");
    fs::write(&nested_file, "leaf")?;
    let mut view = FileSystemTreeView::with_root(root.clone())?;

    let changed = view.refresh_tree_for_changed_paths(&[nested_file]);

    assert!(!changed);
    let _ = fs::remove_dir_all(root);
    Ok(())
}

/// Waits briefly for async file-viewer workers to finish in tests.
fn wait_for_file_viewer_load(view: &mut FileSystemTreeView) {
    for _ in 0..50 {
        if view.poll_watchers() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// Creates an isolated temporary tree root for watcher refresh tests.
fn temp_root(name: &str) -> anyhow::Result<std::path::PathBuf> {
    let root = std::env::temp_dir().join(format!(
        "ratsus_{name}_{}_{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    fs::create_dir_all(&root)?;
    Ok(root)
}
