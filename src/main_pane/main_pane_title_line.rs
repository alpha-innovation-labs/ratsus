use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::main_pane::main_pane_tab::MainPaneTab;

/// Builds the styled title-bar tabs for the main pane.
pub fn main_pane_title_line(selected: MainPaneTab) -> Line<'static> {
    Line::from(vec![
        tab_span("Chat", selected == MainPaneTab::Chat),
        Span::raw(" · "),
        tab_span("Files", selected == MainPaneTab::Files),
        Span::raw(" · "),
        tab_span("Diff", selected == MainPaneTab::Diff),
    ])
}

/// Builds one tab label span with a background when selected.
fn tab_span(label: &'static str, selected: bool) -> Span<'static> {
    if selected {
        return Span::styled(label, Style::default().fg(Color::White).bg(Color::Blue));
    }
    Span::raw(label)
}
