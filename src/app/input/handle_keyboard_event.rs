use crossterm::event::KeyCode;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::deletion::handle_delete_session_confirmation_keyboard::handle_delete_session_confirmation_keyboard;
use crate::app::deletion::open_delete_session_confirmation::open_delete_session_confirmation;
use crate::app::expo::hide_expo::hide_expo;
use crate::app::expo::open_expo_for_focused_conversation::open_expo_for_focused_conversation;
use crate::app::focus::toggle_focused_pane::toggle_focused_pane;
use crate::app::input::handle_left_keyboard::handle_left_keyboard;
use crate::app::input::handle_terminal_keyboard::handle_terminal_keyboard;
use crate::app::input::hotkeys::app_hotkey::AppHotkey;
use crate::app::input::hotkeys::normal_terminal_passthrough_hotkey::normal_terminal_passthrough_hotkey;
use crate::app::input::hotkeys::resolve_app_hotkey::resolve_app_hotkey;
use crate::app::input::hotkeys::scopes::active_hotkey_scope;
use crate::app::navigation::cycle_session_in_left_pane_order::cycle_session_in_left_pane_order;
use crate::app::sessions::start_new_normal_terminal::start_new_normal_terminal;
use crate::app::state::app_state::AppState;
use crate::extensions::command_bar::actions::open::open_command_bar;
use crate::extensions::command_bar::input::handle_keyboard::handle_command_bar_keyboard;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::history_modal::actions::open::open_conversation_picker;
use crate::extensions::history_modal::actions::open_place_in_active_split::open_place_in_active_split_picker;
use crate::extensions::history_modal::input::handle_keyboard::handle_conversation_picker_keyboard;
use crate::extensions::harness::sessions::creation::start_new_chat::start_new_chat;
use crate::extensions::terminal::ghostty::setup_ghostty_config::setup_ghostty_config;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::{
    persist_multiplexer_state, persist_multiplexer_state_now,
};
use crate::ui::grid_layout::split::split_active_pane::split_active_terminal_pane;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::focus::toggle_left_pane_visibility::toggle_left_pane_visibility;
use crate::ui::left_panel::input::select_visible_left_pane_row::select_visible_left_pane_row;
use crate::ui::left_panel::input::visible_row_index_for_keyboard::visible_row_index_for_keyboard;
use crate::ui::left_panel::mode::cycle_left_pane_mode::cycle_left_pane_mode;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;
use crate::ui::notifications::toast::show_failed_to_start_new_chat::show_failed_to_start_new_chat_toast;
use crate::ui::notifications::toast::show_failed_to_start_terminal::show_failed_to_start_terminal_toast;
use crate::ui::workspace_pane::select_workspace_by_visible_index::select_workspace_by_visible_index;
use crate::ui::workspace_pane::toggle_workspace_view::toggle_workspace_view;
use crate::ui::workspace_pane::workspace_index_for_keyboard::workspace_index_for_keyboard;

/// Handles keyboard input for global shortcuts and the focused pane.
pub fn handle_keyboard_event(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    if !keyboard.is_key_down() {
        return Ok(CoordinatorAction::Continue);
    }
    if app.delete_confirmation.is_open() {
        return Ok(handle_delete_session_confirmation_keyboard(app, keyboard));
    }
    if app.command_bar.is_open {
        let outcome = handle_command_bar_keyboard(app, keyboard);
        if let Some(hotkey) = outcome.hotkey {
            return Ok(handle_app_hotkey(app, hotkey));
        }
        return Ok(outcome.action);
    }
    if app.conversation_picker.is_open {
        return Ok(handle_conversation_picker_keyboard(app, keyboard));
    }
    if app.active_main_pane_tab == MainPaneTab::Expo && keyboard.key_code == KeyCode::Esc {
        hide_expo(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if let Some(index) = visible_row_index_for_keyboard(&keyboard) {
        let outcome = select_visible_left_pane_row(app, index);
        persist_left_pane_focus_outcome(app, outcome);
        return Ok(coordinator_action_for_left_pane_outcome(outcome));
    }
    if let Some(index) = workspace_index_for_keyboard(&keyboard) {
        return Ok(redraw_if(select_workspace_by_visible_index(app, index)));
    }
    if !normal_terminal_passthrough_hotkey(app, &keyboard) {
        if let Some(hotkey) =
            resolve_app_hotkey(&app.hotkey_registry, &keyboard, &active_hotkey_scope(app))
        {
            return Ok(handle_app_hotkey(app, hotkey));
        }
    }
    match app.focused_pane {
        FocusedPane::Left => handle_left_keyboard(app, keyboard),
        FocusedPane::Terminal => handle_terminal_keyboard(app, keyboard),
    }
}

/// Applies one resolved top-level hotkey to app state.
fn handle_app_hotkey(app: &mut AppState, hotkey: AppHotkey) -> CoordinatorAction {
    match hotkey {
        AppHotkey::CycleSession(direction) => {
            redraw_if(cycle_session_in_left_pane_order(app, direction))
        }
        AppHotkey::CycleLeftPaneMode => {
            cycle_left_pane_mode(app);
            persist_multiplexer_state(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::OpenCommandBar => {
            open_command_bar(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::OpenConversationPicker => {
            open_conversation_picker(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::OpenFocusedConversationExpo => {
            if app.active_main_pane_tab == MainPaneTab::Expo {
                hide_expo(app);
            } else {
                open_expo_for_focused_conversation(app);
            }
            CoordinatorAction::Redraw
        }
        AppHotkey::PlaceConversationInActiveSplit(direction) => {
            open_place_in_active_split_picker(app, direction);
            CoordinatorAction::Redraw
        }
        AppHotkey::SplitTerminal(direction) => {
            if let Err(error) = split_active_terminal_pane(app, direction) {
                show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
            }
            CoordinatorAction::Redraw
        }
        AppHotkey::SelectWorkspace(index) => {
            redraw_if(select_workspace_by_visible_index(app, index))
        }
        AppHotkey::SetupGhosttyConfig => {
            setup_ghostty_config_for_app(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::StartChat => {
            if let Err(error) = start_new_chat(app) {
                show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
            }
            CoordinatorAction::Redraw
        }
        AppHotkey::StartTerminal => {
            if let Err(error) = start_new_normal_terminal(app) {
                show_failed_to_start_terminal_toast(&mut app.toast_manager, &error);
            }
            CoordinatorAction::Redraw
        }
        AppHotkey::ToggleLeftPane => {
            toggle_left_pane_visibility(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::ToggleFocusedPane => {
            toggle_focused_pane(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::ToggleWorkspaceView => redraw_if(toggle_workspace_view(app)),
        AppHotkey::DeleteFocusedSession => {
            open_delete_session_confirmation(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::Quit => {
            persist_multiplexer_state_now(app);
            CoordinatorAction::Quit
        }
    }
}

/// Configures Ghostty for Ratsus shortcuts and reports the result to the user.
fn setup_ghostty_config_for_app(app: &mut AppState) {
    match setup_ghostty_config() {
        Ok(result) if result.changed => app.toast_manager.success(format!(
            "Updated Ghostty config: {}. Restart Ghostty.",
            result.config_path.display()
        )),
        Ok(result) => app.toast_manager.info(format!(
            "Ghostty config already ready: {}",
            result.config_path.display()
        )),
        Err(error) => app
            .toast_manager
            .error(format!("Failed to setup Ghostty config: {error}")),
    }
}

/// Persists app focus when a left-pane action changed visible focus.
fn persist_left_pane_focus_outcome(app: &AppState, outcome: LeftPaneActionOutcome) {
    if outcome == LeftPaneActionOutcome::Handled {
        persist_multiplexer_state(app);
    }
}

/// Converts a left-pane action outcome into a coordinator action.
fn coordinator_action_for_left_pane_outcome(outcome: LeftPaneActionOutcome) -> CoordinatorAction {
    match outcome {
        LeftPaneActionOutcome::Handled => CoordinatorAction::Redraw,
        LeftPaneActionOutcome::Continue => CoordinatorAction::Continue,
        LeftPaneActionOutcome::Quit => CoordinatorAction::Quit,
    }
}

/// Converts a boolean state change into a coordinator action.
fn redraw_if(changed: bool) -> CoordinatorAction {
    if changed {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use insta::assert_snapshot;
    use ratkit::{CoordinatorAction, KeyboardEvent};

    use super::handle_keyboard_event;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
    use crate::extensions::harness::core::chat_session::{ChatSession, ChatSessionKind};
    use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;

    /// Ctrl+K opens the command bar instead of the conversation picker.
    #[test]
    fn control_k_opens_command_bar() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");

        let outcome = handle_keyboard_event(&mut app, control_key('k')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert!(app.command_bar.is_open);
        assert!(app.command_bar.is_filtering);
        assert!(!app.conversation_picker.is_open);
    }

    /// Ctrl+H opens the conversation picker for chat history.
    #[test]
    fn control_h_opens_conversation_picker() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");

        let outcome = handle_keyboard_event(&mut app, control_key('h')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert!(app.conversation_picker.is_open);
        assert!(!app.command_bar.is_open);
    }

    /// Expo hotkey hides Expo when it is already active.
    #[test]
    fn expo_hotkey_hides_active_expo() {
        let mut app = expo_test_app();

        let outcome = handle_keyboard_event(&mut app, control_key('e')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_main_pane_tab, MainPaneTab::Chat);
    }

    /// Escape hides Expo when it is active.
    #[test]
    fn escape_hides_active_expo() {
        let mut app = expo_test_app();

        let outcome = handle_keyboard_event(&mut app, key(KeyCode::Esc)).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_main_pane_tab, MainPaneTab::Chat);
    }

    /// Ctrl+E is terminal input rather than the Expo shortcut in normal terminals.
    #[test]
    fn normal_terminal_keeps_control_e_as_terminal_input() {
        let mut app =
            app_fixture(vec![normal_terminal_session("shell", "shell")]).expect("app fixture");

        let outcome = handle_keyboard_event(&mut app, control_key('e')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_main_pane_tab, MainPaneTab::Chat);
    }

    /// Ctrl+Tab selects the next visible session row.
    #[test]
    fn control_tab_selects_next_session() {
        let mut app = app_fixture(vec![
            dormant_session("chat", "chat", "/tmp/project"),
            normal_terminal_session("shell", "shell"),
        ])
        .expect("app fixture");
        app.active_index = 0;
        app.focused_index = 0;

        let outcome = handle_keyboard_event(
            &mut app,
            key_with_modifiers(KeyCode::Tab, KeyModifiers::CONTROL),
        )
        .expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_index, 1);
        assert_eq!(app.focused_index, 1);
    }

    /// Ctrl+Shift+Tab selects the previous visible session row.
    #[test]
    fn control_shift_tab_selects_previous_session() {
        let mut app = app_fixture(vec![
            dormant_session("chat", "chat", "/tmp/project"),
            normal_terminal_session("shell", "shell"),
        ])
        .expect("app fixture");
        app.active_index = 0;
        app.focused_index = 0;

        let outcome = handle_keyboard_event(
            &mut app,
            key_with_modifiers(
                KeyCode::BackTab,
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
        )
        .expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_index, 1);
        assert_eq!(app.focused_index, 1);
    }

    /// Reproduces terminal encodings where Ctrl+Tab shortcuts do not change session selection.
    #[test]
    fn ctrl_tab_terminal_encodings_cycle_sessions() {
        let mut app = app_fixture(vec![
            dormant_session("chat", "chat", "/tmp/project"),
            normal_terminal_session("shell", "shell"),
        ])
        .expect("app fixture");

        let ctrl_tab_outcome = handle_keyboard_event(
            &mut app,
            key_with_modifiers(KeyCode::Char('\t'), KeyModifiers::CONTROL),
        )
        .expect("keyboard event");
        let ctrl_tab_report = format!(
            "ctrl_tab_outcome: {ctrl_tab_outcome:?}\nctrl_tab_active_index: {}\nctrl_tab_focused_index: {}",
            app.active_index, app.focused_index
        );

        app.active_index = 0;
        app.focused_index = 0;
        let control_i_outcome = handle_keyboard_event(
            &mut app,
            key_with_modifiers(KeyCode::Char('i'), KeyModifiers::CONTROL),
        )
        .expect("keyboard event");
        let control_i_report = format!(
            "control_i_outcome: {control_i_outcome:?}\ncontrol_i_active_index: {}\ncontrol_i_focused_index: {}",
            app.active_index, app.focused_index
        );

        app.active_index = 0;
        app.focused_index = 0;
        let ctrl_shift_tab_outcome = handle_keyboard_event(
            &mut app,
            key_with_modifiers(
                KeyCode::BackTab,
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
        )
        .expect("keyboard event");
        let ctrl_shift_tab_report = format!(
            "ctrl_shift_tab_outcome: {ctrl_shift_tab_outcome:?}\nctrl_shift_tab_active_index: {}\nctrl_shift_tab_focused_index: {}",
            app.active_index, app.focused_index
        );

        assert_snapshot!(
            format!("{ctrl_tab_report}\n{control_i_report}\n{ctrl_shift_tab_report}"),
            @r###"
ctrl_tab_outcome: Redraw
ctrl_tab_active_index: 1
ctrl_tab_focused_index: 1
control_i_outcome: Redraw
control_i_active_index: 1
control_i_focused_index: 1
ctrl_shift_tab_outcome: Redraw
ctrl_shift_tab_active_index: 1
ctrl_shift_tab_focused_index: 1
"###
        );
    }

    /// Ctrl+T opens a normal terminal in the selected workspace, not the active session folder.
    #[test]
    fn control_t_starts_terminal_in_selected_workspace() {
        let mut app = app_fixture(vec![dormant_session("alpha", "alpha", "/workspace/alpha")])
            .expect("app fixture");
        app.folder_order = vec![
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ];
        app.selected_workspace_path = Some(PathBuf::from("/workspace/beta"));
        app.active_index = 0;
        app.focused_index = 0;

        let outcome = handle_keyboard_event(&mut app, control_key('t')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(
            app.session_terminals[1].session.kind,
            ChatSessionKind::NormalTerminal
        );
        assert_eq!(
            app.session_terminals[1].session.working_dir,
            PathBuf::from("/workspace/beta")
        );
    }

    /// Ctrl+number selects a workspace before terminal passthrough.
    #[test]
    fn control_number_selects_workspace() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.folder_order = vec![PathBuf::from("/a"), PathBuf::from("/b")];

        let outcome = handle_keyboard_event(&mut app, control_key('2')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.selected_workspace_path, Some(PathBuf::from("/b")));
    }

    /// Ctrl+number selects the first folder session in grouped folder mode.
    #[test]
    fn control_number_selects_folder_session_when_workspace_view_is_disabled() {
        let mut app = app_fixture(vec![
            dormant_session("a-one", "a-one", "/workspace/a"),
            dormant_session("b-one", "b-one", "/workspace/b"),
            dormant_session("b-two", "b-two", "/workspace/b"),
        ])
        .expect("app fixture");
        app.workspace_view_enabled = false;
        app.folder_order = vec![PathBuf::from("/workspace/a"), PathBuf::from("/workspace/b")];

        let outcome = handle_keyboard_event(&mut app, control_key('2')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_index, 1);
        assert_eq!(app.focused_index, 1);
        assert_eq!(
            app.selected_workspace_path,
            Some(PathBuf::from("/workspace/b"))
        );
    }

    /// Cmd+number selects the matching visible row in the active left-pane content.
    #[test]
    fn super_number_selects_visible_session_row() {
        let mut app = app_fixture(vec![
            dormant_session("one", "one", "/tmp/project"),
            dormant_session("two", "two", "/tmp/project"),
        ])
        .expect("app fixture");

        let outcome = handle_keyboard_event(&mut app, super_key('2')).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.active_index, 1);
        assert_eq!(app.focused_index, 1);
        assert_eq!(
            app.selected_workspace_path,
            Some(PathBuf::from("/tmp/project"))
        );
    }

    /// Ctrl+` cycles the left pane through sessions, plans, and files.
    #[test]
    fn control_backtick_cycles_left_pane_modes() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");

        let first = handle_keyboard_event(&mut app, control_key('`')).expect("keyboard event");
        assert_eq!(first, CoordinatorAction::Redraw);
        assert_eq!(
            app.left_pane_mode,
            crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode::Plans
        );
        assert_eq!(app.active_main_pane_tab, MainPaneTab::Chat);

        let second = handle_keyboard_event(&mut app, control_key('`')).expect("keyboard event");
        assert_eq!(second, CoordinatorAction::Redraw);
        assert_eq!(
            app.left_pane_mode,
            crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode::Files
        );

        let third = handle_keyboard_event(&mut app, control_key('`')).expect("keyboard event");
        assert_eq!(third, CoordinatorAction::Redraw);
        assert_eq!(
            app.left_pane_mode,
            crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode::Sessions
        );
    }

    /// Command bar workspace command toggles the workspace view mode.
    #[test]
    fn command_bar_workspace_command_toggles_workspace_view() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.command_bar.is_open = true;
        app.command_bar.query = "toggle workspace".to_string();
        app.workspace_view_enabled = true;

        let outcome = handle_keyboard_event(&mut app, key(KeyCode::Enter)).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert!(!app.workspace_view_enabled);
    }

    /// Command bar workspace hotkey command selects the requested workspace.
    #[test]
    fn command_bar_select_workspace_command_selects_workspace() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.folder_order = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        app.command_bar.is_open = true;
        app.command_bar.query = "select workspace 2".to_string();

        let outcome = handle_keyboard_event(&mut app, key(KeyCode::Enter)).expect("keyboard event");

        assert_eq!(outcome, CoordinatorAction::Redraw);
        assert_eq!(app.selected_workspace_path, Some(PathBuf::from("/b")));
    }

    /// Builds an app with Expo active for keyboard tests.
    fn expo_test_app() -> crate::app::state::app_state::AppState {
        let mut app = app_fixture(vec![dormant_session("one", "one", "/tmp/project-one")])
            .expect("app fixture");
        app.active_main_pane_tab = MainPaneTab::Expo;
        app.selected_expo_folder = Some(PathBuf::from("/tmp/project-one"));
        app
    }

    /// Builds a normal terminal session fixture with fake terminal IO.
    fn normal_terminal_session(title: &str, id: &str) -> SessionTerminal {
        let session = ChatSession::new("now", title, id, "/tmp/project")
            .with_kind(ChatSessionKind::NormalTerminal);
        SessionTerminal::with_terminal(session, ChatTerminal::stub(title, "shell"))
    }

    /// Builds a control-character key press.
    fn control_key(character: char) -> KeyboardEvent {
        KeyboardEvent {
            key_code: KeyCode::Char(character),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
        }
    }

    /// Builds a key press with explicit modifiers.
    fn key_with_modifiers(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }

    /// Builds a Cmd-character key press.
    fn super_key(character: char) -> KeyboardEvent {
        KeyboardEvent {
            key_code: KeyCode::Char(character),
            modifiers: KeyModifiers::SUPER,
            kind: KeyEventKind::Press,
        }
    }

    /// Builds a plain key press.
    fn key(key_code: KeyCode) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
        }
    }
}
