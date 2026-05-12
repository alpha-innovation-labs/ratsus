use ratkit::CoordinatorAction;

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::open_folder_conversation_picker::open_folder_conversation_picker;
use crate::copy_mode::handle_terminal_copy_mouse::handle_terminal_copy_mouse;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::handle_resizable_grid_mouse::handle_resizable_grid_mouse;
use crate::left_panel::handle_session_drag_mouse::handle_session_drag_mouse;
use crate::left_panel::scroll_left_panel_view::scroll_left_panel_view;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_row_for_click::session_row_for_click;
use crate::left_panel::toggle_session_folder::toggle_session_folder;
use crate::main_pane::handle_main_pane_tab_click::handle_main_pane_tab_click;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::terminal::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};

const MOUSE_SCROLL_LINES_PER_TICK: usize = 3;

/// Handles one mouse event for the Nexus demo.
pub fn handle_nexus_demo_mouse(
    app: &mut NexusDemo,
    mouse: ratkit::MouseEvent,
) -> CoordinatorAction {
    if app.conversation_picker.is_open {
        return CoordinatorAction::Continue;
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
fn handle_left_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    app.focused_pane = FocusedPane::Left;
    if handle_session_drag_mouse(app, mouse) {
        return CoordinatorAction::Redraw;
    }
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => scroll_left_panel_view(app, -(rows as isize)),
        TerminalScrollAction::Down(rows) => scroll_left_panel_view(app, rows as isize),
        TerminalScrollAction::None => handle_left_click(app, mouse),
    }
    CoordinatorAction::Redraw
}

/// Handles a clickable session row in the left pane.
fn handle_left_click(app: &mut NexusDemo, mouse: ratkit::MouseEvent) {
    if !mouse.is_click() {
        return;
    }
    let rows = app.visible_rows();
    let Some(row) = session_row_for_click(
        mouse.row,
        app.last_session_list_area,
        app.session_scroll,
        &rows,
    ) else {
        return;
    };
    app.focused_row = app.session_scroll + usize::from(mouse.row - app.last_session_list_area.y);
    match row {
        SessionListRow::Folder { path, .. } => toggle_session_folder(app, path),
        SessionListRow::FolderMore { path } => open_folder_conversation_picker(app, path),
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
        }
    }
}

/// Handles mouse input routed to the terminal pane.
fn handle_terminal_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    app.focused_pane = FocusedPane::Terminal;
    if app.active_main_pane_tab != MainPaneTab::Chat {
        return CoordinatorAction::Continue;
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
