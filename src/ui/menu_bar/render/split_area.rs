use ratatui::layout::Rect;

const NEXUS_MENU_BAR_HEIGHT: u16 = 3;

/// Splits the frame into a top menu bar area and the remaining application body.
pub fn split_app_menu_bar_area(area: Rect) -> (Rect, Rect) {
    let menu_height = area.height.min(NEXUS_MENU_BAR_HEIGHT);
    let menu_area = Rect::new(area.x, area.y, area.width, menu_height);
    let body_area = Rect::new(
        area.x,
        area.y.saturating_add(menu_height),
        area.width,
        area.height.saturating_sub(menu_height),
    );
    (menu_area, body_area)
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::split_app_menu_bar_area;

    /// Verifies that the menu bar reserves three rows above the app body.
    #[test]
    fn reserves_top_menu_rows() {
        assert_eq!(
            split_app_menu_bar_area(Rect::new(0, 0, 120, 40)),
            (Rect::new(0, 0, 120, 3), Rect::new(0, 3, 120, 37))
        );
    }

    /// Verifies that tiny frames do not underflow when reserving menu space.
    #[test]
    fn handles_tiny_frames() {
        assert_eq!(
            split_app_menu_bar_area(Rect::new(0, 0, 120, 2)),
            (Rect::new(0, 0, 120, 2), Rect::new(0, 2, 120, 0))
        );
    }
}
