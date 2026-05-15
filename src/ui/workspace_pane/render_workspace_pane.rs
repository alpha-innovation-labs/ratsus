use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::state::app_state::AppState;
use crate::core::rendering::resize::render_resize_placeholder::render_resize_placeholder;
use crate::core::rendering::style::default_border_color::default_border_color;
use crate::ui::workspace_pane::workspace_lines::workspace_lines;

/// Renders the workspace pane shell and folder rows.
pub fn render_workspace_pane(app: &mut AppState, frame: &mut Frame, area: Rect, is_resizing: bool) {
    let block = Block::default()
        .title(Line::styled(
            " Workspaces ",
            Style::default().fg(Color::White),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(default_border_color()));
    let inner = block.inner(area);
    app.last_workspace_list_area = inner;
    frame.render_widget(block, area);
    if is_resizing {
        render_resize_placeholder(inner, frame.buffer_mut());
    } else {
        frame.render_widget(Paragraph::new(workspace_lines(app)), inner);
    }
}
