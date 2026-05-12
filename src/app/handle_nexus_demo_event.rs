use ratkit::{CoordinatorAction, CoordinatorEvent, ResizeEvent};

use crate::app::handle_keyboard_event::handle_keyboard_event;
use crate::app::handle_nexus_demo_mouse::handle_nexus_demo_mouse;
use crate::app::handle_tick_event::handle_tick_event;
use crate::app::nexus_demo_state::NexusDemo;

/// Handles one coordinator event for the terminal demo.
pub fn handle_nexus_demo_event(
    app: &mut NexusDemo,
    event: CoordinatorEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    match event {
        CoordinatorEvent::Resize(ResizeEvent { .. }) => Ok(CoordinatorAction::Redraw),
        CoordinatorEvent::Keyboard(keyboard) => handle_keyboard_event(app, keyboard),
        CoordinatorEvent::Mouse(mouse) => Ok(handle_nexus_demo_mouse(app, mouse)),
        CoordinatorEvent::Tick(tick_count) => Ok(handle_tick_event(app, tick_count)),
        _ => Ok(CoordinatorAction::Continue),
    }
}
