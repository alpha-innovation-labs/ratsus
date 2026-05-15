use crate::app::state::app_state::AppState;
use crate::extensions::expo::filter::filtered_session_indices::filtered_expo_session_indices;
use crate::extensions::expo::input::keep_focused_card_visible::keep_focused_expo_card_visible;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;

/// Moves Expo focus through filtered conversation cards.
pub fn move_focused_expo_conversation(app: &mut AppState, delta: isize) {
    let indices = filtered_expo_session_indices(app);
    if indices.is_empty() {
        return;
    }
    let current = indices
        .iter()
        .position(|index| *index == app.focused_index)
        .unwrap_or(0);
    let next = wrapped_list_position(current, delta, indices.len());
    app.focused_index = indices[next];
    keep_focused_expo_card_visible(app);
}

#[cfg(test)]
mod tests {
    use super::move_focused_expo_conversation;

    /// Keeps the movement function linked from keyboard handling.
    #[test]
    fn movement_function_is_linked() {
        let _ = move_focused_expo_conversation;
    }
}
