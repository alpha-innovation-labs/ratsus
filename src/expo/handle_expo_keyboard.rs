use crossterm::event::KeyCode;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::app_state::AppState;
use crate::expo::decrease_expo_card_width::decrease_expo_card_width;
use crate::expo::increase_expo_card_width::increase_expo_card_width;
use crate::expo::move_focused_expo_conversation::move_focused_expo_conversation;

/// Handles keyboard input for Expo filtering.
pub fn handle_expo_keyboard(app: &mut AppState, keyboard: &KeyboardEvent) -> CoordinatorAction {
    if keyboard.is_char('/') && keyboard.modifiers.is_empty() {
        app.expo_filtering = true;
        return CoordinatorAction::Redraw;
    }
    if !app.expo_filtering {
        return handle_expo_navigation(app, keyboard);
    }
    match keyboard.key_code {
        KeyCode::Esc => {
            app.expo_filtering = false;
            app.expo_filter_query.clear();
            app.expo_scroll = 0;
        }
        KeyCode::Enter => app.expo_filtering = false,
        KeyCode::Backspace => {
            app.expo_filter_query.pop();
            app.expo_scroll = 0;
        }
        KeyCode::Char(value) if keyboard.modifiers.is_empty() => {
            app.expo_filter_query.push(value);
            app.expo_scroll = 0;
        }
        _ => return CoordinatorAction::Continue,
    }
    CoordinatorAction::Redraw
}

/// Handles Expo focus navigation while not actively filtering.
fn handle_expo_navigation(app: &mut AppState, keyboard: &KeyboardEvent) -> CoordinatorAction {
    match keyboard.key_code {
        KeyCode::Char('h') | KeyCode::Char('k') | KeyCode::Left | KeyCode::Up => {
            move_focused_expo_conversation(app, -1);
            CoordinatorAction::Redraw
        }
        KeyCode::Char('j') | KeyCode::Char('l') | KeyCode::Right | KeyCode::Down => {
            move_focused_expo_conversation(app, 1);
            CoordinatorAction::Redraw
        }
        KeyCode::Char('+') | KeyCode::Char('=') => {
            increase_expo_card_width(app);
            CoordinatorAction::Redraw
        }
        KeyCode::Char('-') => {
            decrease_expo_card_width(app);
            CoordinatorAction::Redraw
        }
        KeyCode::Enter => {
            app.activate_focused_session();
            CoordinatorAction::Redraw
        }
        _ => CoordinatorAction::Continue,
    }
}
