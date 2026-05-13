use crossterm::event::KeyModifiers;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::deletion::handle_delete_session_confirmation_keyboard::handle_delete_session_confirmation_keyboard;
use crate::app::expo::open_expo_for_focused_conversation::open_expo_for_focused_conversation;
use crate::app::focus::toggle_focused_pane::toggle_focused_pane;
use crate::app::input::chat_cycle_direction_for_keyboard::chat_cycle_direction_for_keyboard;
use crate::app::input::handle_left_keyboard::handle_left_keyboard;
use crate::app::input::handle_terminal_keyboard::handle_terminal_keyboard;
use crate::app::navigation::cycle_chat_in_left_pane_order::cycle_chat_in_left_pane_order;
use crate::app::sessions::start_new_normal_terminal::start_new_normal_terminal;
use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::actions::open::open_conversation_picker;
use crate::extensions::harness::conversation_picker::actions::open_place_in_active_split::open_place_in_active_split_picker;
use crate::extensions::harness::conversation_picker::input::handle_keyboard::handle_conversation_picker_keyboard;
use crate::extensions::harness::sessions::creation::start_new_chat::start_new_chat;
use crate::ui::grid_layout::bundle::terminal_bundle_for_keyboard::terminal_bundle_for_keyboard;
use crate::ui::grid_layout::split::split_active_pane::split_active_terminal_pane;
use crate::ui::grid_layout::split::split_direction_for_keyboard::terminal_split_direction_for_keyboard;
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
    if let Some(direction) = chat_cycle_direction_for_keyboard(&keyboard) {
        if cycle_chat_in_left_pane_order(app, direction) {
            return Ok(CoordinatorAction::Redraw);
        }
        return Ok(CoordinatorAction::Continue);
    }
    if keyboard.is_char('k') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        open_conversation_picker(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('e') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        open_expo_for_focused_conversation(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if terminal_bundle_for_keyboard(&keyboard) {
        open_place_in_active_split_picker(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if let Some(direction) = terminal_split_direction_for_keyboard(&keyboard) {
        if let Err(error) = split_active_terminal_pane(app, direction) {
            show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        }
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('n') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        if let Err(error) = start_new_chat(app) {
            show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        }
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('t') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        if let Err(error) = start_new_normal_terminal(app) {
            show_failed_to_start_terminal_toast(&mut app.toast_manager, &error);
        }
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('l') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        toggle_left_pane_visibility(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('x') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        toggle_focused_pane(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('q') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        return Ok(CoordinatorAction::Quit);
    }
    match app.focused_pane {
        FocusedPane::Left => handle_left_keyboard(app, keyboard),
        FocusedPane::Terminal => handle_terminal_keyboard(app, keyboard),
    }
}
