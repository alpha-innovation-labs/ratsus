use crossterm::event::KeyCode;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::open_conversation_picker::open_conversation_picker;
use crate::left_panel::activate_focused_left_row::activate_focused_left_row;
use crate::left_panel::collapse_focused_project::collapse_focused_project;
use crate::left_panel::focus_left_panel_end::focus_left_panel_end;
use crate::left_panel::focus_left_panel_start::focus_left_panel_start;
use crate::left_panel::open_focused_project::open_focused_project;

/// Handles keyboard input while the left session pane is focused.
pub fn handle_left_keyboard(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    let handled = match keyboard.key_code {
        KeyCode::Char('j') | KeyCode::Down => {
            handle_left_action(app, |app| app.select_relative_session(1))
        }
        KeyCode::Char('k') | KeyCode::Up => {
            handle_left_action(app, |app| app.select_relative_session(-1))
        }
        KeyCode::Char('h') | KeyCode::Left => handle_left_action(app, collapse_focused_project),
        KeyCode::Char('l') | KeyCode::Right => handle_left_action(app, open_focused_project),
        KeyCode::Char('/') if keyboard.modifiers.is_empty() => {
            handle_left_action(app, open_conversation_picker)
        }
        KeyCode::Char('g') if keyboard.modifiers.is_empty() => handle_left_g(app),
        KeyCode::Char('G') => handle_left_action(app, focus_left_panel_end),
        KeyCode::Enter => handle_left_action(app, activate_focused_left_row),
        KeyCode::Char('q') if keyboard.modifiers.is_empty() => return Ok(CoordinatorAction::Quit),
        _ => false,
    };
    if handled {
        return Ok(CoordinatorAction::Redraw);
    }
    app.pending_left_g = false;
    Ok(CoordinatorAction::Continue)
}

/// Runs a left-panel action and clears any pending multi-key sequence.
fn handle_left_action(app: &mut NexusDemo, action: impl FnOnce(&mut NexusDemo)) -> bool {
    app.pending_left_g = false;
    action(app);
    true
}

/// Handles the `gg` sequence for moving to the start of the left pane.
fn handle_left_g(app: &mut NexusDemo) -> bool {
    if app.pending_left_g {
        app.pending_left_g = false;
        focus_left_panel_start(app);
        return true;
    }
    app.pending_left_g = true;
    true
}
