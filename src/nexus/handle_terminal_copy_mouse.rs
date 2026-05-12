use crossterm::event::{MouseButton, MouseEventKind};
use ratkit::MouseEvent;

use crate::copy_text_to_clipboard::copy_text_to_clipboard;
use crate::finish_terminal_copy_selection::finish_terminal_copy_selection;
use crate::nexus_demo_state::NexusDemo;
use crate::scroll_delta_for_mouse_kind::{scroll_delta_for_mouse_kind, TerminalScrollAction};
use crate::selection_position_for_mouse::selection_position_for_mouse;
use crate::show_copied_to_clipboard_toast::show_copied_to_clipboard_toast;
use crate::start_terminal_copy_selection::start_terminal_copy_selection;
use crate::update_terminal_copy_selection::update_terminal_copy_selection;

/// Handles mouse events that create or update mprocs-style terminal copy selection.
pub fn handle_terminal_copy_mouse(
    app: &mut NexusDemo,
    mouse: MouseEvent,
    scroll_lines_per_tick: usize,
) -> bool {
    let area = app.last_terminal_area;
    let Some(entry) = app.active_session_terminal_mut() else {
        return false;
    };

    if let Some(snapshot) = entry.copy_selection.snapshot.as_mut() {
        match scroll_delta_for_mouse_kind(mouse.kind, scroll_lines_per_tick) {
            TerminalScrollAction::Up(rows) => {
                snapshot.scroll_screen_up(rows);
                return true;
            }
            TerminalScrollAction::Down(rows) => {
                snapshot.scroll_screen_down(rows);
                return true;
            }
            TerminalScrollAction::None => {}
        }
    }

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let Some(screen) = entry.terminal.screen_snapshot() else {
                return false;
            };
            let Some(position) = selection_position_for_mouse(mouse, area, screen.scrollback())
            else {
                return false;
            };
            start_terminal_copy_selection(&mut entry.copy_selection, screen, position);
            true
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            let Some(snapshot) = entry.copy_selection.snapshot.as_ref() else {
                return false;
            };
            let Some(position) = selection_position_for_mouse(mouse, area, snapshot.scrollback())
            else {
                return false;
            };
            update_terminal_copy_selection(&mut entry.copy_selection, position);
            true
        }
        MouseEventKind::Up(MouseButton::Left) => {
            let copied_text = finish_terminal_copy_selection(&mut entry.copy_selection);
            if let Some(text) = copied_text {
                if !text.is_empty() {
                    copy_text_to_clipboard(&text);
                    show_copied_to_clipboard_toast(&mut app.toast_manager);
                }
            }
            true
        }
        _ => false,
    }
}
