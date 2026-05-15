use std::sync::Arc;
use std::time::Duration;

use anyhow::ensure;
use ratsus::app::sessions::drain_initial_sessions_receiver::drain_initial_sessions_receiver;
use ratsus::app::state::app_state::AppState;
use ratsus::extensions::harness::nexus::NexusHarness;

/// Builds a complete app state backed by the real Nexus harness.
pub fn create_real_app() -> anyhow::Result<AppState> {
    let mut app = AppState::new_with_harness(Arc::new(NexusHarness))?;
    wait_for_initial_sessions(&mut app)?;
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

/// Waits for the asynchronous startup session worker used by production startup.
fn wait_for_initial_sessions(app: &mut AppState) -> anyhow::Result<()> {
    for _ in 0..100 {
        if drain_initial_sessions_receiver(app)? || app.initial_sessions_receiver.is_none() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}
