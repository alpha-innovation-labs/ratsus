use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::app_state::AppState;
use crate::keyboard::list_key_behavior::ListKeyBehavior;
use crate::keyboard::list_key_outcome::ListKeyOutcome;
use crate::left_panel::focus_adjacent_folder::focus_adjacent_folder;
use crate::left_panel::left_panel_key_behavior::LeftPanelKeyBehavior;
use crate::main_pane::handle_file_system_tree_key::handle_file_system_tree_key;
use crate::main_pane::main_pane_tab::MainPaneTab;

/// Handles keyboard input while the left session pane is focused.
pub fn handle_left_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    if app.active_main_pane_tab == MainPaneTab::Files {
        return Ok(handle_file_system_tree_key(
            &mut app.file_system_tree_view,
            &keyboard,
        ));
    }

    if let Some(direction) = folder_jump_direction(&keyboard) {
        focus_adjacent_folder(app, direction);
        return Ok(CoordinatorAction::Redraw);
    }

    let mut behavior = LeftPanelKeyBehavior::new(app);
    Ok(coordinator_action_for_list_outcome(
        behavior.handle_list_keyboard(keyboard),
    ))
}

/// Returns folder-jump direction for Shift+J and Shift+K shortcuts.
fn folder_jump_direction(keyboard: &KeyboardEvent) -> Option<isize> {
    match keyboard.key_code {
        KeyCode::Char('J') => Some(1),
        KeyCode::Char('K') => Some(-1),
        KeyCode::Char('j') if keyboard.modifiers.contains(KeyModifiers::SHIFT) => Some(1),
        KeyCode::Char('k') if keyboard.modifiers.contains(KeyModifiers::SHIFT) => Some(-1),
        _ => None,
    }
}

/// Converts shared list-key outcomes into coordinator actions.
fn coordinator_action_for_list_outcome(outcome: ListKeyOutcome) -> CoordinatorAction {
    match outcome {
        ListKeyOutcome::Handled => CoordinatorAction::Redraw,
        ListKeyOutcome::Continue => CoordinatorAction::Continue,
        ListKeyOutcome::Quit => CoordinatorAction::Quit,
    }
}
