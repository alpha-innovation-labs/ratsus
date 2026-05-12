use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{PaneId, PaneLayout};

/// Finds a pane rectangle by pane id in computed grid layouts.
pub fn pane_area_by_id(layouts: &[PaneLayout], pane_id: PaneId) -> Rect {
    layouts
        .iter()
        .find(|layout| layout.pane_id() == pane_id)
        .map(|layout| layout.area())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;
    use ratkit::primitives::resizable_grid::ResizableGrid;

    use super::pane_area_by_id;

    /// Verifies that a pane area can be found from computed grid layouts.
    #[test]
    fn finds_pane_area_by_id() {
        let mut grid = ResizableGrid::new(0);
        let _ = grid.split_pane_vertically(0);
        grid.set_split_percent(20);
        let layouts = grid.layout_panes(Rect::new(0, 0, 100, 20));

        assert_eq!(pane_area_by_id(&layouts, 0), Rect::new(0, 0, 20, 20));
        assert_eq!(pane_area_by_id(&layouts, 1), Rect::new(20, 0, 80, 20));
    }
}
