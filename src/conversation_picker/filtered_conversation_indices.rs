use crate::conversation_picker::matches_conversation_query::matches_conversation_query;
use crate::terminal::session_terminal::SessionTerminal;

/// Returns ordered session indices matching the picker query.
pub fn filtered_conversation_indices(sessions: &[SessionTerminal], query: &str) -> Vec<usize> {
    sessions
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            matches_conversation_query(&entry.session, query).then_some(index)
        })
        .collect()
}
