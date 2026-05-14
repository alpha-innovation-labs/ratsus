use insta::assert_snapshot;

/// Validates the documented core_rendering E2E scenario.
#[test]
fn render_small_terminal_layout() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "core_rendering",
        "render_small_terminal_layout",
        "Small terminal sizes clip content safely without panics or malformed borders.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: core_rendering
test: render_small_terminal_layout
description: Small terminal sizes clip content safely without panics or malformed borders.
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
