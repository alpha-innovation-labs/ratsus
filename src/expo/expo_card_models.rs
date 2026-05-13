use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::expo_card_model::ExpoCardModel;
use crate::expo::expo_session_indices::expo_session_indices;
use crate::expo::filter_expo_cards::filter_expo_cards;

/// Builds Expo card models from session metadata and cached observation previews.
pub fn expo_card_models(app: &NexusDemo) -> Vec<ExpoCardModel> {
    let cards = expo_session_indices(app)
        .into_iter()
        .filter_map(|session_index| card_model_for_index(app, session_index))
        .collect::<Vec<_>>();
    filter_expo_cards(cards, &app.expo_filter_query)
}

/// Builds one Expo card model without reading from disk during rendering.
fn card_model_for_index(app: &NexusDemo, session_index: usize) -> Option<ExpoCardModel> {
    let session = app.session_terminals.get(session_index)?.session.clone();
    let preview = app
        .observation_previews
        .get(&session.id)
        .cloned()
        .unwrap_or_else(ConversationObservationPreview::default);
    Some(ExpoCardModel {
        session_index,
        title: session.title,
        preview,
    })
}
