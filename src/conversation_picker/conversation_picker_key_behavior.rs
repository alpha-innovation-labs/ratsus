use crate::app::app_state::AppState;
use crate::conversation_picker::activate_selected_conversation::activate_selected_conversation;
use crate::conversation_picker::apply_conversation_picker_query_change::apply_conversation_picker_query_change;
use crate::conversation_picker::collapse_selected_conversation_project::collapse_selected_conversation_project;
use crate::conversation_picker::current_conversation_picker_item_count::current_conversation_picker_item_count;
use crate::conversation_picker::focus_conversation_picker_end::focus_conversation_picker_end;
use crate::conversation_picker::focus_conversation_picker_start::focus_conversation_picker_start;
use crate::conversation_picker::handle_conversation_picker_escape::handle_conversation_picker_escape;
use crate::conversation_picker::move_conversation_picker_selection::move_conversation_picker_selection;
use crate::conversation_picker::open_selected_conversation_delete_confirmation::open_selected_conversation_delete_confirmation;
use crate::conversation_picker::open_selected_conversation_project::open_selected_conversation_project;
use crate::conversation_picker::reorder_selected_conversation::reorder_selected_conversation;
use crate::conversation_picker::toggle_selected_conversation_picker_item::toggle_selected_conversation_picker_item;
use crate::keyboard::list_key_behavior::ListKeyBehavior;

/// Adapts the conversation picker modal to the shared list keyboard contract.
pub struct ConversationPickerKeyBehavior<'a> {
    app: &'a mut AppState,
}

impl<'a> ConversationPickerKeyBehavior<'a> {
    /// Creates a shared-key adapter for the conversation picker.
    pub fn new(app: &'a mut AppState) -> Self {
        Self { app }
    }
}

impl ListKeyBehavior for ConversationPickerKeyBehavior<'_> {
    /// Moves picker selection by a signed row delta.
    fn move_selection(&mut self, direction: isize) {
        let item_count = current_conversation_picker_item_count(self.app);
        move_conversation_picker_selection(
            &mut self.app.conversation_picker,
            direction,
            item_count,
        );
    }

    /// Moves picker selection to the first visible row.
    fn focus_first(&mut self) {
        focus_conversation_picker_start(self.app);
    }

    /// Moves picker selection to the last visible row.
    fn focus_last(&mut self) {
        focus_conversation_picker_end(self.app);
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
        toggle_selected_conversation_picker_item(self.app);
    }

    /// Reorders the selected picker session by one visible conversation.
    fn reorder_selection(&mut self, direction: isize) {
        reorder_selected_conversation(self.app, direction);
    }

    /// Handles staged Escape behavior for filter mode, query clearing, and closing.
    fn close_selection_context(&mut self) -> bool {
        handle_conversation_picker_escape(self.app);
        true
    }

    /// Returns whether picker filter-entry mode is active.
    fn is_filtering(&self) -> bool {
        self.app.conversation_picker.is_filtering
    }

    /// Starts picker filter-entry mode.
    fn start_filtering(&mut self) {
        self.app.conversation_picker.is_filtering = true;
    }

    /// Inserts one character into the picker filter.
    fn insert_filter_character(&mut self, character: char) {
        self.app.conversation_picker.query.push(character);
        apply_conversation_picker_query_change(self.app);
    }

    /// Removes one character from the picker filter.
    fn delete_filter_character(&mut self) {
        self.app.conversation_picker.query.pop();
        apply_conversation_picker_query_change(self.app);
    }

    /// Returns whether the picker is waiting for a second `g`.
    fn has_pending_g(&self) -> bool {
        self.app.conversation_picker.pending_g
    }

    /// Updates whether the picker is waiting for a second `g`.
    fn set_pending_g(&mut self, pending: bool) {
        self.app.conversation_picker.pending_g = pending;
    }
}
