use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use ratkit::primitives::resizable_grid::ResizableGridWidget;
use ratkit::primitives::toast::render_toasts;

use crate::apply_cursor_style::apply_cursor_style;
use crate::focused_pane::FocusedPane;
use crate::is_resizing_layout::is_resizing_layout;
use crate::is_terminal_copy_selection_active::is_terminal_copy_selection_active;
use crate::nexus_demo_state::{NexusDemo, LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::pane_area_by_id::pane_area_by_id;
use crate::render_resize_placeholder::render_resize_placeholder;
use crate::render_screen_with_selection::render_screen_with_selection;
use crate::session_lines::session_lines;

/// Renders the full terminal demo, including session list and active session terminal.
pub fn render_nexus_demo(app: &mut NexusDemo, frame: &mut Frame) {
    let area = frame.area();
    app.last_layout_area = area;
    let (left_pane, terminal_pane) = visible_pane_areas(app, area);
    app.last_left_area = left_pane;

    let is_resizing = is_resizing_layout(&app.layout_widget_state);
    if app.left_pane_visible {
        let left_inner = render_left_pane(app, frame, left_pane);
        app.last_session_list_area = left_inner;
        app.keep_focused_session_visible();
        if is_resizing {
            render_resize_placeholder(left_inner, frame.buffer_mut());
        } else {
            frame.render_widget(Paragraph::new(session_lines(app)), left_inner);
        }
    } else {
        app.last_session_list_area = Rect::default();
    }

    let terminal_inner = render_terminal_pane(app, frame, terminal_pane);
    app.resize_active_terminal(terminal_inner);
    if is_resizing {
        render_resize_placeholder(terminal_inner, frame.buffer_mut());
    } else {
        render_active_terminal(app, frame, terminal_inner);
    }
    if app.left_pane_visible {
        render_resizable_grid_overlay(app, frame, area);
    }
    app.toast_manager.remove_expired();
    render_toasts(frame, &app.toast_manager);
}

/// Returns pane areas for the current left-pane visibility state.
fn visible_pane_areas(app: &NexusDemo, area: Rect) -> (Rect, Rect) {
    if !app.left_pane_visible {
        return (Rect::default(), area);
    }
    let pane_layouts = app.layout.layout_panes(area);
    (
        pane_area_by_id(&pane_layouts, LEFT_PANE_ID),
        pane_area_by_id(&pane_layouts, TERMINAL_PANE_ID),
    )
}

/// Renders the active resizable grid divider overlay.
fn render_resizable_grid_overlay(
    app: &mut NexusDemo,
    frame: &mut Frame,
    area: ratatui::layout::Rect,
) {
    let widget = ResizableGridWidget::new(app.layout.clone())
        .with_state(app.layout_widget_state)
        .with_pane_borders(false);
    app.layout_widget_state = widget.state();
    frame.render_widget(widget, area);
}

/// Renders the rounded left session pane and returns its content area.
fn render_left_pane(
    app: &NexusDemo,
    frame: &mut Frame,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let border_style = if app.focused_pane == FocusedPane::Left {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let block = Block::default()
        .title(" nexus sessions ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

/// Renders the rounded terminal pane and returns its content area.
fn render_terminal_pane(
    app: &NexusDemo,
    frame: &mut Frame,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let title = if app
        .active_session_terminal()
        .is_some_and(|entry| entry.copy_selection.snapshot.is_some())
    {
        " session terminal  COPY MODE  <c: copy> <Esc: leave> "
    } else {
        match app.focused_pane {
            FocusedPane::Terminal => " session terminal (focused, wheel scrolls history) ",
            FocusedPane::Left => " session terminal (left pane focused) ",
        }
    };
    let border_style = if app.focused_pane == FocusedPane::Terminal {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

/// Renders the active terminal and its cursor when terminal focus is active.
fn render_active_terminal(app: &NexusDemo, frame: &mut Frame, area: ratatui::layout::Rect) {
    if let Some(entry) = app.active_session_terminal() {
        if is_terminal_copy_selection_active(&entry.copy_selection) {
            if let Some(snapshot) = entry.copy_selection.snapshot.as_ref() {
                render_screen_with_selection(
                    snapshot,
                    &entry.copy_selection,
                    area,
                    frame.buffer_mut(),
                );
            }
        } else if let Some(terminal) = entry.terminal.as_ref() {
            terminal.render(frame, area);
            render_terminal_cursor(app, terminal, frame, area);
        } else {
            frame.render_widget(Paragraph::new("Starting Nexus session…"), area);
        }
    } else {
        frame.render_widget(Paragraph::new("No Nexus sessions found"), area);
    }
}

/// Renders the terminal cursor when it belongs in the visible live screen.
fn render_terminal_cursor(
    app: &NexusDemo,
    terminal: &crate::nexus_terminal::NexusTerminal,
    frame: &mut Frame,
    area: ratatui::layout::Rect,
) {
    if app.focused_pane != FocusedPane::Terminal {
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
