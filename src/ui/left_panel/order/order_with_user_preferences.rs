use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};
use std::mem;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::session::created_timestamp::session_created_timestamp;

/// Resolves session terminal order from saved preferences and any newly inserted ids.
pub fn order_with_user_preferences(
    session_terminals: &mut Vec<SessionTerminal>,
    saved_ids: &[String],
    _inserted_ids: &BTreeSet<String>,
) -> Vec<String> {
    let working = mem::take(session_terminals);
    let (positioned, unpositioned) = partition_by_saved_order(working, saved_ids);
    let unpositioned = sorted_by_newest_creation_first(unpositioned);
    *session_terminals = combined_order(positioned, unpositioned);
    session_terminals
        .iter()
        .map(|entry| entry.session.id.clone())
        .collect()
}

/// Splits the working set into entries placed by saved order and the remaining entries.
fn partition_by_saved_order(
    working: Vec<SessionTerminal>,
    saved_ids: &[String],
) -> (Vec<SessionTerminal>, Vec<SessionTerminal>) {
    let mut slots: Vec<Option<SessionTerminal>> = working.into_iter().map(Some).collect();
    let mut index_by_id = HashMap::with_capacity(slots.len());
    for (index, slot) in slots.iter().enumerate() {
        if let Some(entry) = slot {
            index_by_id.insert(entry.session.id.clone(), index);
        }
    }
    let mut positioned = Vec::with_capacity(saved_ids.len());
    for id in saved_ids {
        let Some(index) = index_by_id.remove(id) else {
            continue;
        };
        if let Some(entry) = slots[index].take() {
            positioned.push(entry);
        }
    }
    let unpositioned = slots.into_iter().flatten().collect();
    (positioned, unpositioned)
}

/// Returns the same entries sorted newest-creation-first.
fn sorted_by_newest_creation_first(mut entries: Vec<SessionTerminal>) -> Vec<SessionTerminal> {
    entries.sort_by_key(|entry| Reverse(session_created_timestamp(&entry.session)));
    entries
}

/// Combines positioned and unpositioned blocks; the unpositioned block is always prepended.
fn combined_order(
    positioned: Vec<SessionTerminal>,
    unpositioned: Vec<SessionTerminal>,
) -> Vec<SessionTerminal> {
    if unpositioned.is_empty() {
        return positioned;
    }
    if positioned.is_empty() {
        return unpositioned;
    }
    let mut combined = Vec::with_capacity(positioned.len() + unpositioned.len());
    combined.extend(unpositioned);
    combined.extend(positioned);
    combined
}
