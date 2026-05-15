use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Builds the top-bar title line for the session/plan toggle.
pub fn left_pane_mode_title_line(mode: LeftPaneMode) -> Line<'static> {
    Line::from(vec![
        toggle_span(" Sessions ", mode == LeftPaneMode::Sessions),
        Span::styled("|", Style::default().fg(Color::DarkGray)),
        toggle_span(" Plans ", mode == LeftPaneMode::Plans),
    ])
}

/// Styles one title toggle label according to active state.
fn toggle_span(label: &'static str, active: bool) -> Span<'static> {
    let style = if active {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    Span::styled(label, style)
}
