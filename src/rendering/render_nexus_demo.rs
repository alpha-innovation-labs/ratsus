use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use ratkit::primitives::resizable_grid::ResizableGridWidget;
use ratkit::primitives::toast::render_toasts;

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::render_conversation_picker_dialog::render_conversation_picker_dialog;
use crate::copy_mode::is_terminal_copy_selection_active::is_terminal_copy_selection_active;
use crate::copy_mode::render_screen_with_selection::render_screen_with_selection;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::is_resizing_layout::is_resizing_layout;
use crate::layout::pane_area_by_id::pane_area_by_id;
use crate::layout::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::left_panel::left_panel_hotkey_footer::left_panel_hotkey_footer;
use crate::left_panel::session_lines::session_lines;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::main_pane::main_pane_title_line::main_pane_title_line;
use crate::main_pane::render_file_system_tree_view::render_file_system_tree_view;
use crate::rendering::apply_cursor_style::apply_cursor_style;
use crate::rendering::render_resize_placeholder::render_resize_placeholder;

/// Renders the full terminal demo, including session list and active session terminal.
pub fn render_nexus_demo(app: &mut NexusDemo, frame: &mut Frame) {
    let area = frame.area();
    app.last_layout_area = area;
    let (left_pane, terminal_pane) = visible_pane_areas(app, area);
    app.last_left_area = left_pane;

    let is_resizing = is_resizing_layout(&app.layout_widget_state);
    if app.left_pane_visible {
        let left_inner = render_left_pane(app, frame, left_pane);
        let (session_list_area, footer_area) = split_left_pane_content(left_inner);
        app.last_session_list_area = session_list_area;
        app.keep_focused_row_visible();
        if is_resizing {
            render_resize_placeholder(session_list_area, frame.buffer_mut());
        } else {
            frame.render_widget(Paragraph::new(session_lines(app)), session_list_area);
        }
        frame.render_widget(left_panel_hotkey_footer(), footer_area);
    } else {
        app.last_session_list_area = Rect::default();
    }

    app.last_main_pane_area = terminal_pane;
    let terminal_inner = render_terminal_pane(app, frame, terminal_pane);
    if app.active_main_pane_tab == MainPaneTab::Chat {
        app.resize_active_terminal(terminal_inner);
    }
    if is_resizing {
        render_resize_placeholder(terminal_inner, frame.buffer_mut());
    } else {
        match app.active_main_pane_tab {
            MainPaneTab::Chat => render_active_terminal(app, frame, terminal_inner),
            MainPaneTab::Files => {
                render_file_system_tree_view(&mut app.file_system_tree_view, frame, terminal_inner)
            }
            MainPaneTab::Diff => {}
        }
    }
    if app.left_pane_visible {
        render_resizable_grid_overlay(app, frame, area);
    }
    app.toast_manager.remove_expired();
    render_toasts(frame, &app.toast_manager);
    render_conversation_picker_dialog(app, frame);
}

/// Splits the left pane into session-list and hotkey-footer areas.
fn split_left_pane_content(area: Rect) -> (Rect, Rect) {
    if area.height <= 1 {
        return (area, Rect::default());
    }
    let list_height = area.height.saturating_sub(1);
    let list_area = Rect::new(area.x, area.y, area.width, list_height);
    let footer_area = Rect::new(area.x, area.y + list_height, area.width, 1);
    (list_area, footer_area)
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
    let border_style = if app.focused_pane == FocusedPane::Terminal {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let block = Block::default()
        .title(main_pane_title_line(app.active_main_pane_tab))
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
    terminal: &crate::terminal::nexus_terminal::NexusTerminal,
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
