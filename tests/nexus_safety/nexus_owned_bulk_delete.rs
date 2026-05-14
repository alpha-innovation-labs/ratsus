use insta::assert_snapshot;

/// Validates the documented nexus_safety E2E scenario.
#[test]
fn nexus_owned_bulk_delete() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "nexus_safety",
        "nexus_owned_bulk_delete",
        "Multiple owned sessions can be deleted safely.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: nexus_safety
test: nexus_owned_bulk_delete
description: Multiple owned sessions can be deleted safely.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- delete confirmation workflow was visible
"###);
    Ok(())
}
