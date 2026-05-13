use crate::app::app_state::AppState;
use crate::expo::filtered_expo_session_indices::filtered_expo_session_indices;
use crate::expo::keep_focused_expo_card_visible::keep_focused_expo_card_visible;

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
    let next = current.saturating_add_signed(delta).min(indices.len() - 1);
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
