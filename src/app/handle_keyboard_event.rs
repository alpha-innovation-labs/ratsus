use crossterm::event::KeyModifiers;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::handle_left_keyboard::handle_left_keyboard;
use crate::app::handle_terminal_keyboard::handle_terminal_keyboard;
use crate::app::nexus_demo_state::NexusDemo;
use crate::app::start_new_normal_terminal::start_new_normal_terminal;
use crate::app::toggle_focused_pane::toggle_focused_pane;
use crate::conversation_picker::handle_conversation_picker_keyboard::handle_conversation_picker_keyboard;
use crate::conversation_picker::open_conversation_picker::open_conversation_picker;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::toggle_left_pane_visibility::toggle_left_pane_visibility;
use crate::nexus_sessions::start_new_nexus_chat::start_new_nexus_chat;
use crate::notifications::show_failed_to_start_new_chat_toast::show_failed_to_start_new_chat_toast;
use crate::notifications::show_failed_to_start_terminal_toast::show_failed_to_start_terminal_toast;

/// Handles keyboard input for global shortcuts and the focused pane.
pub fn handle_keyboard_event(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    if !keyboard.is_key_down() {
        return Ok(CoordinatorAction::Continue);
    }
    if app.conversation_picker.is_open {
        return Ok(handle_conversation_picker_keyboard(app, keyboard));
    }
    if keyboard.is_char('k') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        open_conversation_picker(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('n') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        if let Err(error) = start_new_nexus_chat(app) {
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
