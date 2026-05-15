use std::collections::BTreeSet;
use std::path::Path;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::split_groups::grouped_session_child::GroupedSessionChild;
use crate::ui::left_panel::session::split_groups::grouped_session_lookup::GroupedSessionLookup;

/// Appends folder session rows, replacing split members with one parent and child rows.
pub fn append_visible_folder_session_rows(
    rows: &mut Vec<SessionListRow>,
    session_terminals: &[SessionTerminal],
    folder: &Path,
    visible_indexes: &[usize],
    grouped_lookup: &GroupedSessionLookup,
    split_group_names: &std::collections::BTreeMap<u64, String>,
) {
    let mut emitted_group_ids = BTreeSet::new();
    for index in visible_indexes {
        let Some(group_id) = grouped_lookup.group_id_by_session_index.get(index).copied() else {
            rows.push(SessionListRow::Session { index: *index });
            continue;
        };
        if !emitted_group_ids.insert(group_id) {
            continue;
        }
        let children = folder_children(session_terminals, folder, group_id, grouped_lookup);
        if children.len() < 2 {
            rows.push(SessionListRow::Session { index: *index });
            continue;
        }
        let name = split_group_names
            .get(&group_id)
            .cloned()
            .unwrap_or_else(|| format!("Group {group_id}"));
        rows.push(SessionListRow::SplitGroup {
            group_id,
            name,
            child_count: children.len(),
        });
        append_child_rows(rows, group_id, &children);
    }
}

/// Returns group children that belong to the current folder.
fn folder_children(
    session_terminals: &[SessionTerminal],
    folder: &Path,
    group_id: u64,
    grouped_lookup: &GroupedSessionLookup,
) -> Vec<GroupedSessionChild> {
    grouped_lookup
        .children_by_group_id
        .get(&group_id)
        .into_iter()
        .flatten()
        .copied()
        .filter(|child| {
            session_terminals
                .get(child.session_index)
                .is_some_and(|entry| entry.session.working_dir == folder)
        })
        .collect()
}

/// Appends child rows with pipe-style last-child metadata.
fn append_child_rows(
    rows: &mut Vec<SessionListRow>,
    group_id: u64,
    children: &[GroupedSessionChild],
) {
    for (position, child) in children.iter().enumerate() {
        rows.push(SessionListRow::SplitGroupChild {
            group_id,
            pane_id: child.pane_id,
            index: child.session_index,
            is_last: position + 1 == children.len(),
        });
    }
}
