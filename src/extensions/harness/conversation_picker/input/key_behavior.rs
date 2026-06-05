use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::actions::activate_selected::activate_selected_conversation;
use crate::extensions::history_modal::actions::apply_query_change::apply_history_modal_query_change;
use crate::extensions::history_modal::actions::collapse_selected_project::collapse_selected_conversation_project;
use crate::extensions::history_modal::actions::open_selected_delete_confirmation::open_selected_conversation_delete_confirmation;
use crate::extensions::history_modal::actions::open_selected_project::open_selected_conversation_project;
use crate::extensions::history_modal::actions::reorder_selected::reorder_selected_conversation;
use crate::extensions::history_modal::actions::toggle_selected_item::toggle_selected_history_modal_item;
use crate::extensions::history_modal::data::current_item_count::current_history_modal_item_count;
use crate::extensions::history_modal::input::handle_escape::handle_history_modal_escape;
use crate::extensions::history_modal::selection::focus_end::focus_history_modal_end;
use crate::extensions::history_modal::selection::focus_start::focus_history_modal_start;
use crate::extensions::history_modal::selection::move_selection::move_history_modal_selection;
use crate::ui::keyboard::list::behavior::ListKeyBehavior;

/// Adapts the conversation picker modal to the shared list keyboard contract.
pub struct HistoryModalKeyBehavior<'a> {
    app: &'a mut AppState,
}

impl<'a> HistoryModalKeyBehavior<'a> {
    /// Creates a shared-key adapter for the conversation picker.
    pub fn new(app: &'a mut AppState) -> Self {
        Self { app }
    }
}

impl ListKeyBehavior for HistoryModalKeyBehavior<'_> {
    /// Moves picker selection by a signed row delta.
    fn move_selection(&mut self, direction: isize) {
        let item_count = current_history_modal_item_count(self.app);
        move_history_modal_selection(
            &mut self.app.history_modal,
            direction,
            item_count,
        );
    }

    /// Moves picker selection to the first visible row.
    fn focus_first(&mut self) {
        focus_history_modal_start(self.app);
    }

    /// Moves picker selection to the last visible row.
    fn focus_last(&mut self) {
        focus_history_modal_end(self.app);
    }

    /// Activates the selected picker row.
    fn activate_selection(&mut self) {
        activate_selected_conversation(self.app);
    }

    /// Collapses the selected picker row's project folder.
    fn collapse_selection(&mut self) {
        collapse_selected_conversation_project(self.app);
    }

    /// Opens the selected picker row's project folder.
    fn open_selection(&mut self) {
        open_selected_conversation_project(self.app);
    }

    /// Opens delete confirmation for the selected picker session.
    fn delete_selection(&mut self) {
        open_selected_conversation_delete_confirmation(self.app);
    }

    /// Toggles bulk-selection state for the selected picker session.
    fn toggle_selection(&mut self) {
        toggle_selected_history_modal_item(self.app);
    }

    /// Reorders the selected picker session by one visible conversation.
    fn reorder_selection(&mut self, direction: isize) {
        reorder_selected_conversation(self.app, direction);
    }

    /// Handles staged Escape behavior for filter mode, query clearing, and closing.
    fn close_selection_context(&mut self) -> bool {
        handle_history_modal_escape(self.app);
        true
    }

    /// Returns whether picker filter-entry mode is active.
    fn is_filtering(&self) -> bool {
        self.app.history_modal.is_filtering
    }

    /// Starts picker filter-entry mode.
    fn start_filtering(&mut self) {
        self.app.history_modal.is_filtering = true;
    }

    /// Inserts one character into the picker filter.
    fn insert_filter_character(&mut self, character: char) {
        self.app.history_modal.query.push(character);
        apply_history_modal_query_change(self.app);
    }

    /// Removes one character from the picker filter.
    fn delete_filter_character(&mut self) {
        self.app.history_modal.query.pop();
        apply_history_modal_query_change(self.app);
    }

    /// Returns whether the picker is waiting for a second `g`.
    fn has_pending_g(&self) -> bool {
        self.app.history_modal.pending_g
    }

    /// Updates whether the picker is waiting for a second `g`.
    fn set_pending_g(&mut self, pending: bool) {
        self.app.history_modal.pending_g = pending;
    }
}
