use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::rendering::default_border_color::default_border_color;

/// Renders the file-system tree without a nested pane around the tree content.
pub fn render_file_system_tree_view(view: &mut FileSystemTreeView, frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    view.last_tree_area = layout[0];
    let tree = view.tree.clone();
    frame.render_stateful_widget(tree, layout[0], &mut view.state);

    let footer = Paragraph::new(vec![
        Line::from("j/k or Up/Down move, Enter toggle, h/l collapse/expand, / filter"),
        Line::from(format!("Selected: {}", view.last_selection)),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_border_color()))
            .title(" Status "),
    );
    frame.render_widget(footer, layout[1]);
}
