use ratkit::CoordinatorAction;

use crate::app::activate_expo_folder::activate_expo_folder;
use crate::app::app_state::AppState;
use crate::conversation_picker::handle_conversation_picker_mouse::handle_conversation_picker_mouse;
use crate::conversation_picker::open_folder_conversation_picker::open_folder_conversation_picker;
use crate::copy_mode::handle_terminal_copy_mouse::handle_terminal_copy_mouse;
use crate::expo::handle_expo_mouse::handle_expo_mouse;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::handle_resizable_grid_mouse::handle_resizable_grid_mouse;
use crate::left_panel::folder_click_hits_label::folder_click_hits_icon;
use crate::left_panel::handle_session_drag_mouse::handle_session_drag_mouse;
use crate::left_panel::rendered_left_panel_rows::rendered_left_panel_rows;
use crate::left_panel::scroll_left_panel_view::scroll_left_panel_view;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_row_for_rendered_click::session_row_for_rendered_click;
use crate::left_panel::should_focus_left_pane_for_mouse::should_focus_left_pane_for_mouse;
use crate::left_panel::toggle_session_folder::toggle_session_folder;

use crate::main_pane::handle_file_preview_mouse::handle_file_preview_mouse;
use crate::main_pane::handle_file_system_tree_mouse::handle_file_system_tree_mouse;
use crate::main_pane::handle_main_pane_tab_click::handle_main_pane_tab_click;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::handle_menu_bar_mouse::handle_menu_bar_mouse;
use crate::session_panes::activate_terminal_pane::activate_terminal_pane;
use crate::session_panes::close_terminal_pane::close_terminal_pane;
use crate::session_panes::terminal_pane_close_button_at_position::terminal_pane_close_button_at_position;
use crate::session_panes::terminal_pane_id_at_position::terminal_pane_id_at_position;
use crate::terminal::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};

const MOUSE_SCROLL_LINES_PER_TICK: usize = 3;

/// Handles one mouse event for the app.
pub fn handle_app_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    if app.conversation_picker.is_open {
        return handle_conversation_picker_mouse(app, mouse);
    }
    if app.delete_confirmation.is_open() {
        return CoordinatorAction::Continue;
    }
    if let Some(action) = handle_menu_bar_mouse(app, mouse) {
        return action;
    }
    if let Some(action) = handle_main_pane_tab_click(app, mouse) {
        return action;
    }
    if handle_resizable_grid_mouse(app, mouse) {
        return CoordinatorAction::Redraw;
    }
    if (app.session_drag.is_some() || app.folder_drag.is_some())
        && handle_session_drag_mouse(app, mouse)
    {
        return CoordinatorAction::Redraw;
    }
    if mouse.is_inside(app.last_left_area) {
        return handle_left_mouse(app, mouse);
    }
    if mouse.is_inside(app.last_terminal_area) {
        return handle_terminal_mouse(app, mouse);
    }
    CoordinatorAction::Continue
}

/// Handles mouse input routed to the left session pane.
fn handle_left_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    if should_focus_left_pane_for_mouse(mouse.kind) {
        app.focused_pane = FocusedPane::Left;
    }
    if app.active_main_pane_tab == MainPaneTab::Files {
        return handle_file_system_tree_mouse(&mut app.file_system_tree_view, mouse);
    }
    if handle_session_drag_mouse(app, mouse) {
        return CoordinatorAction::Redraw;
    }
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => {
            app.pending_left_scroll_redraw |= scroll_left_panel_view(app, -(rows as isize));
            return CoordinatorAction::Continue;
        }
        TerminalScrollAction::Down(rows) => {
            app.pending_left_scroll_redraw |= scroll_left_panel_view(app, rows as isize);
            return CoordinatorAction::Continue;
        }
        TerminalScrollAction::None => handle_left_click(app, mouse),
    }
    CoordinatorAction::Redraw
}

/// Handles a clickable session row in the left pane.
fn handle_left_click(app: &mut AppState, mouse: ratkit::MouseEvent) {
    if !mouse.is_click() {
        return;
    }
    let rows = rendered_left_panel_rows(app);
    let Some((source_row_index, row)) =
        session_row_for_rendered_click(mouse.row, app.last_session_list_area, &rows)
    else {
        return;
    };
    app.suppress_left_focus_scroll = false;
    app.focused_row = source_row_index;
    match row {
        SessionListRow::Folder { path, .. } => {
            activate_expo_folder(app, path.clone());
            if folder_click_hits_icon(mouse.column, app.last_session_list_area.x) {
                toggle_session_folder(app, path);
            }
        }
        SessionListRow::FolderMore { path } => open_folder_conversation_picker(app, path),
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
        }
    }
}

/// Handles mouse input routed to the terminal pane.
fn handle_terminal_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    app.focused_pane = FocusedPane::Terminal;
    if app.active_main_pane_tab == MainPaneTab::Expo {
        return handle_expo_mouse(app, mouse);
    }
    if app.active_main_pane_tab == MainPaneTab::Files {
        return handle_file_preview_mouse(
            &mut app.file_system_tree_view,
            mouse,
            app.last_terminal_area,
        );
    }
    if app.active_main_pane_tab != MainPaneTab::Chat {
        return CoordinatorAction::Continue;
    }
    if mouse.is_click() {
        if let Some(pane_id) = terminal_pane_close_button_at_position(app, mouse.column, mouse.row)
        {
            if close_terminal_pane(app, pane_id) {
                return CoordinatorAction::Redraw;
            }
        }
    }
    if let Some(pane_id) = terminal_pane_id_at_position(app, mouse.column, mouse.row) {
        activate_terminal_pane(app, pane_id);
    }
    if handle_terminal_copy_mouse(app, mouse, MOUSE_SCROLL_LINES_PER_TICK) {
        return CoordinatorAction::Redraw;
    }
    if let Some(terminal) = app.active_terminal() {
        match scroll_delta_for_mouse_kind(mouse.kind, MOUSE_SCROLL_LINES_PER_TICK) {
            TerminalScrollAction::Up(rows) => terminal.scrollback_up(rows),
            TerminalScrollAction::Down(rows) => terminal.scrollback_down(rows),
            TerminalScrollAction::None => {}
        }
    }
    CoordinatorAction::Redraw
}
