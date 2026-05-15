use std::sync::Arc;

use insta::assert_snapshot;
use ratkit::CoordinatorAction;
use ratsus::app::events::handle_tick_event::handle_tick_event;
use ratsus::app::sessions::drain_initial_sessions_receiver::drain_initial_sessions_receiver;
use ratsus::app::state::app_state::AppState;
use ratsus::extensions::harness::stub::StubHarness;

/// Recreates the idle CPU regression by driving ticks while stub sessions are marked running.
#[test]
fn app_idle_running_tick_cadence() -> anyhow::Result<()> {
    let mut app = AppState::new_with_harness(Arc::new(StubHarness::new()))?;
    wait_for_initial_sessions(&mut app)?;
    app.session_watcher = None;
    app.observation_cache_receiver = None;

    let actions = (1..=8)
        .map(|tick| action_name(handle_tick_event(&mut app, tick)))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(actions, @r###"
Continue
Continue
Continue
Redraw
Continue
Continue
Continue
Redraw
"###);
    Ok(())
}

/// Waits for startup sessions so cadence assertions measure only idle ticks.
fn wait_for_initial_sessions(app: &mut AppState) -> anyhow::Result<()> {
    for _ in 0..100 {
        if drain_initial_sessions_receiver(app)? || app.initial_sessions_receiver.is_none() {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}

/// Returns the stable label for a coordinator action in the cadence snapshot.
fn action_name(action: CoordinatorAction) -> &'static str {
    match action {
        CoordinatorAction::Continue => "Continue",
        CoordinatorAction::Redraw => "Redraw",
        CoordinatorAction::Quit => "Quit",
    }
}
