use std::collections::BTreeSet;
use std::path::PathBuf;

use chrono::{Duration, TimeZone, Utc};

use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::visible_session_rows::visible_session_rows;
use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds a dormant session entry for left-pane ordering tests.
fn session_entry(date: &str, title: &str, id: &str, working_dir: &str) -> SessionTerminal {
    SessionTerminal::dormant(NexusSession::new(date, title, id, working_dir))
}

/// Verifies visible rows sort folder sessions by newest activity first.
#[test]
fn sorts_folder_sessions_newest_first() {
    let older_date = "2026-05-11T10:00:00Z";
    let newer_date = "2026-05-12T10:00:00Z";
    let entries = vec![
        session_entry(older_date, "Older", "older", "/tmp/project"),
        session_entry(newer_date, "Newer", "newer", "/tmp/project"),
    ];

    let rows = visible_session_rows(&entries, &BTreeSet::new(), &[PathBuf::from("/tmp/project")]);

    assert_eq!(rows[1], SessionListRow::Session { index: 1 });
    assert_eq!(rows[2], SessionListRow::Session { index: 0 });
}

/// Verifies the sidebar shows the ten most recent sessions and excludes older entries.
#[test]
fn limits_folder_sessions_to_ten_most_recent() {
    let base = Utc.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
    let entries = (0..11)
        .map(|hours| {
            let date = (base - Duration::hours(hours)).to_rfc3339();
            session_entry(
                &date,
                &format!("Session {hours}"),
                &format!("id-{hours}"),
                "/tmp/project",
            )
        })
        .collect::<Vec<_>>();

    let rows = visible_session_rows(&entries, &BTreeSet::new(), &[PathBuf::from("/tmp/project")]);

    assert_eq!(
        rows[0],
        SessionListRow::Folder {
            path: PathBuf::from("/tmp/project"),
            current_session_count: 10,
            total_session_count: 11,
        }
    );
    for index in 0..10 {
        assert_eq!(rows[index + 1], SessionListRow::Session { index });
    }
    assert_eq!(
        rows[11],
        SessionListRow::FolderMore {
            path: PathBuf::from("/tmp/project"),
        }
    );
}
