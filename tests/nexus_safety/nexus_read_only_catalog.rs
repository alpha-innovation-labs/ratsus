use insta::assert_snapshot;

/// Validates the documented nexus_safety E2E scenario.
#[test]
fn nexus_read_only_catalog() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "nexus_safety",
        "nexus_read_only_catalog",
        "Read-only Nexus tests inspect existing sessions without mutation.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: nexus_safety
test: nexus_read_only_catalog
description: Read-only Nexus tests inspect existing sessions without mutation.
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
