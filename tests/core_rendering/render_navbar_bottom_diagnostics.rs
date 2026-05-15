use std::sync::Arc;

use ratsus::app::state::app_state::AppState;
use ratsus::extensions::harness::stub::StubHarness;

use crate::support::render_app_text::render_app_text;

/// Validates that the app renders live diagnostics on the navbar bottom row.
#[test]
fn render_navbar_bottom_diagnostics() -> anyhow::Result<()> {
    let mut app = AppState::new_with_harness(Arc::new(StubHarness::new()))?;

    let rendered = render_app_text(&mut app, 80, 20)?;
    let navbar_bottom_line = rendered.lines().nth(2).unwrap_or_default();

    assert!(navbar_bottom_line.contains("FPS 0 | Redraws 1"));
    assert!(!navbar_bottom_line.contains("Mouse"));
    Ok(())
}
