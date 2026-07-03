use std::path::Path;

use anyhow::ensure;
use ratsus::app::state::app_state::AppState;

/// Verifies each scenario used the real Nexus backend and real filesystem boundaries.
pub fn verify_real_case(
    app: &AppState,
    _domain: &str,
    _name: &str,
    workspace: impl AsRef<Path>,
) -> anyhow::Result<()> {
    ensure!(
        app.chat_harness.display_name() == "Nexus",
        "E2E tests must run against the real Nexus harness"
    );
    ensure!(
        !app.session_terminals.is_empty(),
        "real Nexus catalog must not be empty"
    );
    ensure!(
        workspace.as_ref().exists(),
        "test-owned workspace must exist while the scenario runs"
    );
    Ok(())
}
