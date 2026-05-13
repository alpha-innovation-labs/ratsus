use ratatui::layout::Rect;

/// Converts an unscrolled card area into its clipped scrolled viewport area.
pub fn expo_card_visible_area(card: Rect, viewport: Rect, scroll: usize) -> Option<Rect> {
    let scrolled_top = (card.y as isize).saturating_sub(scroll as isize);
    let scrolled_bottom = scrolled_top.saturating_add(card.height as isize);
    let viewport_top = viewport.y as isize;
    let viewport_bottom = viewport.y.saturating_add(viewport.height) as isize;
    let visible_top = scrolled_top.max(viewport_top);
    let visible_bottom = scrolled_bottom.min(viewport_bottom);
    if visible_bottom <= visible_top {
        return None;
    }
    Some(Rect::new(
        card.x,
        visible_top as u16,
        card.width,
        (visible_bottom - visible_top) as u16,
    ))
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::expo_card_visible_area;

    /// Verifies oversized cards are clipped to the viewport instead of disappearing.
    #[test]
    fn clips_tall_card_to_viewport_bottom() {
        assert_eq!(
            expo_card_visible_area(Rect::new(2, 4, 30, 20), Rect::new(0, 4, 40, 8), 0),
            Some(Rect::new(2, 4, 30, 8))
        );
    }

    /// Verifies cards partially above the viewport are clipped instead of dropped.
    #[test]
    fn clips_card_above_viewport_top() {
        assert_eq!(
            expo_card_visible_area(Rect::new(2, 4, 30, 20), Rect::new(0, 4, 40, 8), 6),
            Some(Rect::new(2, 4, 30, 8))
        );
    }
}
