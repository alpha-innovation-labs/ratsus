use ratatui::style::{Modifier, Style};
use ratatui::text::Line;

use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Builds the left-pane title for the selected navbar mode.
pub fn left_pane_mode_title_line(
    mode: LeftPaneMode,
    color: ratatui::style::Color,
) -> Line<'static> {
    Line::styled(left_pane_mode_title(mode), title_style(color))
}

/// Returns the visible title text for one left-pane mode.
fn left_pane_mode_title(mode: LeftPaneMode) -> &'static str {
    match mode {
        LeftPaneMode::Sessions => " Sessions ",
        LeftPaneMode::Plans => " Plans ",
        LeftPaneMode::Files => " Files ",
    }
}

/// Returns the style for the left-pane title, matching the border color.
fn title_style(color: ratatui::style::Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::left_pane_mode_title_line;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

    /// Verifies the title shows only the active mode label with the supplied border color.
    #[test]
    fn title_uses_active_mode_and_border_color() {
        let line = left_pane_mode_title_line(LeftPaneMode::Files, Color::Red);

        assert_eq!(line.spans[0].content.as_ref(), " Files ");
        assert_eq!(line.style.fg, Some(Color::Red));
    }
}
