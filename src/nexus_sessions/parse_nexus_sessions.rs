use std::path::Path;

use crate::nexus_sessions::session_info::NexusSession;

/// Parses `nexus --sessions` table output into session metadata rows.
pub fn parse_nexus_sessions(output: &str, working_dir: &Path) -> Vec<NexusSession> {
    output
        .lines()
        .filter_map(|line| parse_nexus_session_line(line, working_dir))
        .collect()
}

/// Parses one box-table row from `nexus --sessions` output.
fn parse_nexus_session_line(line: &str, working_dir: &Path) -> Option<NexusSession> {
    if !line.trim_start().starts_with('│') {
        return None;
    }

    let columns: Vec<&str> = line.split('│').map(str::trim).collect();
    if columns.len() < 5 || columns[1] == "Date" || columns[3] == "Session ID" {
        return None;
    }

    if columns[1].is_empty() || columns[2].is_empty() || columns[3].is_empty() {
        return None;
    }

    Some(NexusSession::new(
        columns[1],
        columns[2],
        columns[3],
        working_dir.to_path_buf(),
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_nexus_sessions;

    /// Verifies that Nexus table rows become session metadata values.
    #[test]
    fn parses_nexus_sessions_table() {
        let output = "┌──┬──┬──┐\n│ Date │ Session title │ Session ID │\n├──┼──┼──┤\n│ 2026-05-12 15:01:05 │ Add pane │ abc-123 │\n└──┴──┴──┘";

        let working_dir = std::path::PathBuf::from("/tmp/nexus-demo");
        let sessions = parse_nexus_sessions(output, &working_dir);

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].date, "2026-05-12 15:01:05");
        assert_eq!(sessions[0].title, "Add pane");
        assert_eq!(sessions[0].id, "abc-123");
        assert_eq!(sessions[0].working_dir, working_dir);
    }

    /// Verifies that headers and borders are ignored.
    #[test]
    fn ignores_table_header_and_borders() {
        let output = "┌──┬──┬──┐\n│ Date │ Session title │ Session ID │\n└──┴──┴──┘";

        assert!(parse_nexus_sessions(output, std::path::Path::new("/tmp/nexus-demo")).is_empty());
    }
}
