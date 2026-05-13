//! Terminal pane identity and hit-testing helpers.

pub mod activate_terminal_pane;
pub mod close_button_area;
#[cfg(test)]
mod close_button_area_tests;
pub mod close_button_at_position;
pub mod fallback_terminal_pane_id;
pub mod id_at_position;
pub mod pane_id_for_session;
pub mod session_index_for_pane;
