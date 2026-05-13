//! Left-panel keyboard, mouse, selection, and drag handling.

pub mod activate_focused_row;
pub mod dispatch_left_pane_keyboard;
#[cfg(test)]
mod dispatch_left_pane_keyboard_tests;
pub mod handle_session_drag_mouse;
pub mod key_behavior;
pub mod session_drag_state;
pub mod session_index_for_click;
pub mod session_row_for_click;
pub mod session_row_for_rendered_click;
pub mod should_focus_for_mouse;
pub mod should_toggle_folder_on_drop;
pub mod toggle_focused_selection;
