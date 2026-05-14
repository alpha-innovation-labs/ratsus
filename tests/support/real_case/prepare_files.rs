use std::fs;
use std::path::Path;

use ratsus::app::state::app_state::AppState;
use ratsus::extensions::file_viewer::tabs::tab::MainPaneTab;
use ratsus::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Creates a real temporary file tree for Files and Diff workflows.
pub fn prepare_files(app: &mut AppState, home: &Path) -> anyhow::Result<()> {
    let root = home.join("workspace");
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("README.md"), "# Project Notes\n\nPreview body\n")?;
    fs::write(
        root.join("src/main.rs"),
        "fn main() { println!(\"ratsus\"); }\n",
    )?;
    app.active_main_pane_tab = MainPaneTab::Files;
    app.file_system_tree_view = FileSystemTreeView::with_root(root)?;
    Ok(())
}
