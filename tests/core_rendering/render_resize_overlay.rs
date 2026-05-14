use insta::assert_snapshot;

/// Validates the documented core_rendering E2E scenario.
#[test]
fn render_resize_overlay() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "core_rendering",
        "render_resize_overlay",
        "Active resizing displays the resize overlay and returns to normal rendering after release.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: core_rendering
test: render_resize_overlay
description: Active resizing displays the resize overlay and returns to normal rendering after release.
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
