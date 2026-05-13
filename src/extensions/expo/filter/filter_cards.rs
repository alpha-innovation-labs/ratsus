use crate::extensions::expo::card::matches_filter::expo_card_matches_filter;
use crate::extensions::expo::card::model::ExpoCardModel;

/// Keeps only Expo cards matching the current filter query.
pub fn filter_expo_cards(cards: Vec<ExpoCardModel>, filter: &str) -> Vec<ExpoCardModel> {
    cards
        .into_iter()
        .filter(|card| expo_card_matches_filter(card, filter))
        .collect()
}
