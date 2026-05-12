use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::activate_selected_conversation::activate_selected_conversation;
use crate::conversation_picker::apply_conversation_picker_query_change::apply_conversation_picker_query_change;
use crate::conversation_picker::close_conversation_picker::close_conversation_picker;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;
use crate::conversation_picker::move_conversation_picker_selection::move_conversation_picker_selection;

/// Handles keyboard input while the conversation picker modal is open.
pub fn handle_conversation_picker_keyboard(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> CoordinatorAction {
    match keyboard.key_code {
        KeyCode::Esc => close_conversation_picker(app),
        KeyCode::Enter => activate_selected_conversation(app),
        KeyCode::Backspace => {
            app.conversation_picker.query.pop();
            apply_conversation_picker_query_change(app);
        }
        KeyCode::Down => move_picker_selection(app, 1),
        KeyCode::Up => move_picker_selection(app, -1),
        KeyCode::Char('n') if keyboard.modifiers.contains(KeyModifiers::CONTROL) => {
            move_picker_selection(app, 1);
        }
        KeyCode::Char('p') if keyboard.modifiers.contains(KeyModifiers::CONTROL) => {
            move_picker_selection(app, -1);
        }
        KeyCode::Char(character) if accepts_filter_character(keyboard.modifiers) => {
            app.conversation_picker.query.push(character);
            apply_conversation_picker_query_change(app);
        }
        _ => {}
    }
    CoordinatorAction::Redraw
}

/// Moves picker selection using the current visible picker row count.
fn move_picker_selection(app: &mut NexusDemo, direction: isize) {
    let item_count = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        app.conversation_picker.folder_filter.as_deref(),
    )
    .len();
    move_conversation_picker_selection(&mut app.conversation_picker, direction, item_count);
}

/// Returns whether a key modifier set can be inserted into the filter query.
fn accepts_filter_character(modifiers: KeyModifiers) -> bool {
    modifiers.is_empty() || modifiers == KeyModifiers::SHIFT
}
