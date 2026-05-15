use ratatui::layout::Rect;

/// Calculates the right-aligned status area on the menu-bar bottom row.
pub fn menu_bar_bottom_status_area(area: Rect, status: &str) -> Option<Rect> {
    let width = status.chars().count() as u16;
    if area.width <= 2 || area.height == 0 || width == 0 {
        return None;
    }

    let available_width = area.width.saturating_sub(2);
    let width = width.min(available_width);
    let x = area.x + area.width.saturating_sub(width + 1);
    let y = area.y + area.height.saturating_sub(1);
    Some(Rect::new(x, y, width, 1))
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use crate::ui::menu_bar::render::menu_bar_bottom_status_area::menu_bar_bottom_status_area;

    /// Verifies status text is placed on the menu-bar bottom row.
    #[test]
    fn places_status_on_bottom_row() {
        assert_eq!(
            menu_bar_bottom_status_area(Rect::new(0, 0, 80, 3), "FPS 0"),
            Some(Rect::new(74, 2, 5, 1))
        );
    }
}
