use insta::assert_snapshot;

/// Validates the documented core_rendering E2E scenario.
#[test]
fn render_full_screen_baseline() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "core_rendering",
        "render_full_screen_baseline",
        "The full shell renders menu bar, left pane, main pane, borders, and footer consistently.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: core_rendering
test: render_full_screen_baseline
description: The full shell renders menu bar, left pane, main pane, borders, and footer consistently.
backend: nexus
workflow: real Nexus terminal rendering workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
