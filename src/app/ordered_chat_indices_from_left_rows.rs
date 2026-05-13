use crate::left_panel::session_list_row::SessionListRow;
use crate::terminal::is_chat_session::is_chat_session;
use crate::terminal::session_terminal::SessionTerminal;

/// Returns chat session indexes in the same order as visible left-pane rows.
pub fn ordered_chat_indices_from_left_rows(
    rows: &[SessionListRow],
    sessions: &[SessionTerminal],
) -> Vec<usize> {
    rows.iter()
        .filter_map(SessionListRow::session_index)
        .filter(|index| {
            sessions
                .get(*index)
                .is_some_and(|entry| is_chat_session(&entry.session))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::app::ordered_chat_indices_from_left_rows::ordered_chat_indices_from_left_rows;
    use crate::left_panel::session_list_row::SessionListRow;
    use crate::nexus_sessions::session_info::NexusSession;
    use crate::terminal::normal_terminal_session_info::normal_terminal_session_info;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Builds a dormant chat fixture for left-pane order tests.
    fn chat(id: &str) -> SessionTerminal {
        SessionTerminal::dormant(NexusSession::new("now", id, id, "/tmp/project"))
    }

    /// Builds a dormant terminal fixture for left-pane order tests.
    fn terminal() -> SessionTerminal {
        SessionTerminal::dormant(normal_terminal_session_info("/tmp/project".as_ref()))
    }

    /// Verifies chat indexes follow visible row order and skip normal terminals.
    #[test]
    fn returns_only_chat_indexes_in_visible_row_order() {
        let sessions = vec![chat("a"), terminal(), chat("b")];
        let rows = vec![
            SessionListRow::Session { index: 2 },
            SessionListRow::Session { index: 1 },
            SessionListRow::Session { index: 0 },
        ];

        assert_eq!(
            ordered_chat_indices_from_left_rows(&rows, &sessions),
            vec![2, 0]
        );
    }
}
