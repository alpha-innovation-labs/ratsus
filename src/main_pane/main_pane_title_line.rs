use ratatui::text::Line;

use crate::main_pane::main_pane_tab::MainPaneTab;

/// Builds the main pane title for the currently selected content tab.
pub fn main_pane_title_line(selected: MainPaneTab, expo_count: usize) -> Line<'static> {
    let label = match selected {
        MainPaneTab::Chat => "Chat".to_string(),
        MainPaneTab::Files => "Files".to_string(),
        MainPaneTab::Diff => "Diff".to_string(),
        MainPaneTab::Expo => format!("Expo · {expo_count} conversations"),
    };
    Line::from(label)
}
