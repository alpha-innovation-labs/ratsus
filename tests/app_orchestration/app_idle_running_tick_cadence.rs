use std::sync::Arc;

use insta::assert_snapshot;
use ratkit::CoordinatorAction;
use ratsus::app::events::handle_tick_event::handle_tick_event;
use ratsus::app::state::app_state::AppState;
use ratsus::extensions::harness::stub::StubHarness;

/// Recreates the idle CPU regression by driving ticks while stub sessions are marked running.
#[test]
fn app_idle_running_tick_cadence() -> anyhow::Result<()> {
    let mut app = AppState::new_with_harness(Arc::new(StubHarness::new()))?;
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

/// Returns the stable label for a coordinator action in the cadence snapshot.
fn action_name(action: CoordinatorAction) -> &'static str {
    match action {
        CoordinatorAction::Continue => "Continue",
        CoordinatorAction::Redraw => "Redraw",
        CoordinatorAction::Quit => "Quit",
    }
}
