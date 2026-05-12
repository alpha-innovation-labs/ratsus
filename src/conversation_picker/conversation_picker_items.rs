use std::path::{Path, PathBuf};

use crate::conversation_picker::conversation_picker_item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::conversation_picker::matches_conversation_query::matches_conversation_query;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds grouped picker rows for folders and their matching conversations.
pub fn conversation_picker_items(
    sessions: &[SessionTerminal],
    folder_order: &[PathBuf],
    query: &str,
    active_index: usize,
    folder_filter: Option<&Path>,
) -> Vec<ConversationPickerItem> {
    folder_order
        .iter()
        .filter(|folder| folder_filter.map_or(true, |filter| folder.as_path() == filter))
        .flat_map(|folder| folder_items(sessions, folder, query, active_index))
        .collect()
}

/// Builds one folder row plus its matching child session rows when visible.
fn folder_items(
    sessions: &[SessionTerminal],
    folder: &Path,
    query: &str,
    active_index: usize,
) -> Vec<ConversationPickerItem> {
    let session_indices = matching_folder_session_indices(sessions, folder, query);
    if session_indices.is_empty() {
        return Vec::new();
    }

    let mut items = vec![folder_item(folder)];
    items.extend(
        session_indices
            .into_iter()
            .filter_map(|index| session_item(sessions, index, active_index)),
    );
    items
}

/// Returns matching session indices that belong under one folder.
fn matching_folder_session_indices(
    sessions: &[SessionTerminal],
    folder: &Path,
    query: &str,
) -> Vec<usize> {
    sessions
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            let in_folder = entry.session.working_dir == folder;
            let matches_query = matches_conversation_query(&entry.session, query);
            (in_folder && matches_query).then_some(index)
        })
        .collect()
}

/// Builds the selectable parent folder row that creates a new chat.
fn folder_item(folder: &Path) -> ConversationPickerItem {
    ConversationPickerItem {
        title: format!("{} (new chat)", folder.display()),
        subtitle: String::new(),
        is_active: false,
        kind: ConversationPickerItemKind::Folder {
            path: folder.to_path_buf(),
        },
    }
}

/// Builds a selectable child conversation row.
fn session_item(
    sessions: &[SessionTerminal],
    index: usize,
    active_index: usize,
) -> Option<ConversationPickerItem> {
    let entry = sessions.get(index)?;
    Some(ConversationPickerItem {
        title: entry.session.title.clone(),
        subtitle: entry.session.date.clone(),
        is_active: index == active_index,
        kind: ConversationPickerItemKind::Session { index },
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::conversation_picker_items;
    use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
    use crate::nexus_sessions::session_info::NexusSession;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Builds a dormant session fixture for picker grouping tests.
    fn session(title: &str, folder: &str) -> SessionTerminal {
        SessionTerminal::dormant(NexusSession::new("today", title, title, folder))
    }

    /// Folders should be emitted as selectable parent rows before their conversations.
    #[test]
    fn groups_sessions_under_folder_parent_rows() {
        let sessions = vec![session("Chat A", "/tmp/a"), session("Chat B", "/tmp/b")];
        let folders = vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")];

        let items = conversation_picker_items(&sessions, &folders, "", 0, None);

        assert!(matches!(
            items[0].kind,
            ConversationPickerItemKind::Folder { .. }
        ));
        assert_eq!(items[0].title, "/tmp/a (new chat)");
        assert!(matches!(
            items[1].kind,
            ConversationPickerItemKind::Session { index: 0 }
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

        let items = conversation_picker_items(&sessions, &folders, "rust", 0, None);

        assert_eq!(items.len(), 2);
        assert!(matches!(
            items[0].kind,
            ConversationPickerItemKind::Folder { .. }
        ));
        assert!(matches!(
            items[1].kind,
            ConversationPickerItemKind::Session { index: 0 }
        ));
    }

    /// Folder path matches should not include unrelated child conversations.
    #[test]
    fn query_does_not_match_folder_path() {
        let sessions = vec![session("Deploy server", "/tmp/sar-project")];
        let folders = vec![PathBuf::from("/tmp/sar-project")];

        let items = conversation_picker_items(&sessions, &folders, "sar", 0, None);

        assert!(items.is_empty());
    }

    /// Fuzzy title matches should not include unrelated child conversations.
    #[test]
    fn query_does_not_fuzzy_match_title() {
        let sessions = vec![session("super smallest for", "/tmp/a")];
        let folders = vec![PathBuf::from("/tmp/a")];

        let items = conversation_picker_items(&sessions, &folders, "sar", 0, None);

        assert!(items.is_empty());
    }
}
