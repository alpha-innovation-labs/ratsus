use ratatui::text::Span;
use ratkit::services::repo_watcher::GitFileStatus;

use crate::extensions::file_viewer::tree::git_status::git_status_dot_style::git_status_dot_style;
use crate::extensions::file_viewer::tree::git_status::git_status_label_style::git_status_label_style;
use crate::extensions::file_viewer::tree::git_status::status_symbol::status_symbol;

/// Builds the dot and label spans displayed before a changed file-tree row name.
pub fn git_status_spans(status: Option<GitFileStatus>) -> Vec<Span<'static>> {
    let Some(status) = status else {
        return vec![];
    };
    vec![
        Span::styled("● ", git_status_dot_style()),
        Span::styled(
            status_symbol(status).to_string(),
            git_status_label_style(status),
        ),
        Span::raw(" "),
    ]
}

#[cfg(test)]
mod tests {
    use ratkit::services::repo_watcher::GitFileStatus;

    use super::git_status_spans;

    /// Git status spans should include a dot marker and short status label.
    #[test]
    fn renders_dot_and_status_label() {
        let spans = git_status_spans(Some(GitFileStatus::Modified));

        assert_eq!(spans[0].content.as_ref(), "● ");
        assert_eq!(spans[1].content.as_ref(), "M");
    }
}
