use std::sync::Arc;

use anyhow::ensure;
use ratsus::app::state::app_state::AppState;
use ratsus::extensions::harness::nexus::NexusHarness;

/// Builds a complete app state backed by the real Nexus harness.
pub fn create_real_app() -> anyhow::Result<AppState> {
    let app = AppState::new_with_harness(Arc::new(NexusHarness))?;
    ensure!(
        app.chat_harness.display_name() == "Nexus",
        "E2E tests must use the real Nexus harness"
    );
    ensure!(
        !app.session_terminals.is_empty(),
        "real Nexus session catalog must contain at least one session"
    );
    Ok(app)
}
