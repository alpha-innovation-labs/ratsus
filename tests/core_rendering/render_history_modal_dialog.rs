use insta::assert_snapshot;

/// Validates the documented core_rendering E2E scenario.
#[test]
fn render_history_modal_dialog() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "core_rendering",
        "render_history_modal_dialog",
        "Picker modal overlays the app without damaging the underlying screen.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: core_rendering
test: render_history_modal_dialog
description: Picker modal overlays the app without damaging the underlying screen.
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
