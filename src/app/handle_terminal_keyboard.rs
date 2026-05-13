use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::nexus_demo_state::NexusDemo;
use crate::copy_mode::handle_terminal_copy_keyboard::handle_terminal_copy_keyboard;
use crate::expo::handle_expo_keyboard::handle_expo_keyboard;
use crate::main_pane::handle_file_system_tree_key::handle_file_system_tree_key;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::terminal::encode_key_event::encode_key_event;

/// Handles keyboard input while the main pane is focused.
pub fn handle_terminal_keyboard(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    match app.active_main_pane_tab {
        MainPaneTab::Files => {
            return Ok(handle_file_system_tree_key(
                &mut app.file_system_tree_view,
                &keyboard,
            ));
        }
        MainPaneTab::Expo => return Ok(handle_expo_keyboard(app, &keyboard)),
        MainPaneTab::Diff => return Ok(CoordinatorAction::Continue),
        MainPaneTab::Chat => {}
    }
    if let Some(action) = handle_terminal_copy_keyboard(app, &keyboard) {
        return Ok(action);
    }
    if let Some(bytes) = encode_key_event(&keyboard) {
        if let Some(terminal) = app.active_terminal() {
            terminal.reset_scrollback();
            terminal.write_input(&bytes);
        }
    }
    Ok(CoordinatorAction::Redraw)
}
