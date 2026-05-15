use crate::extensions::command_bar::command::command_id::CommandBarCommandId;
use crate::extensions::command_bar::data::command_bar_state::CommandBarState;
use crate::extensions::command_bar::data::filtered_commands::filtered_command_bar_items;

/// Returns the command identifier selected by the current command bar state.
pub fn selected_command_bar_command(state: &CommandBarState) -> Option<CommandBarCommandId> {
    filtered_command_bar_items(&state.query)
        .get(state.selected_position)
        .map(|item| item.id)
}

/// Clamps the command bar selection to the currently visible command rows.
pub fn clamp_command_bar_selection(state: &mut CommandBarState) {
    let item_count = filtered_command_bar_items(&state.query).len();
    if item_count == 0 {
        state.selected_position = 0;
        return;
    }
    state.selected_position = state.selected_position.min(item_count.saturating_sub(1));
}
