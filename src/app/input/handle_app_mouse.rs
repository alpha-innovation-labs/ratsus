use ratkit::CoordinatorAction;

use crate::app::expo::activate_expo_folder::activate_expo_folder;
use crate::app::state::app_state::AppState;
use crate::extensions::expo::input::handle_mouse::handle_expo_mouse;
use crate::extensions::history_modal::actions::open_folder::open_folder_conversation_picker;
use crate::extensions::history_modal::input::handle_mouse::handle_conversation_picker_mouse;
use crate::extensions::plans::input::handle_plan_drag_mouse::handle_plan_drag_mouse;
use crate::extensions::plans::input::handle_plan_left_mouse::handle_plan_left_mouse;
use crate::extensions::plans::preview::handle_plan_preview_mouse::handle_plan_preview_mouse;
use crate::extensions::terminal::copy_mode::input::handle_mouse::handle_terminal_copy_mouse;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::resizable_grid::handle_mouse::handle_resizable_grid_mouse;
use crate::ui::left_panel::folder::click_hits_label::folder_click_hits_icon;
use crate::ui::left_panel::folder::toggle_session_folder::toggle_session_folder;
use crate::ui::left_panel::input::handle_session_drag_mouse::handle_session_drag_mouse;
use crate::ui::left_panel::input::session_row_for_rendered_click::session_row_for_rendered_click;
use crate::ui::left_panel::input::should_focus_for_mouse::should_focus_left_pane_for_mouse;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::render::rendered_rows::rendered_left_panel_rows;
use crate::ui::left_panel::scroll::scroll_view::scroll_left_panel_view;
use crate::ui::left_panel::session::activation::activate_split_group_child::activate_split_group_child;
use crate::ui::left_panel::session::activation::activate_split_group_parent::activate_split_group_parent;
use crate::ui::left_panel::session::list_row::SessionListRow;

use crate::extensions::file_viewer::preview::handle_mouse::handle_file_preview_mouse;
use crate::extensions::file_viewer::tabs::handle_click::handle_main_pane_tab_click;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::handle_mouse::handle_file_system_tree_mouse;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};
use crate::ui::grid_layout::pane::activate_terminal_pane::activate_terminal_pane;
use crate::ui::grid_layout::pane::close_button_at_position::terminal_pane_close_button_at_position;
use crate::ui::grid_layout::pane::id_at_position::terminal_pane_id_at_position;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::grid_layout::split::close_terminal_pane::close_terminal_pane;
use crate::ui::menu_bar::input::handle_mouse::handle_menu_bar_mouse;
use crate::ui::workspace_pane::handle_workspace_mouse::handle_workspace_mouse;

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
    if app.workspace_view_enabled && app.workspace_drag.is_some() {
        return handle_workspace_mouse(app, mouse);
    }
    if app.left_pane_mode == LeftPaneMode::Plans
        && app.plan_list.drag.is_some()
        && handle_plan_drag_mouse(&mut app.plan_list, mouse)
    {
        return CoordinatorAction::Redraw;
    }
    if (app.session_drag.is_some() || app.folder_drag.is_some())
        && handle_session_drag_mouse(app, mouse)
    {
        return CoordinatorAction::Redraw;
    }
    if app.workspace_view_enabled && mouse.is_inside(app.last_workspace_area) {
        return handle_workspace_mouse(app, mouse);
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
    if app.left_pane_mode == LeftPaneMode::Files {
        let action = handle_file_system_tree_mouse(&mut app.file_system_tree_view, mouse);
        if matches!(action, CoordinatorAction::Redraw) {
            persist_multiplexer_state(app);
        }
        return action;
    }
    if app.left_pane_mode == LeftPaneMode::Plans {
        let action = handle_plan_left_mouse(app, mouse);
        if matches!(action, CoordinatorAction::Redraw) {
            persist_multiplexer_state(app);
        }
        return action;
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
        SessionListRow::SplitGroup { group_id, .. } => activate_split_group_parent(app, group_id),
        SessionListRow::SplitGroupChild { pane_id, index, .. } => {
            activate_split_group_child(app, pane_id, index);
        }
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
            app.focused_pane = FocusedPane::Terminal;
        }
    }
}

/// Handles mouse input routed to the terminal pane.
fn handle_terminal_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    app.focused_pane = FocusedPane::Terminal;
    if app.active_main_pane_tab == MainPaneTab::Expo {
        return handle_expo_mouse(app, mouse);
    }
    if app.active_main_pane_tab == MainPaneTab::Chat && app.left_pane_mode == LeftPaneMode::Plans {
        return handle_plan_preview_mouse(&mut app.plan_list, mouse, app.last_terminal_area);
    }
    if app.active_main_pane_tab == MainPaneTab::Chat && app.left_pane_mode == LeftPaneMode::Files {
        return handle_file_preview_mouse(
            &mut app.file_system_tree_view,
            mouse,
            app.last_terminal_area,
        );
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
