use std::path::Path;

use anyhow::Result;

use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::creation::new_chat_insert_index::new_chat_insert_index;
use crate::ui::grid_layout::split::active_spawn_area::active_terminal_spawn_area;
use crate::ui::grid_layout::split::set_active_session::set_active_terminal_pane_session;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::order::persist_preferences::persist_session_order_preferences;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Starts a fresh chat in a specific working directory and focuses it.
pub fn start_new_chat_in_dir(app: &mut AppState, working_dir: &Path) -> Result<()> {
    let area = active_terminal_spawn_area(app);
    let rows = area.height.max(1);
    let cols = area.width.max(1);
    let session_terminal = app.chat_harness.spawn_new_chat(working_dir, rows, cols)?;
    let session_id = session_terminal.session.id.clone();

    let new_index = new_chat_insert_index(&app.session_terminals, working_dir);
    app.session_terminals.insert(new_index, session_terminal);
    app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
    app.active_index = new_index;
    app.focused_index = new_index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_terminal_area = area;
    set_active_terminal_pane_session(app, session_id);
    app.keep_focused_session_visible();
    persist_session_order_preferences(app);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::start_new_chat_in_dir;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;

    /// Creating a new chat inside an existing folder preserves the left-panel folder order.
    #[test]
    fn preserves_folder_order_when_starting_chat_in_existing_folder() {
        let original_home = env::var_os("HOME");
        let isolated_home = env::temp_dir().join(format!(
            "ratsus-start-new-chat-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos()
        ));
        env::set_var("HOME", isolated_home);

        let mut app = app_fixture(vec![
            dormant_session("Alpha", "alpha", "/tmp/a"),
            dormant_session("Beta", "beta", "/tmp/b"),
        ])
        .expect("app fixture should build");
        app.folder_order = vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")];

        let result = start_new_chat_in_dir(&mut app, Path::new("/tmp/b"));

        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }
        result.expect("new chat should start");
        assert_eq!(
            app.folder_order,
            vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")]
        );
    }
}
