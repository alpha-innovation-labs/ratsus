use crate::expo::expo_card_area::ExpoCardArea;

/// Returns the session index for the Expo card under the given position.
pub fn expo_card_session_index_at_position(
    cards: &[ExpoCardArea],
    column: u16,
    row: u16,
) -> Option<usize> {
    cards
        .iter()
        .find(|card| position_is_inside_card(card, column, row))
        .map(|card| card.session_index)
}

/// Returns whether a terminal position is inside one Expo card area.
fn position_is_inside_card(card: &ExpoCardArea, column: u16, row: u16) -> bool {
    column >= card.area.x
        && column < card.area.x.saturating_add(card.area.width)
        && row >= card.area.y
        && row < card.area.y.saturating_add(card.area.height)
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::expo_card_session_index_at_position;
    use crate::expo::expo_card_area::ExpoCardArea;

    /// Verifies that a point inside an Expo card resolves to that card's session index.
    #[test]
    fn returns_session_index_for_card_hit() {
        let cards = [ExpoCardArea {
            session_index: 7,
            area: Rect::new(10, 5, 20, 4),
        }];

        assert_eq!(expo_card_session_index_at_position(&cards, 12, 6), Some(7));
    }

    /// Verifies that positions outside all cards are ignored.
    #[test]
    fn ignores_position_outside_cards() {
        let cards = [ExpoCardArea {
            session_index: 7,
            area: Rect::new(10, 5, 20, 4),
        }];

        assert_eq!(expo_card_session_index_at_position(&cards, 31, 6), None);
    }
}
