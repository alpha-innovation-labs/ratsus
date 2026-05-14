use insta::assert_snapshot;

/// Validates the documented session_refresh E2E scenario.
#[test]
fn refresh_nexus_adopts_owned_id() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_refresh",
        "refresh_nexus_adopts_owned_id",
        "Nexus refresh maps an owned placeholder to its real session ID and title.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_refresh
test: refresh_nexus_adopts_owned_id
description: Nexus refresh maps an owned placeholder to its real session ID and title.
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
