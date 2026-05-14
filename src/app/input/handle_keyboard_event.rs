use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::deletion::handle_delete_session_confirmation_keyboard::handle_delete_session_confirmation_keyboard;
use crate::app::expo::open_expo_for_focused_conversation::open_expo_for_focused_conversation;
use crate::app::focus::toggle_focused_pane::toggle_focused_pane;
use crate::app::input::handle_left_keyboard::handle_left_keyboard;
use crate::app::input::handle_terminal_keyboard::handle_terminal_keyboard;
use crate::app::input::hotkeys::app_hotkey::AppHotkey;
use crate::app::input::hotkeys::resolve_app_hotkey::resolve_app_hotkey;
use crate::app::input::hotkeys::scopes::active_hotkey_scope;
use crate::app::navigation::cycle_chat_in_left_pane_order::cycle_chat_in_left_pane_order;
use crate::app::sessions::start_new_normal_terminal::start_new_normal_terminal;
use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::actions::open::open_conversation_picker;
use crate::extensions::harness::conversation_picker::actions::open_place_in_active_split::open_place_in_active_split_picker;
use crate::extensions::harness::conversation_picker::input::handle_keyboard::handle_conversation_picker_keyboard;
use crate::extensions::harness::sessions::creation::start_new_chat::start_new_chat;
use crate::ui::grid_layout::split::split_active_pane::split_active_terminal_pane;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::focus::toggle_left_pane_visibility::toggle_left_pane_visibility;
use crate::ui::notifications::toast::show_failed_to_start_new_chat::show_failed_to_start_new_chat_toast;
use crate::ui::notifications::toast::show_failed_to_start_terminal::show_failed_to_start_terminal_toast;

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
    if app.conversation_picker.is_open {
        return Ok(handle_conversation_picker_keyboard(app, keyboard));
    }
    if let Some(hotkey) =
        resolve_app_hotkey(&app.hotkey_registry, &keyboard, &active_hotkey_scope(app))
    {
        return Ok(handle_app_hotkey(app, hotkey));
    }
    match app.focused_pane {
        FocusedPane::Left => handle_left_keyboard(app, keyboard),
        FocusedPane::Terminal => handle_terminal_keyboard(app, keyboard),
    }
}

/// Applies one resolved top-level hotkey to app state.
fn handle_app_hotkey(app: &mut AppState, hotkey: AppHotkey) -> CoordinatorAction {
    match hotkey {
        AppHotkey::CycleChat(direction) => redraw_if(cycle_chat_in_left_pane_order(app, direction)),
        AppHotkey::OpenConversationPicker => {
            open_conversation_picker(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::OpenFocusedConversationExpo => {
            open_expo_for_focused_conversation(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::PlaceConversationInActiveSplit => {
            open_place_in_active_split_picker(app);
            CoordinatorAction::Redraw
        }
        AppHotkey::SplitTerminal(direction) => {
            if let Err(error) = split_active_terminal_pane(app, direction) {
                show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
            }
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
        AppHotkey::Quit => CoordinatorAction::Quit,
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
