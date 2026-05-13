use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::input::dispatch_left_pane_keyboard::dispatch_left_pane_keyboard;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

#[derive(Default)]
struct FakeLeftPaneContent {
    actions: Vec<LeftPaneAction>,
    pending_g: bool,
    filtering: bool,
}

impl LeftPaneContent for FakeLeftPaneContent {
    /// Records a routed semantic action for assertion.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        if action == LeftPaneAction::StartFilter {
            self.filtering = true;
        }
        self.actions.push(action);
        LeftPaneActionOutcome::Handled
    }

    /// Returns test footer items.
    fn footer_items(&self) -> Vec<LeftPaneFooterItem> {
        Vec::new()
    }

    /// Returns fake filtering state.
    fn is_filtering(&self) -> bool {
        self.filtering
    }

    /// Returns whether `g` is waiting for a second press.
    fn has_pending_g(&self) -> bool {
        self.pending_g
    }

    /// Updates whether `g` is waiting for a second press.
    fn set_pending_g(&mut self, pending: bool) {
        self.pending_g = pending;
    }
}

/// Verifies left-pane content receives semantic actions for common movement keys.
#[test]
fn routes_common_navigation_to_semantic_actions() {
    let mut content = FakeLeftPaneContent::default();

    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('j')));
    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('k')));
    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('G')));
    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('g')));
    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('g')));

    assert_eq!(
        content.actions,
        vec![
            LeftPaneAction::MoveBy(1),
            LeftPaneAction::MoveBy(-1),
            LeftPaneAction::FocusLast,
            LeftPaneAction::FocusFirst,
        ]
    );
}

/// Verifies filter mode routes typed keys as filter text instead of navigation.
#[test]
fn routes_filter_text_to_semantic_actions() {
    let mut content = FakeLeftPaneContent::default();

    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('/')));
    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('j')));
    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Backspace));

    assert_eq!(
        content.actions,
        vec![
            LeftPaneAction::StartFilter,
            LeftPaneAction::InsertFilterCharacter('j'),
            LeftPaneAction::DeleteFilterCharacter,
        ]
    );
}

/// Verifies adjacent-group shortcuts are routed as semantic actions.
#[test]
fn routes_adjacent_group_to_semantic_action() {
    let mut content = FakeLeftPaneContent::default();

    let _ = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('J')));

    assert_eq!(content.actions, vec![LeftPaneAction::FocusAdjacentGroup(1)]);
}

/// Verifies quit is routed as a semantic action owned by active content.
#[test]
fn routes_quit_to_active_content() {
    let mut content = FakeLeftPaneContent::default();

    let outcome = dispatch_left_pane_keyboard(&mut content, key(KeyCode::Char('q')));

    assert_eq!(content.actions, vec![LeftPaneAction::Quit]);
    assert_eq!(outcome, LeftPaneActionOutcome::Handled);
}

/// Builds a key press event without modifiers.
fn key(key_code: KeyCode) -> KeyboardEvent {
    KeyboardEvent {
        key_code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
    }
}
