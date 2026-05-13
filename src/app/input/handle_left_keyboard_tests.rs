use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::input::handle_left_keyboard::handle_left_keyboard;
use crate::app::test_support::app_fixture::app_fixture;
use crate::app::test_support::dormant_session::dormant_session;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;

/// Verifies chat left-pane filtering uses the shared semantic dispatcher.
#[test]
fn chat_filter_shortcut_uses_shared_left_pane_dispatcher() {
    let mut app =
        app_fixture(vec![dormant_session("one", "one", "/tmp/project-one")]).expect("app fixture");
    app.active_main_pane_tab = MainPaneTab::Chat;

    let outcome = handle_left_keyboard(&mut app, key(KeyCode::Char('/'))).expect("left keyboard");

    assert_eq!(outcome, CoordinatorAction::Redraw);
    assert!(app.conversation_picker.is_open);
}

/// Verifies chat folder jumps are routed as semantic left-pane actions.
#[test]
fn chat_folder_jump_uses_shared_left_pane_dispatcher() {
    let mut app = app_fixture(vec![
        dormant_session("one", "one", "/tmp/project-one"),
        dormant_session("two", "two", "/tmp/project-two"),
    ])
    .expect("app fixture");
    app.active_main_pane_tab = MainPaneTab::Chat;
    app.folder_order = vec![
        PathBuf::from("/tmp/project-one"),
        PathBuf::from("/tmp/project-two"),
    ];
    app.focused_row = 1;

    let outcome = handle_left_keyboard(&mut app, key(KeyCode::Char('J'))).expect("left keyboard");

    assert_eq!(outcome, CoordinatorAction::Redraw);
    assert_eq!(app.active_main_pane_tab, MainPaneTab::Expo);
    assert!(app.selected_expo_folder.is_some());
}

/// Builds a key press event without modifiers.
fn key(key_code: KeyCode) -> KeyboardEvent {
    KeyboardEvent {
        key_code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
    }
}
