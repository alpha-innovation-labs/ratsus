use insta::assert_snapshot;

/// Validates the documented expo E2E scenario.
#[test]
fn expo_open_folder_cards() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "expo",
        "expo_open_folder_cards",
        "Opening Expo for a folder renders cards for that folder.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: expo
test: expo_open_folder_cards
description: Opening Expo for a folder renders cards for that folder.
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
