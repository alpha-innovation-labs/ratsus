use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::ui::keyboard::list::behavior::{handle_list_keyboard, ListKeyBehavior};
use crate::ui::keyboard::list::outcome::ListKeyOutcome;

#[derive(Default)]
struct FakeList {
    position: isize,
    pending_g: bool,
    filtering: bool,
    filter: String,
    deleted: bool,
    toggled_count: usize,
}

impl ListKeyBehavior for FakeList {
    /// Moves the fake selection by the provided direction.
    fn move_selection(&mut self, direction: isize) {
        self.position += direction;
    }

    /// Moves the fake selection to the first row.
    fn focus_first(&mut self) {
        self.position = 0;
    }

    /// Moves the fake selection to the last row.
    fn focus_last(&mut self) {
        self.position = 99;
    }

    /// Records activation of the fake selection.
    fn activate_selection(&mut self) {}

    /// Records delete flow for the fake selection.
    fn delete_selection(&mut self) {
        self.deleted = true;
    }

    /// Records bulk-selection toggles for the fake selection.
    fn toggle_selection(&mut self) {
        self.toggled_count += 1;
    }

    /// Returns whether fake filtering is active.
    fn is_filtering(&self) -> bool {
        self.filtering
    }

    /// Starts fake filter mode.
    fn start_filtering(&mut self) {
        self.filtering = true;
    }

    /// Appends a fake filter character.
    fn insert_filter_character(&mut self, character: char) {
        self.filter.push(character);
    }

    /// Returns fake pending `g` state.
    fn has_pending_g(&self) -> bool {
        self.pending_g
    }

    /// Sets fake pending `g` state.
    fn set_pending_g(&mut self, pending: bool) {
        self.pending_g = pending;
    }
}

/// Verifies j and k move list selection by default.
#[test]
fn jk_move_selection() {
    let mut list = FakeList::default();
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('j')));
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('k')));

    assert_eq!(list.position, 0);
}

/// Verifies gg and G navigate to list boundaries by default.
#[test]
fn vim_boundaries_move_selection() {
    let mut list = FakeList::default();
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('G')));
    assert_eq!(list.position, 99);
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('g')));
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('g')));

    assert_eq!(list.position, 0);
}

/// Verifies filtering captures typed letters instead of list navigation.
#[test]
fn filtering_captures_j_and_k() {
    let mut list = FakeList::default();
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('/')));
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('j')));
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('k')));

    assert_eq!(list.filter, "jk");
    assert_eq!(list.position, 0);
}

/// Verifies d and q use the common delete and quit behavior.
#[test]
fn delete_and_quit_are_shared() {
    let mut list = FakeList::default();
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char('d')));
    let outcome = handle_list_keyboard(&mut list, key(KeyCode::Char('q')));

    assert!(list.deleted);
    assert_eq!(outcome, ListKeyOutcome::Quit);
}

/// Verifies held space does not repeatedly toggle the same selection.
#[test]
fn repeated_space_is_ignored_for_toggle_selection() {
    let mut list = FakeList::default();
    let _ = handle_list_keyboard(&mut list, key(KeyCode::Char(' ')));
    let outcome = handle_list_keyboard(&mut list, repeated_key(KeyCode::Char(' ')));

    assert_eq!(list.toggled_count, 1);
    assert_eq!(outcome, ListKeyOutcome::Continue);
}

/// Builds a key press event without modifiers.
fn key(key_code: KeyCode) -> KeyboardEvent {
    keyboard_event(key_code, KeyEventKind::Press)
}

/// Builds a key repeat event without modifiers.
fn repeated_key(key_code: KeyCode) -> KeyboardEvent {
    keyboard_event(key_code, KeyEventKind::Repeat)
}

/// Builds a keyboard event without modifiers.
fn keyboard_event(key_code: KeyCode, kind: KeyEventKind) -> KeyboardEvent {
    KeyboardEvent {
        key_code,
        modifiers: KeyModifiers::empty(),
        kind,
    }
}
