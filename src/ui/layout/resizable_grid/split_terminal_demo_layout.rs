use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout areas used by the terminal demo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalDemoLayout {
    pub left_pane: Rect,
    pub terminal_pane: Rect,
}

/// Splits the demo into a 20% left pane and an 80% terminal pane.
pub fn split_terminal_demo_layout(area: Rect) -> TerminalDemoLayout {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);

    TerminalDemoLayout {
        left_pane: chunks[0],
        terminal_pane: chunks[1],
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::{split_terminal_demo_layout, TerminalDemoLayout};

    /// Verifies that the demo reserves 20% of the width for the left pane.
    #[test]
    fn reserves_twenty_percent_for_left_pane() {
        assert_eq!(
            split_terminal_demo_layout(Rect::new(0, 0, 100, 40)),
            TerminalDemoLayout {
                left_pane: Rect::new(0, 0, 20, 40),
                terminal_pane: Rect::new(20, 0, 80, 40),
            }
        );
    }

    /// Verifies that non-zero origins are preserved when splitting the layout.
    #[test]
    fn preserves_origin_when_splitting() {
        assert_eq!(
            split_terminal_demo_layout(Rect::new(3, 2, 50, 10)),
            TerminalDemoLayout {
                left_pane: Rect::new(3, 2, 10, 10),
                terminal_pane: Rect::new(13, 2, 40, 10),
            }
        );
    }
}
