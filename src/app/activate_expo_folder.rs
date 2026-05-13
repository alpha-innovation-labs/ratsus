use std::path::PathBuf;

use crate::app::app_state::AppState;
use crate::expo::start_observation_cache_load::start_observation_cache_load;
use crate::main_pane::main_pane_tab::MainPaneTab;

/// Activates the Expo extension for the selected folder.
pub fn activate_expo_folder(app: &mut AppState, folder: PathBuf) {
    let folder_changed = app.selected_expo_folder.as_ref() != Some(&folder);
    if folder_changed {
        app.expo_scroll = 0;
        app.expo_filter_query.clear();
        app.expo_filtering = false;
    }
    app.selected_expo_folder = Some(folder);
    app.active_main_pane_tab = MainPaneTab::Expo;
    if folder_changed {
        start_observation_cache_load(app);
    }
}
