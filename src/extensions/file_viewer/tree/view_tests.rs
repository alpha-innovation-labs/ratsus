use std::fs;
use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::input::dispatch_left_pane_keyboard::dispatch_left_pane_keyboard;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Verifies file-tree content uses the shared left-pane keyboard dispatcher.
#[test]
fn file_tree_uses_shared_left_pane_dispatcher() {
    let root = create_test_tree_root("file_tree_shared_dispatcher");
    let mut view = FileSystemTreeView::with_root(root.clone()).expect("file tree view");

    let first = view.workspace_selected_status();
    let _ = dispatch_left_pane_keyboard(&mut view, key(KeyCode::Char('l')));
    let _ = dispatch_left_pane_keyboard(&mut view, key(KeyCode::Char('j')));

    assert_ne!(view.workspace_selected_status(), first);
    let _ = fs::remove_dir_all(root);
}

/// Verifies file-tree filtering is driven by semantic left-pane filter actions.
#[test]
fn file_tree_filtering_uses_shared_left_pane_dispatcher() {
    let root = create_test_tree_root("file_tree_shared_filtering");
    let mut view = FileSystemTreeView::with_root(root.clone()).expect("file tree view");

    let _ = dispatch_left_pane_keyboard(&mut view, key(KeyCode::Char('/')));
    let _ = dispatch_left_pane_keyboard(&mut view, key(KeyCode::Char('j')));
    let outcome = dispatch_left_pane_keyboard(&mut view, key(KeyCode::Esc));

    assert_eq!(outcome, LeftPaneActionOutcome::Handled);
    assert!(!view.workspace_filtering);
    let _ = fs::remove_dir_all(root);
}

/// Creates a deterministic temporary file tree for dispatcher tests.
fn create_test_tree_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("ratsus_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("alpha")).expect("alpha dir");
    fs::write(root.join("beta.txt"), "beta").expect("beta file");
    root
}

/// Builds a key press event without modifiers.
fn key(key_code: KeyCode) -> KeyboardEvent {
    KeyboardEvent {
        key_code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
    }
}
