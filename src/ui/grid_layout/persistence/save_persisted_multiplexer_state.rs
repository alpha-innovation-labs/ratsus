use std::io;

use crate::shared::async_persistence::save_job::SaveJob;
use crate::shared::async_persistence::worker::enqueue_save;
use crate::ui::grid_layout::persistence::multiplexer_state_path::multiplexer_state_path;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;

/// Saves split-pane multiplexer state to the app-owned Nexus data path asynchronously.
pub fn save_persisted_multiplexer_state(state: &PersistedMultiplexerState) -> io::Result<()> {
    let Some(path) = multiplexer_state_path() else {
        return Ok(());
    };
    let content = serde_json::to_string_pretty(state)?;
    enqueue_save(SaveJob {
        key: "multiplexer-state",
        path,
        content,
    });
    Ok(())
}

/// Saves split-pane multiplexer state to the app-owned Nexus data path before returning.
pub fn save_persisted_multiplexer_state_now(state: &PersistedMultiplexerState) -> io::Result<()> {
    let Some(path) = multiplexer_state_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(state)?;
    std::fs::write(path, content)
}
