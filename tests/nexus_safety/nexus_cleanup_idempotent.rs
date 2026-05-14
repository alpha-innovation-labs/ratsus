use insta::assert_snapshot;

/// Validates the documented nexus_safety E2E scenario.
#[test]
fn nexus_cleanup_idempotent() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "nexus_safety",
        "nexus_cleanup_idempotent",
        "Cleanup can run more than once without deleting foreign sessions.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: nexus_safety
test: nexus_cleanup_idempotent
description: Cleanup can run more than once without deleting foreign sessions.
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
