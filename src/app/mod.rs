//! Application orchestration for the Nexus TUI.

use ratatui::Frame;
use ratkit::{CoordinatorAction, CoordinatorApp, CoordinatorEvent};

pub mod adjust_index_after_removals;
pub mod clamp_session_index;
pub mod close_exited_sessions;
pub mod exited_session_indices;
pub mod handle_keyboard_event;
pub mod handle_left_keyboard;
pub mod handle_nexus_demo_event;
pub mod handle_nexus_demo_mouse;
pub mod handle_terminal_keyboard;
pub mod handle_tick_event;
pub mod next_chat_index;
pub mod nexus_demo_methods;
pub mod nexus_demo_state;
pub mod redraw_action;
pub mod remove_exited_sessions;
pub mod restore_focus_after_removals;
pub mod run_nexus_app;
pub mod start_new_normal_terminal;
pub mod toggle_focused_pane;

use crate::rendering::render_nexus_demo::render_nexus_demo;
use handle_nexus_demo_event::handle_nexus_demo_event;
use nexus_demo_state::NexusDemo;

impl CoordinatorApp for NexusDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        handle_nexus_demo_event(self, event)
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        render_nexus_demo(self, frame);
    }
}

pub use run_nexus_app::run_nexus_app;
