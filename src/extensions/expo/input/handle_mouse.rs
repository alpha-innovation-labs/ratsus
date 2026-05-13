use ratkit::CoordinatorAction;

use crate::app::state::app_state::AppState;
use crate::extensions::expo::card::session_index_at_position::expo_card_session_index_at_position;
use crate::extensions::expo::input::scroll_view::scroll_expo_view;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};

/// Activates the conversation represented by a clicked Expo card.
pub fn handle_expo_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => {
            if scroll_expo_view(app, -(rows as isize)) {
                return CoordinatorAction::Redraw;
            }
            return CoordinatorAction::Continue;
        }
        TerminalScrollAction::Down(rows) => {
            if scroll_expo_view(app, rows as isize) {
                return CoordinatorAction::Redraw;
            }
            return CoordinatorAction::Continue;
        }
        TerminalScrollAction::None => {}
    }
    if !mouse.is_click() {
        return CoordinatorAction::Continue;
    }
    let Some(session_index) =
        expo_card_session_index_at_position(&app.expo_card_areas, mouse.column, mouse.row)
    else {
        return CoordinatorAction::Continue;
    };
    app.focused_index = session_index;
    app.activate_focused_session();
    CoordinatorAction::Redraw
}
