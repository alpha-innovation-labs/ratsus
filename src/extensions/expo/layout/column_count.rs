use crate::extensions::expo::card::clamp_width::clamp_expo_card_width;

/// Returns the Expo masonry column count for a target card width.
pub fn expo_column_count(area_width: u16, card_width: u16) -> usize {
    let card_width = clamp_expo_card_width(card_width);
    usize::from((area_width / card_width).max(1))
}

#[cfg(test)]
mod tests {
    use super::expo_column_count;

    /// Verifies default-width cards keep the existing column behavior.
    #[test]
    fn uses_default_card_width_columns() {
        assert_eq!(expo_column_count(37, 38), 1);
        assert_eq!(expo_column_count(76, 38), 2);
        assert_eq!(expo_column_count(114, 38), 3);
    }

    /// Verifies wider cards reduce the number of columns.
    #[test]
    fn wider_cards_reduce_columns() {
        assert_eq!(expo_column_count(120, 60), 2);
    }

    /// Verifies narrower cards increase the number of columns.
    #[test]
    fn narrower_cards_increase_columns() {
        assert_eq!(expo_column_count(120, 24), 5);
    }
}
