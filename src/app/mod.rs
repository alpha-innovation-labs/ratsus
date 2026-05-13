//! Application orchestration for the harness-backed TUI.

use ratatui::Frame;
use ratkit::{CoordinatorAction, CoordinatorApp, CoordinatorEvent};

pub mod deletion;
pub mod events;
pub mod expo;
pub mod focus;
pub mod input;
pub mod lifecycle;
pub mod navigation;
pub mod sessions;
pub mod state;
#[cfg(test)]
pub mod test_support;

use crate::app::events::handle_app_event::handle_app_event;
use crate::app::state::app_state::AppState;
use crate::core::rendering::screen::render_app::render_app;

impl CoordinatorApp for AppState {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        handle_app_event(self, event)
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        render_app(self, frame);
    }
}

pub use lifecycle::run_app::run_app;
