use crossterm::event::KeyCode;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::update_file_system_tree_selection::update_file_system_tree_selection;

/// Handles file-system tree keyboard input using the Ratkit demo behavior.
pub fn handle_file_system_tree_key(
    view: &mut FileSystemTreeView,
    keyboard: &KeyboardEvent,
) -> CoordinatorAction {
    match keyboard.key_code {
        KeyCode::Char('q') => return CoordinatorAction::Quit,
        KeyCode::Down
        | KeyCode::Up
        | KeyCode::Char('j')
        | KeyCode::Char('k')
        | KeyCode::Enter
        | KeyCode::Left
        | KeyCode::Right
        | KeyCode::Char('h')
        | KeyCode::Char('l') => {
            let _ = view
                .tree
                .handle_navigation_key(keyboard.key_code, &mut view.state);
        }
        KeyCode::Char('/') => {
            if !view.tree.is_filter_mode(&view.state) {
                view.tree.enter_filter_mode(&mut view.state);
            }
        }
        _ => {
            if view.tree.is_filter_mode(&view.state) {
                let _ = view
                    .tree
                    .handle_filter_key(keyboard.key_code, &mut view.state);
            }
        }
    }

    update_file_system_tree_selection(view);
    CoordinatorAction::Redraw
}
