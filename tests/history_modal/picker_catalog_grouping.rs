use insta::assert_snapshot;

/// Validates the documented history_modal E2E scenario.
#[test]
fn picker_catalog_grouping() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "history_modal",
        "picker_catalog_grouping",
        "The picker lists grouped conversations in deterministic order.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: history_modal
test: picker_catalog_grouping
description: The picker lists grouped conversations in deterministic order.
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
