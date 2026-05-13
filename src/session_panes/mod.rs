//! Split terminal pane state and routing helpers.

pub mod activate_terminal_pane;
pub mod active_terminal_bundle_insert_index;
pub mod active_terminal_spawn_area;
pub mod bundle_new_session_in_active_terminal_pane;
pub mod close_exited_terminal_panes;
pub mod close_terminal_pane;
pub mod ensure_active_terminal_pane_session;
pub mod fallback_terminal_pane_id;
pub mod pane_id_for_session;
pub mod place_existing_session_in_active_terminal_pane;
pub mod prune_terminal_pane_session_bundles;
pub mod render_chat_sessions;
pub mod resize_terminal_pane_session;
pub mod session_bundle_marker;
pub mod session_index_for_pane;
pub mod set_active_terminal_pane_bundle_session;
pub mod set_active_terminal_pane_session;
pub mod split_active_terminal_pane;
pub mod terminal_bundle_for_keyboard;
pub mod terminal_pane_close_button_area;
#[cfg(test)]
mod terminal_pane_close_button_area_tests;
pub mod terminal_pane_close_button_at_position;
pub mod terminal_pane_id_at_position;
pub mod terminal_pane_session_ids;
pub mod terminal_split_direction;
pub mod terminal_split_direction_for_keyboard;
