use ratatui::style::{Color, Style};
use ratkit::services::repo_watcher::GitFileStatus;

/// Returns the style for the visible git status letter.
pub fn git_status_label_style(status: GitFileStatus) -> Style {
    match status {
        GitFileStatus::Added => Style::default().fg(Color::Rgb(126, 211, 33)),
        GitFileStatus::Modified => Style::default().fg(Color::Rgb(91, 192, 222)),
        GitFileStatus::Deleted => Style::default().fg(Color::Rgb(255, 119, 132)),
        GitFileStatus::Renamed => Style::default().fg(Color::Rgb(177, 156, 217)),
        GitFileStatus::Untracked => Style::default().fg(Color::Rgb(245, 184, 0)),
    }
}
