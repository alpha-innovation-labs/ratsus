use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::state::app_state::AppState;
use crate::extensions::expo::input::handle_keyboard::handle_expo_keyboard;
use crate::extensions::file_viewer::preview::handle_key::handle_file_preview_key;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::terminal::copy_mode::input::handle_keyboard::handle_terminal_copy_keyboard;
use crate::extensions::terminal::input::encode_key_event::encode_key_event;

/// Handles keyboard input while the main pane is focused.
pub fn handle_terminal_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    match app.active_main_pane_tab {
        MainPaneTab::Files => {
            return Ok(handle_file_preview_key(
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
