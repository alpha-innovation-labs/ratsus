use std::collections::BTreeSet;

use chrono::{Duration, Utc};

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::order::order_with_user_preferences::order_with_user_preferences;

/// Builds a dormant session entry with explicit created and modified timestamps.
fn session_entry(created_at: &str, id: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new_with_created(
        created_at,
        created_at,
        id,
        id,
        "/tmp/project",
    ))
}

/// Builds an empty inserted-ids set.
fn empty_inserted() -> BTreeSet<String> {
    BTreeSet::new()
}

/// Builds an inserted-ids set with the requested ids.
fn inserted(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|id| (*id).to_string()).collect()
}

/// Verifies that known ids appear in the saved order before unpositioned entries.
#[test]
fn positions_known_ids_in_saved_order() {
    let mut entries = vec![
        session_entry("2026-05-10T10:00:00Z", "alpha"),
        session_entry("2026-05-11T10:00:00Z", "beta"),
    ];
    let saved_ids = vec!["beta".to_string(), "alpha".to_string()];

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    assert_eq!(entries[0].session.id, "beta");
    assert_eq!(entries[1].session.id, "alpha");
    assert_eq!(resolved, vec!["beta".to_string(), "alpha".to_string()]);
}

/// Verifies that unpositioned entries appear newest-first by creation date.
#[test]
fn appends_unpositioned_ids_in_creation_date_descending_order() {
    let mut entries = vec![
        session_entry("2026-05-09T10:00:00Z", "older"),
        session_entry("2026-05-10T10:00:00Z", "middle"),
        session_entry("2026-05-11T10:00:00Z", "newer"),
    ];
    let saved_ids: Vec<String> = Vec::new();

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    assert_eq!(entries[0].session.id, "newer");
    assert_eq!(entries[1].session.id, "middle");
    assert_eq!(entries[2].session.id, "older");
    assert_eq!(
        resolved,
        vec![
            "newer".to_string(),
            "middle".to_string(),
            "older".to_string()
        ]
    );
}

/// Locks the regression: a brand-new unpositioned session must always land at index 0,
/// even when its creation timestamp is older than the newest positioned session's
/// creation timestamp. The old "prepend if newer" rule dropped a T-1m new chat below
/// a T-0m "today" session because `1m < 0m`.
#[test]
fn always_prepends_unpositioned_even_when_older_than_newest_positioned() {
    let now = Utc::now();
    let today_created = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let one_minute_ago =
        (now - Duration::minutes(1)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let mut entries = vec![
        SessionTerminal::dormant(ChatSession::new_with_created(
            &today_created,
            &today_created,
            "today",
            "today-id",
            "/tmp/project",
        )),
        SessionTerminal::dormant(ChatSession::new_with_created(
            &one_minute_ago,
            &one_minute_ago,
            "new-chat",
            "new-chat-id",
            "/tmp/project",
        )),
    ];
    let saved_ids = vec!["today-id".to_string()];

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    assert_eq!(entries[0].session.id, "new-chat-id");
    assert_eq!(entries[1].session.id, "today-id");
    assert_eq!(
        resolved,
        vec!["new-chat-id".to_string(), "today-id".to_string()]
    );
}

/// Verifies that saved ids missing from the working set are dropped from the resolved order.
#[test]
fn drops_saved_ids_not_in_working_set() {
    let mut entries = vec![
        session_entry("2026-05-10T10:00:00Z", "alpha"),
        session_entry("2026-05-11T10:00:00Z", "beta"),
    ];
    let saved_ids = vec!["ghost".to_string(), "beta".to_string(), "alpha".to_string()];

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    assert_eq!(entries[0].session.id, "beta");
    assert_eq!(entries[1].session.id, "alpha");
    assert_eq!(resolved, vec!["beta".to_string(), "alpha".to_string()]);
}

/// Verifies the returned id list mirrors the resolved order of session terminals.
#[test]
fn returns_persistable_id_list_in_resolved_order() {
    let mut entries = vec![
        session_entry("2026-05-10T10:00:00Z", "alpha"),
        session_entry("2026-05-11T10:00:00Z", "beta"),
        session_entry("2026-05-09T10:00:00Z", "gamma"),
    ];
    let saved_ids = vec!["alpha".to_string()];

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    let actual_ids: Vec<String> = entries
        .iter()
        .map(|entry| entry.session.id.clone())
        .collect();
    assert_eq!(resolved, actual_ids);
}

/// Verifies inserted ids appear in the returned saved-ids list at their resolved slot.
#[test]
fn promotes_inserted_ids_to_positioned_slots() {
    let mut entries = vec![
        session_entry("2026-05-10T10:00:00Z", "saved-old"),
        session_entry("2026-05-12T10:00:00Z", "external-newer"),
    ];
    let saved_ids = vec!["saved-old".to_string()];

    let resolved =
        order_with_user_preferences(&mut entries, &saved_ids, &inserted(&["external-newer"]));

    assert!(resolved.contains(&"external-newer".to_string()));
    assert!(resolved.contains(&"saved-old".to_string()));
    let actual_ids: Vec<String> = entries
        .iter()
        .map(|entry| entry.session.id.clone())
        .collect();
    assert_eq!(resolved, actual_ids);
}

/// Verifies known ids keep their saved positions when no inserted ids are reported.
#[test]
fn keeps_existing_known_ids_unchanged_when_inserted_set_is_empty() {
    let mut entries = vec![
        session_entry("2026-05-10T10:00:00Z", "alpha"),
        session_entry("2026-05-11T10:00:00Z", "beta"),
        session_entry("2026-05-12T10:00:00Z", "gamma"),
    ];
    let saved_ids = vec!["gamma".to_string(), "alpha".to_string(), "beta".to_string()];

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    assert_eq!(entries[0].session.id, "gamma");
    assert_eq!(entries[1].session.id, "alpha");
    assert_eq!(entries[2].session.id, "beta");
    assert_eq!(resolved, saved_ids);
}

/// Verifies unpositioned entries still land above positioned entries when the unpositioned
/// block is older than the top positioned entry. There is no "newer than top" gate anymore.
#[test]
fn prepends_unpositioned_even_when_older_than_top_positioned() {
    let mut entries = vec![
        session_entry("2026-05-12T10:00:00Z", "saved-top"),
        session_entry("2026-05-11T10:00:00Z", "saved-mid"),
        session_entry("2026-05-10T10:00:00Z", "external-older"),
    ];
    let saved_ids = vec!["saved-top".to_string(), "saved-mid".to_string()];

    let resolved = order_with_user_preferences(&mut entries, &saved_ids, &empty_inserted());

    assert_eq!(entries[0].session.id, "external-older");
    assert_eq!(entries[1].session.id, "saved-top");
    assert_eq!(entries[2].session.id, "saved-mid");
    assert_eq!(
        resolved,
        vec![
            "external-older".to_string(),
            "saved-top".to_string(),
            "saved-mid".to_string(),
        ]
    );
}
