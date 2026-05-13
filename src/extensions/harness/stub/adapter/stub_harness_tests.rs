use std::collections::HashMap;

use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::extensions::harness::stub::adapter::stub_harness::StubHarness;

/// Verifies refreshes keep stub active states stable for deterministic UI checks.
#[test]
fn refresh_sessions_preserves_active_conversation_distribution() {
    let harness = StubHarness::new();

    let first_refresh = harness.refresh_sessions().expect("first refresh succeeds");
    let second_refresh = harness.refresh_sessions().expect("second refresh succeeds");

    assert_eq!(
        active_counts(&first_refresh),
        active_counts(&second_refresh)
    );
    assert_eq!(
        active_counts(&second_refresh).get("/tmp/ratsus-alpha"),
        Some(&1)
    );
    assert_eq!(
        active_counts(&second_refresh).get("/tmp/ratsus-beta"),
        Some(&1)
    );
    assert_eq!(active_counts(&second_refresh).len(), 2);
}

/// Counts active conversations per project folder.
fn active_counts(
    sessions: &[crate::extensions::harness::core::chat_session::ChatSession],
) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for session in sessions.iter().filter(|session| session.is_running) {
        let folder = session.working_dir.display().to_string();
        *counts.entry(folder).or_default() += 1;
    }
    counts
}
