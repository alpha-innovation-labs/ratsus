use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::extensions::history_modal::data::item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::extensions::history_modal::data::matches_query::matches_conversation_query;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::folder::has_running_session::folder_has_running_session;
use crate::ui::left_panel::session::format_age::format_session_age;
use crate::ui::left_panel::session::icon::session_icon;

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
    items.extend(
        pinned_folder_session_indices(sessions, session_indices, active_index)
            .into_iter()
            .filter_map(|index| {
                session_item(sessions, index, active_index, selected_conversation_ids)
            }),
    );
    items
}

/// Pins active and running sessions before other matching sessions in one folder.
fn pinned_folder_session_indices(
    sessions: &[SessionTerminal],
    indices: Vec<usize>,
    active_index: usize,
) -> Vec<usize> {
    let mut pinned = Vec::new();
    push_index_if_present(&mut pinned, &indices, active_index);
    for index in &indices {
        if sessions
            .get(*index)
            .is_some_and(|entry| entry.session.is_running)
        {
            push_index_if_present(&mut pinned, &indices, *index);
        }
    }
    for index in indices {
        push_unique_index(&mut pinned, index);
    }
    pinned
}

/// Adds an index when it belongs to the source list and is not already present.
fn push_index_if_present(target: &mut Vec<usize>, source: &[usize], index: usize) {
    if source.contains(&index) {
        push_unique_index(target, index);
    }
}

/// Adds an index when it is not already present.
fn push_unique_index(target: &mut Vec<usize>, index: usize) {
    if !target.contains(&index) {
        target.push(index);
    }
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
