use crossterm::event::{KeyModifiers, MouseButton, MouseEventKind};
use ratkit::CoordinatorAction;

use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::actions::activate_selected::activate_selected_conversation;
use crate::extensions::history_modal::data::item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::extensions::history_modal::data::items::conversation_picker_items;
use crate::extensions::history_modal::layout::dialog_body_area::conversation_picker_dialog_body_area;
use crate::extensions::history_modal::layout::frame_area::conversation_picker_frame_area;
use crate::extensions::history_modal::layout::item_position_at_row::conversation_picker_item_position_at_row;
use crate::extensions::history_modal::selection::focus_session::focus_conversation_picker_session;
use crate::extensions::history_modal::selection::move_selection::move_conversation_picker_selection;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};
use crate::ui::left_panel::input::session_drag_state::SessionDragState;

const PICKER_SCROLL_LINES_PER_TICK: usize = 3;

/// Handles mouse wheel scrolling and drag reordering while the conversation picker is open.
pub fn handle_conversation_picker_mouse(
    app: &mut AppState,
    mouse: ratkit::MouseEvent,
) -> CoordinatorAction {
    if handle_conversation_picker_drag_mouse(app, mouse) {
        return CoordinatorAction::Redraw;
    }
    let direction = match scroll_delta_for_mouse_kind(mouse.kind, PICKER_SCROLL_LINES_PER_TICK) {
        TerminalScrollAction::Up(rows) => -(rows as isize),
        TerminalScrollAction::Down(rows) => rows as isize,
        TerminalScrollAction::None => return CoordinatorAction::Continue,
    };
    let item_count = current_picker_items(app).len();
    move_conversation_picker_selection(&mut app.conversation_picker, direction, item_count);
    CoordinatorAction::Redraw
}

/// Handles picker mouse drag events that reorder conversation rows.
fn handle_conversation_picker_drag_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> bool {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => start_picker_mouse(app, mouse),
        MouseEventKind::Drag(MouseButton::Left) => move_picker_drag(app, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => finish_picker_mouse(app),
        _ => false,
    }
}

/// Handles picker mouse down by opening normal clicks or starting Shift-drag reorder.
fn start_picker_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> bool {
    let Some((position, item)) = picker_item_at_row(app, mouse.row) else {
        return false;
    };
    app.conversation_picker.selected_position = position;
    if !mouse.modifiers.contains(KeyModifiers::SHIFT) {
        clear_picker_mouse_gesture(app);
        activate_selected_conversation(app);
        return true;
    }
    start_picker_drag(app, position, item)
}

/// Starts a picker drag and focuses the row under the mouse.
fn start_picker_drag(app: &mut AppState, position: usize, item: ConversationPickerItem) -> bool {
    app.conversation_picker.mouse_down_position = Some(position);
    app.conversation_picker.mouse_drag_moved = false;
    let ConversationPickerItemKind::Session { index, .. } = item.kind else {
        return true;
    };
    app.session_drag = Some(SessionDragState::new(index));
    app.focused_index = index;
    true
}

/// Moves the active picker drag to the session row under the mouse.
fn move_picker_drag(app: &mut AppState, row: u16) -> bool {
    if app.session_drag.is_none() {
        if app.conversation_picker.mouse_down_position.is_some() {
            app.conversation_picker.mouse_drag_moved = true;
            return true;
        }
        return false;
    }
    let Some((_, item)) = picker_item_at_row(app, row) else {
        return true;
    };
    let ConversationPickerItemKind::Session { index, .. } = item.kind else {
        return true;
    };
    let Some(session_id) = dragged_session_id(app) else {
        return true;
    };
    let Some(current_index) = app.session_drag.map(|drag| drag.current_index) else {
        return true;
    };
    if current_index != index {
        app.conversation_picker.mouse_drag_moved = true;
    }
    app.move_dragged_session(index);
    focus_conversation_picker_session(app, &session_id);
    true
}

/// Finishes a picker mouse gesture by opening clicks or ending drag reorders.
fn finish_picker_mouse(app: &mut AppState) -> bool {
    if app.conversation_picker.mouse_down_position.is_none() && app.session_drag.is_none() {
        return false;
    }
    let should_open = app.conversation_picker.mouse_down_position.is_some()
        && !app.conversation_picker.mouse_drag_moved;
    app.finish_left_panel_drag();
    clear_picker_mouse_gesture(app);
    if should_open {
        activate_selected_conversation(app);
    }
    true
}

/// Clears picker-local mouse gesture tracking.
fn clear_picker_mouse_gesture(app: &mut AppState) {
    app.conversation_picker.mouse_down_position = None;
    app.conversation_picker.mouse_drag_moved = false;
}

/// Returns the currently dragged session id.
fn dragged_session_id(app: &AppState) -> Option<String> {
    let index = app.session_drag?.current_index;
    app.session_terminals
        .get(index)
        .map(|entry| entry.session.id.clone())
}

/// Resolves a mouse row to a picker item snapshot and visible position.
fn picker_item_at_row(app: &AppState, row: u16) -> Option<(usize, ConversationPickerItem)> {
    let items = current_picker_items(app);
    let body_area = conversation_picker_dialog_body_area(conversation_picker_frame_area(app));
    let position = conversation_picker_item_position_at_row(
        body_area,
        row,
        app.conversation_picker.selected_position,
        &items,
    )?;
    items.get(position).cloned().map(|item| (position, item))
}

/// Builds the current picker item snapshot.
fn current_picker_items(app: &AppState) -> Vec<ConversationPickerItem> {
    conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
}
