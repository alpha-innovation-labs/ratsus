use crate::app::navigation::ordered_session_indices_from_left_rows::ordered_session_indices_from_left_rows;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Returns the adjacent session index by following visible left-pane row order with wrapping.
pub fn adjacent_session_index_in_left_pane_order(
    rows: &[SessionListRow],
    sessions: &[SessionTerminal],
    active_index: usize,
    direction: isize,
) -> Option<usize> {
    let session_indices = ordered_session_indices_from_left_rows(rows, sessions);
    if session_indices.is_empty() {
        return None;
    }
    let current_position = session_indices
        .iter()
        .position(|index| *index == active_index);
    match (current_position, direction.is_negative()) {
        (Some(position), false) => Some(session_indices[(position + 1) % session_indices.len()]),
        (Some(position), true) => {
            Some(session_indices[(position + session_indices.len() - 1) % session_indices.len()])
        }
        (None, false) => session_indices.first().copied(),
        (None, true) => session_indices.last().copied(),
    }
}

#[cfg(test)]
mod tests {
    use super::adjacent_session_index_in_left_pane_order;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::harness::core::chat_session::{ChatSession, ChatSessionKind};
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies next session follows visible left-pane order, including normal terminals.
    #[test]
    fn selects_next_session_in_left_pane_order() {
        let sessions = vec![chat("a"), normal_terminal("shell"), chat("b")];

        assert_eq!(
            adjacent_session_index_in_left_pane_order(&rows(&[2, 1, 0]), &sessions, 2, 1),
            Some(1)
        );
    }

    /// Verifies previous session follows visible left-pane order.
    #[test]
    fn selects_previous_session_in_left_pane_order() {
        let sessions = vec![chat("a"), normal_terminal("shell"), chat("b")];

        assert_eq!(
            adjacent_session_index_in_left_pane_order(&rows(&[2, 1, 0]), &sessions, 1, -1),
            Some(2)
        );
    }

    /// Verifies cycling wraps from the final visible session to the first visible session.
    #[test]
    fn wraps_next_session() {
        let sessions = vec![chat("a"), normal_terminal("shell")];

        assert_eq!(
            adjacent_session_index_in_left_pane_order(&rows(&[0, 1]), &sessions, 1, 1),
            Some(0)
        );
    }

    /// Verifies cycling starts at an edge when the active index is not visible.
    #[test]
    fn starts_from_edge_when_active_session_is_not_visible() {
        let sessions = vec![chat("a"), normal_terminal("shell"), chat("b")];

        assert_eq!(
            adjacent_session_index_in_left_pane_order(&rows(&[0, 2]), &sessions, 1, -1),
            Some(2)
        );
    }

    /// Builds a dormant chat fixture.
    fn chat(id: &str) -> SessionTerminal {
        dormant_session(id, id, "/tmp/project")
    }

    /// Builds a dormant normal terminal fixture.
    fn normal_terminal(id: &str) -> SessionTerminal {
        let session = ChatSession::new("now", id, id, "/tmp/project")
            .with_kind(ChatSessionKind::NormalTerminal);
        SessionTerminal::dormant(session)
    }

    /// Builds visible session rows in left-pane order.
    fn rows(indexes: &[usize]) -> Vec<SessionListRow> {
        indexes
            .iter()
            .map(|index| SessionListRow::Session { index: *index })
            .collect()
    }
}
