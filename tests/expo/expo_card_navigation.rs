use insta::assert_snapshot;

/// Validates the documented expo E2E scenario.
#[test]
fn expo_card_navigation() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "expo",
        "expo_card_navigation",
        "Keyboard navigation keeps the focused card visible.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: expo
test: expo_card_navigation
description: Keyboard navigation keeps the focused card visible.
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
