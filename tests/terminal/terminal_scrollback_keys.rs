use insta::assert_snapshot;

/// Validates the documented terminal E2E scenario.
#[test]
fn terminal_scrollback_keys() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "terminal",
        "terminal_scrollback_keys",
        "Scrollback commands keep terminal rendering valid.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: terminal
test: terminal_scrollback_keys
description: Scrollback commands keep terminal rendering valid.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
