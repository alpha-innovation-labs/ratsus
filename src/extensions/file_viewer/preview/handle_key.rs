use crossterm::event::{KeyEvent as CrosstermKeyEvent, KeyEventState};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Handles keyboard input for the selected file preview.
pub fn handle_file_preview_key(
    view: &mut FileSystemTreeView,
    keyboard: &KeyboardEvent,
) -> CoordinatorAction {
    let event = CrosstermKeyEvent {
        code: keyboard.key_code,
        modifiers: keyboard.modifiers,
        kind: keyboard.kind,
        state: KeyEventState::NONE,
    };
    if view.handle_preview_key(event) {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}
