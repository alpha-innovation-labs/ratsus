use crate::app::state::app_state::AppState;
use crate::extensions::expo::card::models::expo_card_models;

/// Returns session indexes for cards matching the current Expo filter.
pub fn filtered_expo_session_indices(app: &AppState) -> Vec<usize> {
    expo_card_models(app)
        .into_iter()
        .map(|card| card.session_index)
        .collect()
}
