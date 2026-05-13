use crate::extensions::harness::conversation_picker::data::matches_query::matches_conversation_query;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

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
