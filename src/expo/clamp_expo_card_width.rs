use crate::expo::expo_card_width_limits::{max_expo_card_width, min_expo_card_width};

/// Clamps an Expo card target width to supported terminal-cell limits.
pub fn clamp_expo_card_width(width: u16) -> u16 {
    width.clamp(min_expo_card_width(), max_expo_card_width())
}

#[cfg(test)]
mod tests {
    use super::clamp_expo_card_width;

    /// Verifies card width cannot drop below the minimum.
    #[test]
    fn clamps_to_minimum_width() {
        assert_eq!(clamp_expo_card_width(1), 24);
    }

    /// Verifies card width cannot exceed the maximum.
    #[test]
    fn clamps_to_maximum_width() {
        assert_eq!(clamp_expo_card_width(200), 80);
    }
}
