use std::collections::HashMap;

use crate::extensions::harness::stub::data::stub_sessions::stub_sessions;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::order::session_folder_order::session_folder_order;
use crate::ui::left_panel::session::sort_by_creation_date::sort_sessions_by_creation_date;

/// Verifies the deterministic stub catalog covers the requested folder distribution.
#[test]
fn stub_sessions_have_requested_folder_distribution() {
    let sessions = stub_sessions();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for session in sessions {
        let folder = session.working_dir.display().to_string();
        *counts.entry(folder).or_default() += 1;
    }

    assert_eq!(counts.get("/tmp/ratsus-alpha"), Some(&15));
    assert_eq!(counts.get("/tmp/ratsus-beta"), Some(&15));
    assert_eq!(counts.get("/tmp/ratsus-gamma"), Some(&5));
    assert_eq!(counts.get("/tmp/ratsus-delta"), Some(&4));
    assert_eq!(counts.get("/tmp/ratsus-epsilon"), Some(&2));
    assert_eq!(counts.len(), 5);
}

/// Verifies only the first two stub folders start with one active conversation each.
#[test]
fn stub_sessions_only_top_two_folders_have_active_conversations() {
    let sessions = stub_sessions();
    let mut active_counts: HashMap<String, usize> = HashMap::new();

    for session in sessions.into_iter().filter(|session| session.is_running) {
        let folder = session.working_dir.display().to_string();
        *active_counts.entry(folder).or_default() += 1;
    }

    assert_eq!(active_counts.get("/tmp/ratsus-alpha"), Some(&1));
    assert_eq!(active_counts.get("/tmp/ratsus-beta"), Some(&1));
    assert_eq!(active_counts.get("/tmp/ratsus-gamma"), None);
    assert_eq!(active_counts.get("/tmp/ratsus-delta"), None);
    assert_eq!(active_counts.get("/tmp/ratsus-epsilon"), None);
    assert_eq!(active_counts.len(), 2);
}

/// Verifies projects with more than ten sessions are ordered before smaller projects at startup.
#[test]
fn stub_sessions_put_large_projects_on_top_after_startup_sort() {
    let mut entries = stub_sessions()
        .into_iter()
        .map(SessionTerminal::dormant)
        .collect::<Vec<_>>();

    sort_sessions_by_creation_date(&mut entries);
    let folders = session_folder_order(&entries);

    assert_eq!(folders[0].display().to_string(), "/tmp/ratsus-alpha");
    assert_eq!(folders[1].display().to_string(), "/tmp/ratsus-beta");
}
