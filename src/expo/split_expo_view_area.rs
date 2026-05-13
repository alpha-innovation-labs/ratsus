use ratatui::layout::Rect;

/// Splits Expo into scrollable card body and one-line footer/search area.
pub fn split_expo_view_area(area: Rect) -> (Rect, Rect) {
    if area.height <= 1 {
        return (area, Rect::default());
    }
    let body_height = area.height.saturating_sub(1);
    (
        Rect::new(area.x, area.y, area.width, body_height),
        Rect::new(area.x, area.y.saturating_add(body_height), area.width, 1),
    )
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::split_expo_view_area;

    /// Verifies Expo reserves the final row for footer or search input.
    #[test]
    fn reserves_footer_row() {
        assert_eq!(
            split_expo_view_area(Rect::new(2, 3, 40, 10)),
            (Rect::new(2, 3, 40, 9), Rect::new(2, 12, 40, 1))
        );
    }
}
