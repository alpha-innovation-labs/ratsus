use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::extensions::history_modal::data::item::ConversationPickerItemKind;
use crate::extensions::history_modal::data::items::conversation_picker_items;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Builds a dormant session fixture for picker grouping tests.
fn session(title: &str, folder: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new("today", title, title, folder))
}

/// Builds a dormant running session fixture for picker grouping tests.
fn running_session(title: &str, folder: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new("today", title, title, folder).with_running(true))
}

/// Folders should be emitted as selectable parent rows before their conversations.
#[test]
fn groups_sessions_under_folder_parent_rows() {
    let sessions = vec![session("Chat A", "/tmp/a"), session("Chat B", "/tmp/b")];
    let folders = vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")];

    let items = picker_items(&sessions, &folders, "", &BTreeSet::new());

    assert!(matches!(
        items[0].kind,
        ConversationPickerItemKind::Folder { .. }
    ));
    assert_eq!(items[0].title, "/tmp/a");
    assert!(matches!(
        items[1].kind,
        ConversationPickerItemKind::Session { index: 0, .. }
    ));
    assert!(matches!(
        items[2].kind,
        ConversationPickerItemKind::Folder { .. }
    ));
}

/// Matching a child conversation should keep its folder parent visible.
#[test]
fn query_keeps_matching_child_with_folder_parent() {
    let sessions = vec![
        session("Rust modal", "/tmp/a"),
        session("Python cli", "/tmp/b"),
    ];
    let folders = vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")];

    let items = picker_items(&sessions, &folders, "rust", &BTreeSet::new());

    assert_eq!(items.len(), 2);
    assert!(matches!(
        items[0].kind,
        ConversationPickerItemKind::Folder { .. }
    ));
    assert!(matches!(
        items[1].kind,
        ConversationPickerItemKind::Session { index: 0, .. }
    ));
}

/// Folder path matches should not include unrelated child conversations.
#[test]
fn query_does_not_match_folder_path() {
    let sessions = vec![session("Deploy server", "/tmp/sar-project")];
    let folders = vec![PathBuf::from("/tmp/sar-project")];

    let items = picker_items(&sessions, &folders, "sar", &BTreeSet::new());

    assert!(items.is_empty());
}

/// Fuzzy title matches should not include unrelated child conversations.
#[test]
fn query_does_not_fuzzy_match_title() {
    let sessions = vec![session("super smallest for", "/tmp/a")];
    let folders = vec![PathBuf::from("/tmp/a")];

    let items = picker_items(&sessions, &folders, "sar", &BTreeSet::new());

    assert!(items.is_empty());
}

/// The active conversation should be pinned above other matches in its folder.
#[test]
fn pins_active_session_to_top_of_folder() {
    let sessions = vec![
        session("Older", "/tmp/a"),
        session("Active", "/tmp/a"),
        session("Other", "/tmp/a"),
    ];
    let folders = vec![PathBuf::from("/tmp/a")];

    let items = picker_items_with_active(&sessions, &folders, "", &BTreeSet::new(), 1);

    assert!(matches!(
        items[1].kind,
        ConversationPickerItemKind::Session { index: 1, .. }
    ));
    assert!(items[1].is_active);
}

/// Running conversations should be pinned after the active row in their folder.
#[test]
fn pins_running_sessions_after_active_session() {
    let sessions = vec![
        session("Older", "/tmp/a"),
        session("Active", "/tmp/a"),
        running_session("Running", "/tmp/a"),
    ];
    let folders = vec![PathBuf::from("/tmp/a")];

    let items = picker_items_with_active(&sessions, &folders, "", &BTreeSet::new(), 1);

    assert!(matches!(
        items[1].kind,
        ConversationPickerItemKind::Session { index: 1, .. }
    ));
    assert!(matches!(
        items[2].kind,
        ConversationPickerItemKind::Session { index: 2, .. }
    ));
}

/// Selected conversation ids should mark matching picker session rows as toggled.
#[test]
fn marks_toggled_session_items() {
    let sessions = vec![session("Chat A", "/tmp/a")];
    let folders = vec![PathBuf::from("/tmp/a")];
    let selected = BTreeSet::from(["Chat A".to_string()]);

    let items = picker_items(&sessions, &folders, "", &selected);

    assert!(items[1].is_toggled);
}

/// Builds picker items with default active and folder state for tests.
fn picker_items(
    sessions: &[SessionTerminal],
    folders: &[PathBuf],
    query: &str,
    selected: &BTreeSet<String>,
) -> Vec<crate::extensions::history_modal::data::item::ConversationPickerItem> {
    picker_items_with_active(sessions, folders, query, selected, 0)
}

/// Builds picker items with a custom active index for tests.
fn picker_items_with_active(
    sessions: &[SessionTerminal],
    folders: &[PathBuf],
    query: &str,
    selected: &BTreeSet<String>,
    active_index: usize,
) -> Vec<crate::extensions::history_modal::data::item::ConversationPickerItem> {
    conversation_picker_items(
        sessions,
        folders,
        query,
        active_index,
        selected,
        None,
        &BTreeSet::new(),
    )
}
