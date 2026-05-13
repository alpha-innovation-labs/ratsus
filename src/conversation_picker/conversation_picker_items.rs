use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::conversation_picker::conversation_picker_item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::conversation_picker::matches_conversation_query::matches_conversation_query;
use crate::left_panel::folder_has_running_session::folder_has_running_session;
use crate::left_panel::format_session_age::format_session_age;
use crate::left_panel::session_icon::session_icon;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds grouped picker rows for folders and their matching conversations.
pub fn conversation_picker_items(
    sessions: &[SessionTerminal],
    folder_order: &[PathBuf],
    query: &str,
    active_index: usize,
    selected_conversation_ids: &BTreeSet<String>,
    folder_filter: Option<&Path>,
    collapsed_folders: &BTreeSet<PathBuf>,
) -> Vec<ConversationPickerItem> {
    folder_order
        .iter()
        .filter(|folder| folder_filter.map_or(true, |filter| folder.as_path() == filter))
        .flat_map(|folder| {
            folder_items(
                sessions,
                folder,
                query,
                active_index,
                selected_conversation_ids,
                collapsed_folders,
            )
        })
        .collect()
}

/// Builds one folder row plus its matching child session rows when visible.
fn folder_items(
    sessions: &[SessionTerminal],
    folder: &Path,
    query: &str,
    active_index: usize,
    selected_conversation_ids: &BTreeSet<String>,
    collapsed_folders: &BTreeSet<PathBuf>,
) -> Vec<ConversationPickerItem> {
    let session_indices = matching_folder_session_indices(sessions, folder, query);
    if session_indices.is_empty() {
        return Vec::new();
    }

    let total_session_count = total_folder_session_count(sessions, folder);
    let mut items = vec![folder_item(
        sessions,
        folder,
        session_indices.len(),
        total_session_count,
        collapsed_folders.contains(folder),
    )];
    items.extend(session_indices.into_iter().filter_map(|index| {
        session_item(sessions, index, active_index, selected_conversation_ids)
    }));
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

/// Counts all sessions that belong under one folder.
fn total_folder_session_count(sessions: &[SessionTerminal], folder: &Path) -> usize {
    sessions
        .iter()
        .filter(|entry| entry.session.working_dir == folder)
        .count()
}

/// Builds the selectable parent folder row that creates a new chat.
fn folder_item(
    sessions: &[SessionTerminal],
    folder: &Path,
    current_session_count: usize,
    total_session_count: usize,
    is_collapsed: bool,
) -> ConversationPickerItem {
    ConversationPickerItem {
        title: folder.display().to_string(),
        is_active: false,
        is_toggled: false,
        kind: ConversationPickerItemKind::Folder {
            path: folder.to_path_buf(),
            current_session_count,
            total_session_count,
            is_collapsed,
            has_running_session: folder_has_running_session(folder, sessions),
        },
    }
}

/// Builds a selectable child conversation row.
fn session_item(
    sessions: &[SessionTerminal],
    index: usize,
    active_index: usize,
    selected_conversation_ids: &BTreeSet<String>,
) -> Option<ConversationPickerItem> {
    let entry = sessions.get(index)?;
    Some(ConversationPickerItem {
        title: entry.session.title.clone(),
        is_active: index == active_index,
        is_toggled: selected_conversation_ids.contains(&entry.session.id),
        kind: ConversationPickerItemKind::Session {
            index,
            age: format_session_age(&entry.session, Utc::now()),
            icon: session_icon(&entry.session).to_string(),
            is_running: entry.session.is_running,
        },
    })
}
