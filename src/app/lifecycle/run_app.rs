use std::sync::Arc;

use anyhow::Result;
use ratkit::{run, RunnerConfig};

use crate::app::state::app_state::AppState;
use crate::extensions::harness::core::chat_harness::ChatHarness;

/// Starts the TUI application with an injected chat harness.
pub fn run_app(chat_harness: Arc<dyn ChatHarness>) -> Result<()> {
    let app = AppState::new_with_harness(chat_harness)?;
    run(app, RunnerConfig::default())?;
    Ok(())
}
