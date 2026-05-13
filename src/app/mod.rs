//! Application orchestration for the harness-backed TUI.

use ratatui::Frame;
use ratkit::{CoordinatorAction, CoordinatorApp, CoordinatorEvent};

pub mod activate_expo_folder;
pub mod adjacent_chat_index_in_left_pane_order;
pub mod adjust_index_after_removals;
pub mod app_state;
pub mod app_state_methods;
pub mod chat_cycle_direction_for_keyboard;
pub mod clamp_session_index;
pub mod close_exited_sessions;
pub mod complete_delete_sessions;
pub mod confirm_delete_session;
pub mod cycle_chat_in_left_pane_order;
pub mod delete_focus_row_index;
pub mod delete_session_confirmation_message;
pub mod delete_session_confirmation_state;
pub mod delete_session_title;
pub mod delete_sessions_result;
pub mod drain_delete_session_receiver;
pub mod exited_session_indices;
pub mod exited_session_pane_ids;
pub mod handle_app_event;
pub mod handle_app_mouse;
pub mod handle_delete_session_confirmation_keyboard;
pub mod handle_keyboard_event;
pub mod handle_left_keyboard;
pub mod handle_terminal_keyboard;
pub mod handle_tick_event;
pub mod new_app_state;
pub mod next_chat_index;
pub mod normal_terminal_insert_index;
pub mod normal_terminal_working_dir;
pub mod open_delete_session_confirmation;
pub mod open_expo_for_focused_conversation;
pub mod ordered_chat_indices_from_left_rows;
pub mod redraw_action;
pub mod removable_delete_session_ids;
pub mod remove_exited_sessions;
pub mod reorder_session_to_index;
pub mod restore_focus_after_bulk_delete;
pub mod restore_focus_after_delete;
pub mod restore_focus_after_removals;
pub mod run_app;
pub mod selected_delete_targets;
pub mod session_index_after_delete;
pub mod spawn_delete_sessions_worker;
pub mod start_new_normal_terminal;
pub mod toggle_conversation_selection_by_index;
pub mod toggle_focused_pane;
pub mod visible_rows_cache_methods;

use crate::rendering::render_app::render_app;
use app_state::AppState;
use handle_app_event::handle_app_event;

impl CoordinatorApp for AppState {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        handle_app_event(self, event)
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        render_app(self, frame);
    }
}

pub use run_app::run_app;
