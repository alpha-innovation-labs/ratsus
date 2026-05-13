use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use ratkit::primitives::toast::render_toasts;

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::render_conversation_picker_dialog::render_conversation_picker_dialog;
use crate::expo::expo_conversation_count::expo_conversation_count;
use crate::expo::render_expo_view::render_expo_view;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::is_resizing_layout::is_resizing_layout;
use crate::layout::pane_area_by_id::pane_area_by_id;
use crate::layout::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::left_panel::left_panel_hotkey_footer::left_panel_hotkey_footer;
use crate::left_panel::render_left_panel_scrollbar::render_left_panel_scrollbar;
use crate::left_panel::session_lines::session_lines;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::main_pane::main_pane_title_line::main_pane_title_line;
use crate::main_pane::render_file_system_tree_view::render_file_system_tree_view;
use crate::menu_bar::render_nexus_menu_bar::render_nexus_menu_bar;
use crate::menu_bar::split_nexus_menu_bar_area::split_nexus_menu_bar_area;
use crate::rendering::default_border_color::default_border_color;
use crate::rendering::left_focused_border_color::left_focused_border_color;
use crate::rendering::render_delete_session_confirmation_dialog::render_delete_session_confirmation_dialog;
use crate::rendering::render_resizable_grid_overlay::render_resizable_grid_overlay;
use crate::rendering::render_resize_placeholder::render_resize_placeholder;
use crate::rendering::split_left_pane_content::split_left_pane_content;
use crate::session_panes::render_chat_sessions::render_chat_sessions;

/// Renders the full terminal demo, including session list and active session terminal.
pub fn render_nexus_demo(app: &mut NexusDemo, frame: &mut Frame) {
    let frame_area = frame.area();
    let (menu_area, area) = split_nexus_menu_bar_area(frame_area);
    render_nexus_menu_bar(
        &mut app.menu_bar,
        app.active_main_pane_tab,
        frame,
        menu_area,
    );
    app.last_layout_area = area;
    let (left_pane, terminal_pane) = visible_pane_areas(app, area);
    app.last_left_area = left_pane;

    let is_resizing = is_resizing_layout(&app.layout_widget_state);
    if app.left_pane_visible {
        let left_inner = render_left_pane(app, frame, left_pane);
        let (session_list_area, footer_area) = split_left_pane_content(left_inner);
        app.last_session_list_area = session_list_area;
        if is_resizing {
            render_resize_placeholder(session_list_area, frame.buffer_mut());
        } else {
            frame.render_widget(Paragraph::new(session_lines(app)), session_list_area);
        }
        render_left_panel_scrollbar(app, frame);
        frame.render_widget(left_panel_hotkey_footer(), footer_area);
    } else {
        app.last_session_list_area = Rect::default();
    }

    app.last_main_pane_area = terminal_pane;
    let terminal_inner = render_terminal_pane(app, frame, terminal_pane);
    app.last_terminal_area = terminal_inner;
    if is_resizing {
        render_resize_placeholder(terminal_inner, frame.buffer_mut());
    } else {
        match app.active_main_pane_tab {
            MainPaneTab::Chat => render_chat_sessions(app, frame, terminal_inner),
            MainPaneTab::Files => {
                render_file_system_tree_view(&mut app.file_system_tree_view, frame, terminal_inner)
            }
            MainPaneTab::Diff => {}
            MainPaneTab::Expo => render_expo_view(app, frame, terminal_inner),
        }
    }
    if app.left_pane_visible {
        render_resizable_grid_overlay(app, frame, area);
    }
    app.toast_manager.remove_expired();
    render_toasts(frame, &app.toast_manager);
    render_conversation_picker_dialog(app, frame);
    render_delete_session_confirmation_dialog(app, frame);
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

/// Renders the rounded left session pane and returns its content area.
fn render_left_pane(
    app: &NexusDemo,
    frame: &mut Frame,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let border_style = if app.focused_pane == FocusedPane::Left {
        Style::default().fg(left_focused_border_color())
    } else {
        Style::default().fg(default_border_color())
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
    let border_style = Style::default().fg(default_border_color());
    let block = Block::default()
        .title(main_pane_title_line(
            app.active_main_pane_tab,
            expo_conversation_count(app),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}
