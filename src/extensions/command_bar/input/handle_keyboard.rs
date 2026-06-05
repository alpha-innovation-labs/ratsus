use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::input::hotkeys::app_hotkey::AppHotkey;
use crate::app::state::app_state::AppState;
use crate::extensions::command_bar::actions::close::close_command_bar;
use crate::extensions::command_bar::command::app_hotkey_for_command::app_hotkey_for_command;
use crate::extensions::command_bar::input::key_behavior::CommandBarKeyBehavior;
use crate::ui::keyboard::list::behavior::ListKeyBehavior;
use crate::ui::keyboard::list::outcome::ListKeyOutcome;

/// Result of routing one keyboard event through the command bar.
pub struct CommandBarKeyboardOutcome {
    pub action: CoordinatorAction,
    pub hotkey: Option<AppHotkey>,
}

/// Handles keyboard input while the command bar modal is open.
pub fn handle_command_bar_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> CommandBarKeyboardOutcome {
    let mut behavior = CommandBarKeyBehavior::new(&mut app.command_bar);
    let list_outcome = behavior.handle_list_keyboard(keyboard);
    if let Some(command_id) = behavior.take_activated_command() {
        close_command_bar(app);
        return CommandBarKeyboardOutcome {
            action: CoordinatorAction::Redraw,
            hotkey: Some(app_hotkey_for_command(command_id)),
        };
    }
    command_bar_keyboard_outcome_for_list_outcome(app, list_outcome)
}

/// Converts shared list-key outcomes into command bar outcomes.
fn command_bar_keyboard_outcome_for_list_outcome(
    app: &mut AppState,
    outcome: ListKeyOutcome,
) -> CommandBarKeyboardOutcome {
    let action = match outcome {
        ListKeyOutcome::Handled => CoordinatorAction::Redraw,
        ListKeyOutcome::Continue => CoordinatorAction::Continue,
        ListKeyOutcome::Quit => {
            close_command_bar(app);
            CoordinatorAction::Redraw
        }
    };
    CommandBarKeyboardOutcome {
        action,
        hotkey: None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::{CoordinatorAction, KeyboardEvent};

    use super::handle_command_bar_keyboard;
    use crate::app::input::hotkeys::app_hotkey::AppHotkey;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::extensions::command_bar::data::filtered_commands::command_bar_items;

    /// Enter executes the selected command and closes the command bar.
    #[test]
    fn enter_executes_selected_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Enter));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(outcome.hotkey, Some(AppHotkey::OpenHistoryModal));
        assert!(!app.command_bar.is_open);
    }

    /// Enter executes the workspace-view toggle command without a keyboard shortcut.
    #[test]
    fn enter_executes_workspace_view_toggle_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.query = "toggle workspace".to_string();

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Enter));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(outcome.hotkey, Some(AppHotkey::ToggleWorkspaceView));
        assert!(!app.command_bar.is_open);
    }

    /// Enter executes a workspace selection command and closes the command bar.
    #[test]
    fn enter_executes_workspace_selection_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.query = "select workspace 2".to_string();

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Enter));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(outcome.hotkey, Some(AppHotkey::SelectWorkspace(1)));
        assert!(!app.command_bar.is_open);
    }

    /// Enter executes the Ghostty setup command and closes the command bar.
    #[test]
    fn enter_executes_ghostty_setup_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.query = "setup ghostty".to_string();

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Enter));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(outcome.hotkey, Some(AppHotkey::SetupGhosttyConfig));
        assert!(!app.command_bar.is_open);
    }

    /// Escape clears the active filter without closing the command bar.
    #[test]
    fn escape_clears_filter_without_closing() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.is_filtering = true;
        app.command_bar.query = "split".to_string();
        app.command_bar.selected_position = 3;

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Esc));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(outcome.hotkey, None);
        assert!(app.command_bar.is_open);
        assert!(!app.command_bar.is_filtering);
        assert!(app.command_bar.query.is_empty());
        assert_eq!(app.command_bar.selected_position, 0);
    }

    /// Escape closes the command bar when filter mode is already inactive.
    #[test]
    fn escape_closes_without_filtering() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.is_filtering = false;

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Esc));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(outcome.hotkey, None);
        assert!(!app.command_bar.is_open);
    }

    /// Ctrl+P wraps from the first command to the last command.
    #[test]
    fn control_p_wraps_to_last_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.is_filtering = true;
        app.command_bar.selected_position = 0;

        let outcome = handle_command_bar_keyboard(&mut app, control_key('p'));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(
            app.command_bar.selected_position,
            command_bar_items().len().saturating_sub(1)
        );
    }

    /// Plain j wraps from the last command to the first command when not filtering.
    #[test]
    fn j_wraps_to_first_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.is_filtering = false;
        app.command_bar.selected_position = command_bar_items().len().saturating_sub(1);

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Char('j')));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(app.command_bar.selected_position, 0);
    }

    /// Plain k wraps from the first command to the last command when not filtering.
    #[test]
    fn k_wraps_to_last_command() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.is_filtering = false;
        app.command_bar.selected_position = 0;

        let outcome = handle_command_bar_keyboard(&mut app, key(KeyCode::Char('k')));

        assert_eq!(outcome.action, CoordinatorAction::Redraw);
        assert_eq!(
            app.command_bar.selected_position,
            command_bar_items().len().saturating_sub(1)
        );
    }

    /// Builds a control-modified key press for command bar tests.
    fn control_key(character: char) -> KeyboardEvent {
        KeyboardEvent {
            key_code: KeyCode::Char(character),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
        }
    }

    /// Builds a plain key press for command bar tests.
    fn key(key_code: KeyCode) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
        }
    }
}
