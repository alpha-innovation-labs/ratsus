use std::io;

use crate::shared::async_persistence::save_job::SaveJob;
use crate::shared::async_persistence::worker::enqueue_save;
use crate::ui::grid_layout::persistence::multiplexer_state_path::multiplexer_state_path;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;

/// Saves split-pane multiplexer state to the app-owned Nexus data path.
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
