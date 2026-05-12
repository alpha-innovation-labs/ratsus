use std::collections::HashMap;

use crate::terminal::session_terminal::SessionTerminal;

/// Reorders sessions by saved session ids while appending unknown sessions in source order.
pub fn apply_session_id_order(entries: &mut [SessionTerminal], ordered_ids: &[String]) {
    let positions = ordered_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (id.as_str(), index))
        .collect::<HashMap<_, _>>();
    entries.sort_by_key(|entry| {
        positions
            .get(entry.session.id.as_str())
            .copied()
            .unwrap_or(usize::MAX)
    });
}
