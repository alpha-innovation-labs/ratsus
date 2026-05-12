use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::{CoordinatorAction, CoordinatorEvent, KeyboardEvent, ResizeEvent};

use crate::encode_key_event::encode_key_event;
use crate::focused_pane::FocusedPane;
use crate::handle_resizable_grid_mouse::handle_resizable_grid_mouse;
use crate::handle_session_drag_mouse::handle_session_drag_mouse;
use crate::handle_terminal_copy_keyboard::handle_terminal_copy_keyboard;
use crate::handle_terminal_copy_mouse::handle_terminal_copy_mouse;
use crate::is_resizing_layout::is_resizing_layout;
use crate::nexus_demo_state::NexusDemo;
use crate::scroll_delta_for_mouse_kind::{scroll_delta_for_mouse_kind, TerminalScrollAction};
use crate::session_list_row::SessionListRow;
use crate::session_row_for_click::session_row_for_click;
use crate::show_failed_to_start_new_chat_toast::show_failed_to_start_new_chat_toast;
use crate::start_new_nexus_chat::start_new_nexus_chat;
use crate::toggle_left_pane_visibility::toggle_left_pane_visibility;
use crate::visible_session_rows::visible_session_rows;

const MOUSE_SCROLL_LINES_PER_TICK: usize = 3;

/// Handles one coordinator event for the terminal demo.
pub fn handle_nexus_demo_event(
    app: &mut NexusDemo,
    event: CoordinatorEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    match event {
        CoordinatorEvent::Resize(ResizeEvent { .. }) => Ok(CoordinatorAction::Redraw),
        CoordinatorEvent::Keyboard(keyboard) => handle_keyboard_event(app, keyboard),
        CoordinatorEvent::Mouse(mouse) => {
            if handle_resizable_grid_mouse(app, mouse) {
                return Ok(CoordinatorAction::Redraw);
            }
            if app.session_drag.is_some() && handle_session_drag_mouse(app, mouse) {
                return Ok(CoordinatorAction::Redraw);
            }
            if mouse.is_inside(app.last_left_area) {
                return Ok(handle_left_mouse(app, mouse));
            }
            if mouse.is_inside(app.last_terminal_area) {
                return Ok(handle_terminal_mouse(app, mouse));
            }
            Ok(CoordinatorAction::Continue)
        }
        CoordinatorEvent::Tick(_) => Ok(handle_tick_event(app)),
        _ => Ok(CoordinatorAction::Continue),
    }
}

/// Handles keyboard input for the focused pane.
fn handle_keyboard_event(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    if !keyboard.is_key_down() {
        return Ok(CoordinatorAction::Continue);
    }
    if keyboard.is_char('n') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        if let Err(error) = start_new_nexus_chat(app) {
            show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        }
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('l') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        toggle_left_pane_visibility(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('x') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        toggle_focused_pane(app);
        return Ok(CoordinatorAction::Redraw);
    }
    if keyboard.is_char('q') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        return Ok(CoordinatorAction::Quit);
    }
    match app.focused_pane {
        FocusedPane::Left => handle_left_keyboard(app, keyboard),
        FocusedPane::Terminal => handle_terminal_keyboard(app, keyboard),
    }
}

/// Toggles focus between the session list and terminal pane.
fn toggle_focused_pane(app: &mut NexusDemo) {
    if !app.left_pane_visible {
        app.focused_pane = FocusedPane::Terminal;
        return;
    }
    app.focused_pane = match app.focused_pane {
        FocusedPane::Terminal => FocusedPane::Left,
        FocusedPane::Left => FocusedPane::Terminal,
    };
}

/// Handles keyboard input while the left session pane is focused.
fn handle_left_keyboard(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    match keyboard.key_code {
        KeyCode::Char('j') | KeyCode::Down => app.select_relative_session(1),
        KeyCode::Char('k') | KeyCode::Up => app.select_relative_session(-1),
        KeyCode::Enter => app.activate_focused_session(),
        KeyCode::Char('q') if keyboard.modifiers.is_empty() => return Ok(CoordinatorAction::Quit),
        _ => return Ok(CoordinatorAction::Continue),
    }
    Ok(CoordinatorAction::Redraw)
}

/// Handles keyboard input while the terminal pane is focused.
fn handle_terminal_keyboard(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    if let Some(action) = handle_terminal_copy_keyboard(app, &keyboard) {
        return Ok(action);
    }
    if let Some(bytes) = encode_key_event(&keyboard) {
        if let Some(terminal) = app.active_terminal() {
            terminal.reset_scrollback();
            terminal.write_input(&bytes);
        }
    }
    Ok(CoordinatorAction::Redraw)
}

/// Handles mouse input routed to the left session pane.
fn handle_left_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    app.focused_pane = FocusedPane::Left;
    if handle_session_drag_mouse(app, mouse) {
        return CoordinatorAction::Redraw;
    }
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(_) => app.select_relative_session(-1),
        TerminalScrollAction::Down(_) => app.select_relative_session(1),
        TerminalScrollAction::None => handle_left_click(app, mouse),
    }
    CoordinatorAction::Redraw
}

/// Handles a clickable session row in the left pane.
fn handle_left_click(app: &mut NexusDemo, mouse: ratkit::MouseEvent) {
    if !mouse.is_click() {
        return;
    }
    let rows = visible_session_rows(&app.session_terminals, &app.collapsed_folders);
    let Some(row) = session_row_for_click(
        mouse.row,
        app.last_session_list_area,
        app.session_scroll,
        &rows,
    ) else {
        return;
    };
    match row {
        SessionListRow::Folder { path, .. } => app.toggle_folder(path),
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
        }
    }
}

/// Handles mouse input routed to the terminal pane.
fn handle_terminal_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    app.focused_pane = FocusedPane::Terminal;
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

/// Converts pending terminal redraw flags into a coordinator action.
fn handle_tick_event(app: &NexusDemo) -> CoordinatorAction {
    if is_resizing_layout(&app.layout_widget_state) {
        return CoordinatorAction::Continue;
    }

    let needs_redraw = app
        .session_terminals
        .iter()
        .any(|entry| entry.terminal.take_needs_redraw());
    if needs_redraw {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}
