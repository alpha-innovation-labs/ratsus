use std::sync::Arc;

use anyhow::Result;
use ratkit::{run, RunnerConfig};

use crate::app::state::app_state::AppState;
use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::ui::grid_layout::persistence::restore_multiplexer_state_into_app::restore_multiplexer_state_into_app;

/// Starts the TUI application with an injected chat harness.
pub fn run_app(chat_harness: Arc<dyn ChatHarness>) -> Result<()> {
    let mut app = AppState::new_with_harness(chat_harness)?;
    restore_multiplexer_state_into_app(&mut app);
    run(app, RunnerConfig::default())?;
    Ok(())
}
