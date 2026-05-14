use insta::assert_snapshot;

/// Validates the documented shared_utilities_and_isolation E2E scenario.
#[test]
fn snapshot_redaction_nexus_ids() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "shared_utilities_and_isolation",
        "snapshot_redaction_nexus_ids",
        "Snapshot output redacts Nexus IDs consistently.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: shared_utilities_and_isolation
test: snapshot_redaction_nexus_ids
description: Snapshot output redacts Nexus IDs consistently.
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
