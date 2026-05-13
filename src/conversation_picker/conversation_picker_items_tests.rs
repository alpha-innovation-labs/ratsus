use std::collections::BTreeSet;
use std::path::PathBuf;

use super::conversation_picker_items::conversation_picker_items;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::harness::chat_session::ChatSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds a dormant session fixture for picker grouping tests.
fn session(title: &str, folder: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new("today", title, title, folder))
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
) -> Vec<crate::conversation_picker::conversation_picker_item::ConversationPickerItem> {
    conversation_picker_items(
        sessions,
        folders,
        query,
        0,
        selected,
        None,
        &BTreeSet::new(),
    )
}
