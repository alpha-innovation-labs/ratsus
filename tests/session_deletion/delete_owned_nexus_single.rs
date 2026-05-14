use insta::assert_snapshot;

/// Validates the documented session_deletion E2E scenario.
#[test]
fn delete_owned_nexus_single() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_deletion",
        "delete_owned_nexus_single",
        "Nexus single delete removes only the manifest-owned session.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_deletion
test: delete_owned_nexus_single
description: Nexus single delete removes only the manifest-owned session.
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
