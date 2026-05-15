use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;
use ratkit::primitives::toast::render_toasts;

use crate::app::diagnostics::app_diagnostics_status_line::app_diagnostics_status_line;
use crate::app::diagnostics::record_app_redraw::record_app_redraw;
use crate::app::state::app_state::AppState;
use crate::core::rendering::dialog::render_delete_session_confirmation_dialog::render_delete_session_confirmation_dialog;
use crate::core::rendering::resize::render_resizable_grid_overlay::render_resizable_grid_overlay;
use crate::core::rendering::resize::render_resize_placeholder::render_resize_placeholder;
use crate::core::rendering::screen::split_left_pane_content::split_left_pane_content;
use crate::core::rendering::style::default_border_color::default_border_color;
use crate::core::rendering::style::left_focused_border_color::left_focused_border_color;
use crate::extensions::command_bar::render::render_dialog::render_command_bar_dialog;
use crate::extensions::expo::filter::conversation_count::expo_conversation_count;
use crate::extensions::expo::render::render_view::render_expo_view;
use crate::extensions::file_viewer::preview::render_preview::render_file_preview;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tabs::title_line::main_pane_title_line;
use crate::extensions::harness::conversation_picker::render::render_dialog::render_conversation_picker_dialog;
use crate::extensions::plans::preview::render_plan_preview::render_plan_preview;
use crate::ui::grid_layout::render::render_chat_sessions::render_chat_sessions;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::resizable_grid::is_resizing::is_resizing_layout;
use crate::ui::layout::resizable_grid::pane_area_by_id::pane_area_by_id;
use crate::ui::layout::resizable_grid::pane_ids::{
    LEFT_PANE_ID, TERMINAL_PANE_ID, WORKSPACE_PANE_ID,
};
use crate::ui::left_panel::active_content::ActiveLeftPaneContent;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::mode::title_line::left_pane_mode_title_line;
use crate::ui::left_panel::render::hotkey_footer::left_panel_hotkey_footer;
use crate::ui::menu_bar::render::render_app_menu_bar::render_app_menu_bar;
use crate::ui::menu_bar::render::render_menu_bar_bottom_status::render_menu_bar_bottom_status;
use crate::ui::menu_bar::render::split_area::split_app_menu_bar_area;
use crate::ui::workspace_pane::render_workspace_pane::render_workspace_pane;

/// Renders the full terminal demo, including session list and active session terminal.
pub fn render_app(app: &mut AppState, frame: &mut Frame) {
    record_app_redraw(&mut app.diagnostics);
    let frame_area = frame.area();
    let (menu_area, area) = split_app_menu_bar_area(frame_area);
    render_app_menu_bar(
        &mut app.menu_bar,
        app.active_main_pane_tab,
        frame,
        menu_area,
    );
    render_menu_bar_bottom_status(
        frame,
        menu_area,
        &app_diagnostics_status_line(&app.diagnostics),
    );
    app.last_layout_area = area;
    let (workspace_pane, left_pane, terminal_pane) = visible_pane_areas(app, area);
    app.last_workspace_area = workspace_pane;
    app.last_left_area = left_pane;

    let is_resizing = is_resizing_layout(&app.layout_widget_state);
    if app.left_pane_visible {
        if app.workspace_view_enabled {
            render_workspace_pane(app, frame, workspace_pane, is_resizing);
        } else {
            app.last_workspace_list_area = Rect::default();
        }
        let left_focused = app.focused_pane == FocusedPane::Left;
        let title = left_pane_title_for_app(app);
        let left_inner = render_left_pane(left_focused, title, frame, left_pane);
        record_left_pane_toggle_areas(app, left_pane);
        let mut content = ActiveLeftPaneContent::for_app(app);
        render_active_left_pane_content(&mut content, frame, left_inner, is_resizing);
    } else {
        app.last_workspace_list_area = Rect::default();
        app.last_session_list_area = Rect::default();
        app.last_left_session_toggle_area = Rect::default();
        app.last_left_plan_toggle_area = Rect::default();
    }

    app.last_main_pane_area = terminal_pane;
    let terminal_inner = render_terminal_pane(app, frame, terminal_pane);
    app.last_terminal_area = terminal_inner;
    if is_resizing {
        render_resize_placeholder(terminal_inner, frame.buffer_mut());
    } else {
        match app.active_main_pane_tab {
            MainPaneTab::Chat if app.left_pane_mode == LeftPaneMode::Plans => {
                render_plan_preview(&mut app.plan_list, frame, terminal_inner)
            }
            MainPaneTab::Chat => render_chat_sessions(app, frame, terminal_inner),
            MainPaneTab::Files => {
                render_file_preview(&mut app.file_system_tree_view, frame, terminal_inner)
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
    render_command_bar_dialog(app, frame);
    render_delete_session_confirmation_dialog(app, frame);
}

/// Renders the shared left-pane shell body and footer for active content.
fn render_active_left_pane_content(
    content: &mut impl LeftPaneContent,
    frame: &mut Frame,
    area: Rect,
    is_resizing: bool,
) {
    let (body_area, footer_area) = split_left_pane_content(area);
    content.prepare_body_area(body_area);
    if is_resizing {
        render_resize_placeholder(body_area, frame.buffer_mut());
    } else {
        content.render_body(frame, body_area);
    }
    frame.render_widget(
        left_panel_hotkey_footer(content.footer_items(), content.footer_status()),
        footer_area,
    );
}

/// Builds the left-pane title for the currently hosted content.
fn left_pane_title_for_app(app: &AppState) -> Line<'static> {
    if app.active_main_pane_tab == MainPaneTab::Files {
        return Line::styled(" files ", Style::default().fg(Color::White));
    }
    left_pane_mode_title_line(app.left_pane_mode)
}

/// Records clickable top-bar toggle areas for mouse routing.
fn record_left_pane_toggle_areas(app: &mut AppState, area: Rect) {
    if app.active_main_pane_tab == MainPaneTab::Files {
        app.last_left_session_toggle_area = Rect::default();
        app.last_left_plan_toggle_area = Rect::default();
        return;
    }
    app.last_left_session_toggle_area = Rect::new(area.x.saturating_add(2), area.y, 10, 1);
    app.last_left_plan_toggle_area = Rect::new(area.x.saturating_add(13), area.y, 7, 1);
}

/// Returns pane areas for the current left-pane visibility state.
fn visible_pane_areas(app: &AppState, area: Rect) -> (Rect, Rect, Rect) {
    if !app.left_pane_visible {
        return (Rect::default(), Rect::default(), area);
    }
    let pane_layouts = app.layout.layout_panes(area);
    let workspace = pane_area_by_id(&pane_layouts, WORKSPACE_PANE_ID);
    let left = pane_area_by_id(&pane_layouts, LEFT_PANE_ID);
    let terminal = pane_area_by_id(&pane_layouts, TERMINAL_PANE_ID);
    if app.workspace_view_enabled {
        return (workspace, left, terminal);
    }
    (
        Rect::default(),
        combined_left_area(workspace, left),
        terminal,
    )
}

/// Combines workspace and session pane rectangles for legacy all-folders mode.
fn combined_left_area(workspace: Rect, left: Rect) -> Rect {
    if workspace == Rect::default() {
        return left;
    }
    let x = workspace.x.min(left.x);
    let y = workspace.y.min(left.y);
    let right = workspace
        .x
        .saturating_add(workspace.width)
        .max(left.x.saturating_add(left.width));
    let bottom = workspace
        .y
        .saturating_add(workspace.height)
        .max(left.y.saturating_add(left.height));
    Rect::new(x, y, right.saturating_sub(x), bottom.saturating_sub(y))
}

/// Renders the rounded left session pane and returns its content area.
fn render_left_pane(
    is_focused: bool,
    title: Line<'static>,
    frame: &mut Frame,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let border_style = if is_focused {
        Style::default().fg(left_focused_border_color())
    } else {
        Style::default().fg(default_border_color())
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

/// Renders the rounded terminal pane and returns its content area.
fn render_terminal_pane(
    app: &AppState,
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
