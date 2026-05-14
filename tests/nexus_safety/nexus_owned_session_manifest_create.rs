use insta::assert_snapshot;

/// Validates the documented nexus_safety E2E scenario.
#[test]
fn nexus_owned_session_manifest_create() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "nexus_safety",
        "nexus_owned_session_manifest_create",
        "Mutating tests record owned session IDs before cleanup is allowed.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: nexus_safety
test: nexus_owned_session_manifest_create
description: Mutating tests record owned session IDs before cleanup is allowed.
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
