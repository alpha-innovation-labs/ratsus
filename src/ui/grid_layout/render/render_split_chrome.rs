use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::core::rendering::style::default_border_color::default_border_color;
use crate::ui::grid_layout::bundle::pane_session_ids::terminal_pane_session_ids;
use crate::ui::grid_layout::group::pane_group_name::pane_group_name;
use crate::ui::grid_layout::pane::close_button_area::terminal_pane_close_button_area;
use crate::ui::grid_layout::pane::session_index_for_pane::session_index_for_pane;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Renders split-pane border chrome and returns the inner content area.
pub fn render_split_chrome(app: &AppState, frame: &mut Frame, pane_id: PaneId, area: Rect) -> Rect {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style(app, pane_id))
        .title(Line::from(split_title(app, pane_id)).style(title_style()));
    frame.render_widget(block, area);
    if let Some(close_area) = close_title_area(area) {
        frame.render_widget(close_title(), close_area);
    }
    content_area(area)
}

/// Returns the close-button hit target for visible split-pane chrome.
pub fn visible_close_button_area(area: Rect) -> Option<Rect> {
    close_title_area(area)
}

/// Returns the content area inside split-pane borders.
pub fn content_area(area: Rect) -> Rect {
    Rect::new(
        area.x.saturating_add(1),
        area.y.saturating_add(1),
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

/// Returns the three-cell close title area for a pane.
fn close_title_area(area: Rect) -> Option<Rect> {
    terminal_pane_close_button_area(area)
        .map(|area| Rect::new(area.x.saturating_sub(1), area.y, 3, 1))
}

/// Returns a compact title for split pane chrome.
fn split_title(app: &AppState, pane_id: PaneId) -> String {
    if let Some(group_name) = pane_group_name(&app.split_pane_session_groups, pane_id) {
        return group_name.to_string();
    }
    session_index_for_pane(app, pane_id)
        .and_then(|index| app.session_terminals.get(index))
        .map(|entry| title_for_entry(app, pane_id, entry.session.title.clone()))
        .unwrap_or_else(|| "session".to_string())
}

/// Returns a title with bundle count when a pane contains multiple sessions.
fn title_for_entry(app: &AppState, pane_id: PaneId, title: String) -> String {
    let bundle_count = terminal_pane_session_ids(app, pane_id).len();
    if bundle_count > 1 {
        format!("{title} ({bundle_count})")
    } else {
        title
    }
}

/// Returns the style for split pane title text.
fn title_style() -> Style {
    Style::default().fg(Color::White)
}

/// Returns the close button title for split pane chrome.
fn close_title() -> Line<'static> {
    Line::from(Span::styled(
        " x ",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ))
}

/// Returns the border style for a pane.
fn border_style(app: &AppState, pane_id: PaneId) -> Style {
    if pane_id == app.active_terminal_pane_id && app.focused_pane == FocusedPane::Terminal {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(default_border_color())
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::{split_title, title_style};
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroup;
    use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;

    /// Verifies grouped split panes show the shared group name as their pane title.
    #[test]
    fn grouped_pane_title_uses_group_name() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.terminal_pane_sessions
            .insert(TERMINAL_PANE_ID, "a".to_string());
        app.terminal_pane_session_bundles
            .insert(TERMINAL_PANE_ID, vec!["a".to_string()]);
        app.split_pane_session_groups.groups.insert(
            1,
            SplitPaneSessionGroup {
                id: 1,
                name: "Group 1".to_string(),
                panes: vec![TERMINAL_PANE_ID],
            },
        );

        assert_eq!(split_title(&app, TERMINAL_PANE_ID), "Group 1");
        assert_eq!(title_style().fg, Some(Color::White));
        Ok(())
    }
}
