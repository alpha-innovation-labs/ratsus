use insta::assert_snapshot;

/// Validates the documented session_deletion E2E scenario.
#[test]
fn delete_guard_blocks_foreign_nexus_id() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_deletion",
        "delete_guard_blocks_foreign_nexus_id",
        "Nexus cleanup rejects IDs not recorded in the test manifest.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_deletion
test: delete_guard_blocks_foreign_nexus_id
description: Nexus cleanup rejects IDs not recorded in the test manifest.
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
