use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::ui::keyboard::filter::key_accepted::filter_key_accepted;
use crate::ui::keyboard::list::outcome::ListKeyOutcome;

/// Shared keyboard contract for list-like panes, panels, and modals.
pub trait ListKeyBehavior {
    /// Moves the list selection by a signed row delta.
    fn move_selection(&mut self, direction: isize);

    /// Moves the list selection to the first visible item.
    fn focus_first(&mut self);

    /// Moves the list selection to the last visible item.
    fn focus_last(&mut self);

    /// Activates the current list selection.
    fn activate_selection(&mut self);

    /// Collapses the current selection when it owns a collapsible group.
    fn collapse_selection(&mut self) {}

    /// Opens the current selection when it owns an expandable group.
    fn open_selection(&mut self) {}

    /// Starts delete flow for the current list selection.
    fn delete_selection(&mut self) {}

    /// Toggles bulk-selection state for the current list item.
    fn toggle_selection(&mut self) {}

    /// Reorders the current list item by a signed row delta when supported.
    fn reorder_selection(&mut self, _direction: isize) {}

    /// Closes the list overlay when the implementation supports closing.
    fn close_selection_context(&mut self) -> bool {
        false
    }

    /// Returns true when text typed by the user should edit a filter query.
    fn is_filtering(&self) -> bool {
        false
    }

    /// Starts filter-entry mode for the list.
    fn start_filtering(&mut self) {}

    /// Inserts a character into the active filter query.
    fn insert_filter_character(&mut self, _character: char) {}

    /// Removes a character from the active filter query.
    fn delete_filter_character(&mut self) {}

    /// Returns whether this list has a pending `g` prefix.
    fn has_pending_g(&self) -> bool;

    /// Sets whether this list has a pending `g` prefix.
    fn set_pending_g(&mut self, pending: bool);

    /// Handles a keyboard event using the standard list shortcut map.
    fn handle_list_keyboard(&mut self, keyboard: KeyboardEvent) -> ListKeyOutcome {
        handle_list_keyboard(self, keyboard)
    }
}

/// Handles one keyboard event against the shared list shortcut map.
pub fn handle_list_keyboard<T: ListKeyBehavior + ?Sized>(
    behavior: &mut T,
    keyboard: KeyboardEvent,
) -> ListKeyOutcome {
    match keyboard.key_code {
        KeyCode::Esc => close_selection_context(behavior),
        KeyCode::Enter => activate(behavior),
        KeyCode::Backspace if behavior.is_filtering() => delete_filter_character(behavior),
        KeyCode::Char(character)
            if behavior.is_filtering() && filter_key_accepted(keyboard.modifiers) =>
        {
            insert_filter_character(behavior, character)
        }
        KeyCode::Down if keyboard.modifiers.contains(KeyModifiers::SHIFT) => {
            reorder_selection(behavior, 1)
        }
        KeyCode::Up if keyboard.modifiers.contains(KeyModifiers::SHIFT) => {
            reorder_selection(behavior, -1)
        }
        KeyCode::Down | KeyCode::Char('j') if keyboard.modifiers.is_empty() => {
            move_selection(behavior, 1)
        }
        KeyCode::Up | KeyCode::Char('k') if keyboard.modifiers.is_empty() => {
            move_selection(behavior, -1)
        }
        KeyCode::Char('n') if keyboard.modifiers.contains(KeyModifiers::CONTROL) => {
            move_selection(behavior, 1)
        }
        KeyCode::Char('p') if keyboard.modifiers.contains(KeyModifiers::CONTROL) => {
            move_selection(behavior, -1)
        }
        KeyCode::PageDown => move_selection(behavior, 8),
        KeyCode::PageUp => move_selection(behavior, -8),
        KeyCode::Left | KeyCode::Char('h') if keyboard.modifiers.is_empty() => collapse(behavior),
        KeyCode::Right | KeyCode::Char('l') if keyboard.modifiers.is_empty() => open(behavior),
        KeyCode::Char('/') if filter_key_accepted(keyboard.modifiers) => start_filtering(behavior),
        KeyCode::Char(' ') if keyboard.modifiers.is_empty() => toggle_selection(behavior),
        KeyCode::Char('d') if keyboard.modifiers.is_empty() => delete_selection(behavior),
        KeyCode::Char('g') if keyboard.modifiers.is_empty() => handle_g(behavior),
        KeyCode::Char('G') => focus_last(behavior),
        KeyCode::Char('q') if keyboard.modifiers.is_empty() => ListKeyOutcome::Quit,
        _ => continue_unhandled(behavior),
    }
}

/// Marks a key event as handled and clears pending multi-key state.
fn handled<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.set_pending_g(false);
    ListKeyOutcome::Handled
}

/// Closes the current list context when supported.
fn close_selection_context<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    if behavior.close_selection_context() {
        return handled(behavior);
    }
    continue_unhandled(behavior)
}

/// Activates the current selection.
fn activate<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.activate_selection();
    handled(behavior)
}

/// Moves the current selection by a signed delta.
fn move_selection<T: ListKeyBehavior + ?Sized>(
    behavior: &mut T,
    direction: isize,
) -> ListKeyOutcome {
    behavior.move_selection(direction);
    handled(behavior)
}

/// Collapses the current selection.
fn collapse<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.collapse_selection();
    handled(behavior)
}

/// Opens the current selection.
fn open<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.open_selection();
    handled(behavior)
}

/// Starts filtering the current list.
fn start_filtering<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.start_filtering();
    handled(behavior)
}

/// Inserts one filter character.
fn insert_filter_character<T: ListKeyBehavior + ?Sized>(
    behavior: &mut T,
    character: char,
) -> ListKeyOutcome {
    behavior.insert_filter_character(character);
    handled(behavior)
}

/// Deletes one filter character.
fn delete_filter_character<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.delete_filter_character();
    handled(behavior)
}

/// Starts delete flow for the current selection.
fn delete_selection<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.delete_selection();
    handled(behavior)
}

/// Toggles bulk-selection state for the current selection.
fn toggle_selection<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.toggle_selection();
    handled(behavior)
}

/// Reorders the current selection by a signed row delta.
fn reorder_selection<T: ListKeyBehavior + ?Sized>(
    behavior: &mut T,
    direction: isize,
) -> ListKeyOutcome {
    behavior.reorder_selection(direction);
    handled(behavior)
}

/// Handles `g` as a pending prefix and `gg` as first-row navigation.
fn handle_g<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    if behavior.has_pending_g() {
        behavior.focus_first();
        return handled(behavior);
    }
    behavior.set_pending_g(true);
    ListKeyOutcome::Handled
}

/// Moves the selection to the last row.
fn focus_last<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.focus_last();
    handled(behavior)
}

/// Clears pending state for unhandled keys.
fn continue_unhandled<T: ListKeyBehavior + ?Sized>(behavior: &mut T) -> ListKeyOutcome {
    behavior.set_pending_g(false);
    ListKeyOutcome::Continue
}
