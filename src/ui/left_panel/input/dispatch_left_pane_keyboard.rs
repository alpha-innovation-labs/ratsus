use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::ui::keyboard::list::behavior::{handle_list_keyboard, ListKeyBehavior};
use crate::ui::keyboard::list::outcome::ListKeyOutcome;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Dispatches one keyboard event through the shared list map into left-pane actions.
pub fn dispatch_left_pane_keyboard<C: LeftPaneContent + ?Sized>(
    content: &mut C,
    keyboard: KeyboardEvent,
) -> LeftPaneActionOutcome {
    if !content.is_filtering() {
        if let Some(direction) = adjacent_group_direction(&keyboard) {
            return content.handle_left_pane_action(LeftPaneAction::FocusAdjacentGroup(direction));
        }
    }
    let mut behavior = LeftPaneActionBehavior::new(content);
    let list_outcome = handle_list_keyboard(&mut behavior, keyboard);
    action_outcome_for_list_outcome(&mut behavior, list_outcome)
}

/// Returns adjacent-group direction for Shift+J and Shift+K left-pane shortcuts.
fn adjacent_group_direction(keyboard: &KeyboardEvent) -> Option<isize> {
    match keyboard.key_code {
        KeyCode::Char('J') => Some(1),
        KeyCode::Char('K') => Some(-1),
        KeyCode::Char('j') if keyboard.modifiers.contains(KeyModifiers::SHIFT) => Some(1),
        KeyCode::Char('k') if keyboard.modifiers.contains(KeyModifiers::SHIFT) => Some(-1),
        _ => None,
    }
}

/// Converts a list-key outcome into a left-pane content outcome.
fn action_outcome_for_list_outcome<C: LeftPaneContent + ?Sized>(
    behavior: &mut LeftPaneActionBehavior<'_, C>,
    list_outcome: ListKeyOutcome,
) -> LeftPaneActionOutcome {
    match list_outcome {
        ListKeyOutcome::Handled => behavior.outcome.unwrap_or(LeftPaneActionOutcome::Handled),
        ListKeyOutcome::Continue => LeftPaneActionOutcome::Continue,
        ListKeyOutcome::Quit => behavior.dispatch(LeftPaneAction::Quit),
    }
}

/// Adapter that keeps ListKeyBehavior as the source for default left-pane hotkeys.
struct LeftPaneActionBehavior<'a, C: LeftPaneContent + ?Sized> {
    content: &'a mut C,
    outcome: Option<LeftPaneActionOutcome>,
}

impl<'a, C: LeftPaneContent + ?Sized> LeftPaneActionBehavior<'a, C> {
    /// Creates a semantic action adapter around active left-pane content.
    fn new(content: &'a mut C) -> Self {
        Self {
            content,
            outcome: None,
        }
    }

    /// Sends one semantic action to active left-pane content.
    fn dispatch(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        let outcome = self.content.handle_left_pane_action(action);
        self.outcome = Some(outcome);
        outcome
    }
}

impl<C: LeftPaneContent + ?Sized> ListKeyBehavior for LeftPaneActionBehavior<'_, C> {
    /// Routes signed movement into a semantic left-pane action.
    fn move_selection(&mut self, direction: isize) {
        self.dispatch(LeftPaneAction::MoveBy(direction));
    }

    /// Routes first-row focus into a semantic left-pane action.
    fn focus_first(&mut self) {
        self.dispatch(LeftPaneAction::FocusFirst);
    }

    /// Routes last-row focus into a semantic left-pane action.
    fn focus_last(&mut self) {
        self.dispatch(LeftPaneAction::FocusLast);
    }

    /// Routes activation into a semantic left-pane action.
    fn activate_selection(&mut self) {
        self.dispatch(LeftPaneAction::Activate);
    }

    /// Routes collapse into a semantic left-pane action.
    fn collapse_selection(&mut self) {
        self.dispatch(LeftPaneAction::Collapse);
    }

    /// Routes expand/open into a semantic left-pane action.
    fn open_selection(&mut self) {
        self.dispatch(LeftPaneAction::Expand);
    }

    /// Routes delete into a semantic left-pane action.
    fn delete_selection(&mut self) {
        self.dispatch(LeftPaneAction::Delete);
    }

    /// Routes selection toggling into a semantic left-pane action.
    fn toggle_selection(&mut self) {
        self.dispatch(LeftPaneAction::ToggleSelection);
    }

    /// Routes reordering into a semantic left-pane action.
    fn reorder_selection(&mut self, direction: isize) {
        self.dispatch(LeftPaneAction::ReorderBy(direction));
    }

    /// Routes close-context only when content has an active filter context.
    fn close_selection_context(&mut self) -> bool {
        if !self.content.is_filtering() {
            return false;
        }
        self.dispatch(LeftPaneAction::Quit) != LeftPaneActionOutcome::Continue
    }

    /// Returns whether active content is in filter-entry mode.
    fn is_filtering(&self) -> bool {
        self.content.is_filtering()
    }

    /// Routes filter start into a semantic left-pane action.
    fn start_filtering(&mut self) {
        self.dispatch(LeftPaneAction::StartFilter);
    }

    /// Routes typed filter text into a semantic left-pane action.
    fn insert_filter_character(&mut self, character: char) {
        self.dispatch(LeftPaneAction::InsertFilterCharacter(character));
    }

    /// Routes filter deletion into a semantic left-pane action.
    fn delete_filter_character(&mut self) {
        self.dispatch(LeftPaneAction::DeleteFilterCharacter);
    }

    /// Returns whether active content is waiting for a second `g` key.
    fn has_pending_g(&self) -> bool {
        self.content.has_pending_g()
    }

    /// Updates whether active content is waiting for a second `g` key.
    fn set_pending_g(&mut self, pending: bool) {
        self.content.set_pending_g(pending);
    }
}
