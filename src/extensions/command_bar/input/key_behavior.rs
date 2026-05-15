use crate::extensions::command_bar::command::command_id::CommandBarCommandId;
use crate::extensions::command_bar::data::command_bar_state::CommandBarState;
use crate::extensions::command_bar::data::filtered_commands::filtered_command_bar_items;
use crate::extensions::command_bar::data::selected_command::{
    clamp_command_bar_selection, selected_command_bar_command,
};
use crate::ui::keyboard::list::behavior::ListKeyBehavior;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;

/// Adapts command bar state to the shared list keyboard contract.
pub struct CommandBarKeyBehavior<'a> {
    state: &'a mut CommandBarState,
    activated_command: Option<CommandBarCommandId>,
}

impl<'a> CommandBarKeyBehavior<'a> {
    /// Creates a shared-key adapter around mutable command bar state.
    pub fn new(state: &'a mut CommandBarState) -> Self {
        Self {
            state,
            activated_command: None,
        }
    }

    /// Takes the command selected by an activation key, when present.
    pub fn take_activated_command(&mut self) -> Option<CommandBarCommandId> {
        self.activated_command.take()
    }
}

impl ListKeyBehavior for CommandBarKeyBehavior<'_> {
    /// Moves command bar selection by a signed row delta.
    fn move_selection(&mut self, direction: isize) {
        let item_count = filtered_command_bar_items(&self.state.query).len();
        if item_count == 0 {
            self.state.selected_position = 0;
            return;
        }
        self.state.selected_position =
            wrapped_list_position(self.state.selected_position, direction, item_count);
    }

    /// Moves command bar selection to the first row.
    fn focus_first(&mut self) {
        self.state.selected_position = 0;
    }

    /// Moves command bar selection to the last visible row.
    fn focus_last(&mut self) {
        let item_count = filtered_command_bar_items(&self.state.query).len();
        self.state.selected_position = item_count.saturating_sub(1);
    }

    /// Captures the currently selected command for execution.
    fn activate_selection(&mut self) {
        self.activated_command = selected_command_bar_command(self.state);
    }

    /// Closes command bar filter mode or the modal itself.
    fn close_selection_context(&mut self) -> bool {
        if self.state.is_filtering {
            self.state.query.clear();
            self.state.selected_position = 0;
            self.state.is_filtering = false;
            return true;
        }
        self.state.is_open = false;
        true
    }

    /// Returns whether typed characters should edit the command filter.
    fn is_filtering(&self) -> bool {
        self.state.is_filtering
    }

    /// Starts command filtering.
    fn start_filtering(&mut self) {
        self.state.is_filtering = true;
    }

    /// Adds one character to the command filter query.
    fn insert_filter_character(&mut self, character: char) {
        self.state.query.push(character);
        clamp_command_bar_selection(self.state);
    }

    /// Removes one character from the command filter query.
    fn delete_filter_character(&mut self) {
        self.state.query.pop();
        clamp_command_bar_selection(self.state);
    }

    /// Returns whether the command bar is waiting for a second `g` key.
    fn has_pending_g(&self) -> bool {
        self.state.pending_g
    }

    /// Updates whether the command bar is waiting for a second `g` key.
    fn set_pending_g(&mut self, pending: bool) {
        self.state.pending_g = pending;
    }
}
