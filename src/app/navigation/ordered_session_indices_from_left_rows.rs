use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Returns session indexes in the same order as visible left-pane rows.
pub fn ordered_session_indices_from_left_rows(
    rows: &[SessionListRow],
    sessions: &[SessionTerminal],
) -> Vec<usize> {
    let mut ordered = Vec::new();
    for index in rows.iter().filter_map(SessionListRow::session_index) {
        if sessions.get(index).is_some() && !ordered.contains(&index) {
            ordered.push(index);
        }
    }
    ordered
}

#[cfg(test)]
mod tests {
    use super::ordered_session_indices_from_left_rows;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::harness::core::chat_session::{ChatSession, ChatSessionKind};
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies all session kinds follow visible row order.
    #[test]
    fn returns_all_session_indexes_in_visible_row_order() {
        let sessions = vec![chat("a"), normal_terminal("shell"), chat("b")];
        let rows = vec![
            SessionListRow::Session { index: 2 },
            SessionListRow::Session { index: 1 },
            SessionListRow::Session { index: 0 },
        ];

        assert_eq!(
            ordered_session_indices_from_left_rows(&rows, &sessions),
            vec![2, 1, 0]
        );
    }

    /// Verifies duplicate rows do not duplicate a session in cycle order.
    #[test]
    fn deduplicates_repeated_session_indexes() {
        let sessions = vec![chat("a")];
        let rows = vec![
            SessionListRow::Session { index: 0 },
            SessionListRow::Session { index: 0 },
        ];

        assert_eq!(
            ordered_session_indices_from_left_rows(&rows, &sessions),
            vec![0]
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
}
