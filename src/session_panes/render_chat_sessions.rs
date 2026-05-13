use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::nexus_demo_state::NexusDemo;
use crate::copy_mode::is_terminal_copy_selection_active::is_terminal_copy_selection_active;
use crate::copy_mode::render_screen_with_selection::render_screen_with_selection;
use crate::layout::focused_pane::FocusedPane;
use crate::rendering::apply_cursor_style::apply_cursor_style;
use crate::rendering::default_border_color::default_border_color;
use crate::session_panes::ensure_active_terminal_pane_session::ensure_active_terminal_pane_session;
use crate::session_panes::resize_terminal_pane_session::resize_terminal_pane_session;
use crate::session_panes::session_index_for_pane::session_index_for_pane;
use crate::session_panes::terminal_pane_close_button_area::terminal_pane_close_button_area;
use crate::session_panes::terminal_pane_session_ids::terminal_pane_session_ids;
use crate::terminal::nexus_terminal::NexusTerminal;

/// Renders every visible chat split and updates terminal sizes to match pane areas.
pub fn render_chat_sessions(app: &mut NexusDemo, frame: &mut Frame, area: Rect) {
    ensure_active_terminal_pane_session(app);
    let panes = app.terminal_layout.layout_panes(area);
    app.terminal_pane_close_buttons.clear();
    let use_inner_borders = panes.len() > 1;
    for pane in panes {
        let pane_id = pane.pane_id();
        let pane_area = pane.area();
        let content_area = render_split_chrome(app, frame, pane_id, pane_area, use_inner_borders);
        if use_inner_borders {
            register_close_button(app, pane_id, pane_area);
        }
        resize_terminal_pane_session(app, pane_id, content_area);
        render_pane_session(app, frame, pane_id, content_area);
    }
}

/// Renders optional chrome around split panes and returns the pane content area.
fn render_split_chrome(
    app: &NexusDemo,
    frame: &mut Frame,
    pane_id: u32,
    area: Rect,
    use_inner_borders: bool,
) -> Rect {
    if !use_inner_borders {
        return area;
    }
    let border_style =
        if pane_id == app.active_terminal_pane_id && app.focused_pane == FocusedPane::Terminal {
            Style::default().fg(ratatui::style::Color::Cyan)
        } else {
            Style::default().fg(default_border_color())
        };
    let block = Block::default()
        .title(split_title(app, pane_id))
        .title(close_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

/// Stores the close-button hit target for one rendered split pane.
fn register_close_button(app: &mut NexusDemo, pane_id: u32, area: Rect) {
    if let Some(button_area) = terminal_pane_close_button_area(area) {
        app.terminal_pane_close_buttons.insert(pane_id, button_area);
    }
}

/// Returns the right-aligned close button title for split pane chrome.
fn close_title() -> Line<'static> {
    Line::from(Span::styled(
        " x ",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ))
    .right_aligned()
}

/// Returns a compact title for a split terminal pane.
fn split_title(app: &NexusDemo, pane_id: u32) -> String {
    session_index_for_pane(app, pane_id)
        .and_then(|index| app.session_terminals.get(index))
        .map(|entry| {
            let bundle_count = terminal_pane_session_ids(app, pane_id).len();
            if bundle_count > 1 {
                format!(" {} ({bundle_count}) ", entry.session.title)
            } else {
                format!(" {} ", entry.session.title)
            }
        })
        .unwrap_or_else(|| " session ".to_string())
}

/// Renders the terminal or placeholder assigned to a split pane.
fn render_pane_session(app: &NexusDemo, frame: &mut Frame, pane_id: u32, area: Rect) {
    let Some(index) = session_index_for_pane(app, pane_id) else {
        frame.render_widget(Paragraph::new("No session assigned"), area);
        return;
    };
    let Some(entry) = app.session_terminals.get(index) else {
        frame.render_widget(Paragraph::new("Session unavailable"), area);
        return;
    };
    if is_terminal_copy_selection_active(&entry.copy_selection) {
        if let Some(snapshot) = entry.copy_selection.snapshot.as_ref() {
            render_screen_with_selection(snapshot, &entry.copy_selection, area, frame.buffer_mut());
        }
    } else if let Some(terminal) = entry.terminal.as_ref() {
        terminal.render(frame, area);
        render_terminal_cursor(app, pane_id, terminal, frame, area);
    } else {
        frame.render_widget(Paragraph::new("Starting Nexus session…"), area);
    }
}

/// Renders the cursor for the active terminal split when it is visible.
fn render_terminal_cursor(
    app: &NexusDemo,
    pane_id: u32,
    terminal: &NexusTerminal,
    frame: &mut Frame,
    area: Rect,
) {
    if app.focused_pane != FocusedPane::Terminal || pane_id != app.active_terminal_pane_id {
        return;
    }
    if let Some(cursor) = terminal.cursor_state() {
        let x = area.x.saturating_add(cursor.col);
        let y = area.y.saturating_add(cursor.row);
        if x < area.x + area.width && y < area.y + area.height {
            frame.set_cursor_position((x, y));
            apply_cursor_style(cursor.style);
        }
    }
}
