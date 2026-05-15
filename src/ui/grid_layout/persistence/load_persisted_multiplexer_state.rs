use crate::ui::grid_layout::persistence::multiplexer_state_path::multiplexer_state_path;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;

/// Loads persisted split-pane multiplexer state from the Nexus data path.
pub fn load_persisted_multiplexer_state() -> Option<PersistedMultiplexerState> {
    let path = multiplexer_state_path()?;
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<PersistedMultiplexerState>(&content).ok()
}
