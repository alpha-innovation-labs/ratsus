use crate::app::state::app_state::AppState;
use crate::ui::left_panel::render::truncate_text_to_width::truncate_text_to_width;

const TITLE_WIDTH: usize = 54;
const MAX_VISIBLE_TITLES: usize = 6;

/// Builds the delete confirmation message for one or many sessions.
pub fn delete_session_confirmation_message(app: &AppState) -> String {
    delete_session_confirmation_message_for_titles(
        &app.delete_confirmation.session_titles,
        app.delete_confirmation.is_deleting,
    )
}

/// Builds a delete confirmation message from pending session titles.
pub fn delete_session_confirmation_message_for_titles(
    titles: &[String],
    is_deleting: bool,
) -> String {
    let header = confirmation_header(titles.len(), is_deleting);
    let mut lines = vec![header, String::new()];
    lines.extend(title_lines(titles));
    lines.join("\n")
}

/// Returns the delete dialog header line.
fn confirmation_header(count: usize, is_deleting: bool) -> String {
    let verb = if is_deleting { "Deleting" } else { "Delete" };
    if count == 1 {
        return format!("{verb} this conversation?");
    }
    format!("{verb} {count} selected conversations?")
}

/// Returns bullet-list title lines, truncating long titles.
fn title_lines(titles: &[String]) -> Vec<String> {
    let mut lines = titles
        .iter()
        .take(MAX_VISIBLE_TITLES)
        .map(|title| format!("• {}", truncate_text_to_width(title, TITLE_WIDTH)))
        .collect::<Vec<_>>();
    let hidden = titles.len().saturating_sub(MAX_VISIBLE_TITLES);
    if hidden > 0 {
        lines.push(format!("• ... and {hidden} more"));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::delete_session_confirmation_message_for_titles;

    /// Verifies delete confirmation lists selected titles one per line.
    #[test]
    fn lists_titles_as_bullets() {
        let message = delete_session_confirmation_message_for_titles(
            &["One".to_string(), "Two".to_string()],
            false,
        );

        assert!(message.contains("• One\n• Two"));
    }

    /// Verifies long titles are truncated in the delete confirmation list.
    #[test]
    fn truncates_long_titles() {
        let message = delete_session_confirmation_message_for_titles(&["x".repeat(80)], false);

        assert!(message.contains('…'));
    }
}
