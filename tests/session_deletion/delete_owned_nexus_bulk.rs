use insta::assert_snapshot;

/// Validates the documented session_deletion E2E scenario.
#[test]
fn delete_owned_nexus_bulk() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_deletion",
        "delete_owned_nexus_bulk",
        "Nexus bulk delete removes only manifest-owned selected sessions.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_deletion
test: delete_owned_nexus_bulk
description: Nexus bulk delete removes only manifest-owned selected sessions.
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
