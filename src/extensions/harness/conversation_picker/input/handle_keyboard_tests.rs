use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::test_support::app_fixture::app_fixture;
use crate::extensions::harness::conversation_picker::input::handle_keyboard::handle_conversation_picker_keyboard;

/// Verifies Tab toggles the picker from workspace scope to all scope.
#[test]
fn tab_toggles_workspace_scope_to_all() -> anyhow::Result<()> {
    let mut app = app_fixture(Vec::new())?;
    app.selected_workspace_path = Some(PathBuf::from("/workspace/beta"));
    app.conversation_picker.folder_filter = Some(PathBuf::from("/workspace/beta"));

    let action = handle_conversation_picker_keyboard(&mut app, key(KeyCode::Tab));

    assert_eq!(action, CoordinatorAction::Redraw);
    assert_eq!(app.conversation_picker.folder_filter, None);
    Ok(())
}

/// Builds a keyboard event for picker input tests.
fn key(key_code: KeyCode) -> KeyboardEvent {
    KeyboardEvent {
        key_code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
    }
}
