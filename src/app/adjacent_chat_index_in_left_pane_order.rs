use crate::app::ordered_chat_indices_from_left_rows::ordered_chat_indices_from_left_rows;
use crate::left_panel::session_list_row::SessionListRow;
use crate::terminal::session_terminal::SessionTerminal;

/// Returns the adjacent chat index by following visible left-pane row order with wrapping.
pub fn adjacent_chat_index_in_left_pane_order(
    rows: &[SessionListRow],
    sessions: &[SessionTerminal],
    active_index: usize,
    direction: isize,
) -> Option<usize> {
    let chat_indices = ordered_chat_indices_from_left_rows(rows, sessions);
    if chat_indices.is_empty() {
        return None;
    }
    let current_position = chat_indices.iter().position(|index| *index == active_index);
    match (current_position, direction.is_negative()) {
        (Some(position), false) => Some(chat_indices[(position + 1) % chat_indices.len()]),
        (Some(position), true) => {
            Some(chat_indices[(position + chat_indices.len() - 1) % chat_indices.len()])
        }
        (None, false) => chat_indices.first().copied(),
        (None, true) => chat_indices.last().copied(),
    }
}

#[cfg(test)]
mod tests {
    use super::adjacent_chat_index_in_left_pane_order;
    use crate::harness::chat_session::ChatSession;
    use crate::left_panel::session_list_row::SessionListRow;
    use crate::terminal::normal_terminal_session_info::normal_terminal_session_info;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Builds a dormant chat fixture for adjacent chat tests.
    fn chat(id: &str) -> SessionTerminal {
        SessionTerminal::dormant(ChatSession::new("now", id, id, "/tmp/project"))
    }

    /// Builds a dormant terminal fixture for adjacent chat tests.
    fn terminal() -> SessionTerminal {
        SessionTerminal::dormant(normal_terminal_session_info("/tmp/project".as_ref()))
    }

    /// Builds visible session rows in left-pane order.
    fn rows(indexes: &[usize]) -> Vec<SessionListRow> {
        indexes
            .iter()
            .map(|index| SessionListRow::Session { index: *index })
            .collect()
    }

    /// Verifies next chat follows visible left-pane order instead of vector order.
    #[test]
    fn selects_next_chat_in_left_pane_order() {
        let sessions = vec![chat("a"), terminal(), chat("b")];

        assert_eq!(
            adjacent_chat_index_in_left_pane_order(&rows(&[2, 1, 0]), &sessions, 2, 1),
            Some(0)
        );
    }

    /// Verifies previous chat follows visible left-pane order.
    #[test]
    fn selects_previous_chat_in_left_pane_order() {
        let sessions = vec![chat("a"), terminal(), chat("b")];

        assert_eq!(
            adjacent_chat_index_in_left_pane_order(&rows(&[2, 1, 0]), &sessions, 0, -1),
            Some(2)
        );
    }

    /// Verifies cycling wraps from the final visible chat to the first visible chat.
    #[test]
    fn wraps_next_chat() {
        let sessions = vec![chat("a"), chat("b")];

        assert_eq!(
            adjacent_chat_index_in_left_pane_order(&rows(&[0, 1]), &sessions, 1, 1),
            Some(0)
        );
    }

    /// Verifies cycling starts at an edge when the active session is not a chat row.
    #[test]
    fn starts_from_edge_when_active_session_is_not_chat() {
        let sessions = vec![chat("a"), terminal(), chat("b")];

        assert_eq!(
            adjacent_chat_index_in_left_pane_order(&rows(&[0, 1, 2]), &sessions, 1, -1),
            Some(2)
        );
    }
}
