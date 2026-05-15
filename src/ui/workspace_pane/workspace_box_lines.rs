use std::path::Path;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::app::state::app_state::AppState;
use crate::ui::left_panel::folder::blue_color::folder_blue_color;
use crate::ui::left_panel::folder::compact_display_name::folder_compact_display_name;
use crate::ui::left_panel::render::truncate_text_to_width::truncate_text_to_width;
use crate::ui::left_panel::session::title_color::session_title_color;
use crate::ui::workspace_pane::session_count_for_workspace::session_count_for_workspace;

/// Builds the rounded workspace folder box with its session count in the top border.
pub fn workspace_box_lines(app: &AppState, path: &Path, width: u16) -> Vec<Line<'static>> {
    if width < 2 {
        return vec![Line::styled(
            truncate_text_to_width(
                &session_count_for_workspace(path, &app.session_terminals).to_string(),
                usize::from(width),
            ),
            Style::default()
                .fg(folder_blue_color())
                .add_modifier(Modifier::BOLD),
        )];
    }

    let count = session_count_for_workspace(path, &app.session_terminals).to_string();
    let inner_width = usize::from(width).saturating_sub(2);
    let name = truncate_text_to_width(&folder_compact_display_name(path), inner_width);
    let name_padding = " ".repeat(inner_width.saturating_sub(name.chars().count()));
    let top_text = top_border_text(&count, inner_width);
    let top_padding = "─".repeat(inner_width.saturating_sub(top_text.chars().count()));
    let style = workspace_box_style(app, path);

    vec![
        Line::styled(format!("╭{}{}╮", top_padding, top_text), style),
        Line::styled(format!("│{}{}│", name, name_padding), style),
        Line::styled(format!("╰{}╯", "─".repeat(inner_width)), style),
    ]
}

/// Returns top border text with the chat count right aligned.
fn top_border_text(count: &str, inner_width: usize) -> String {
    if inner_width >= count.chars().count().saturating_add(1) {
        return format!(" {count}");
    }
    truncate_text_to_width(count, inner_width)
}

/// Returns the workspace box color for active, inactive, and dragged workspaces.
fn workspace_box_style(app: &AppState, path: &Path) -> Style {
    if app.workspace_drag.as_deref() == Some(path) {
        return Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
    }
    if app.selected_workspace_path.as_deref() == Some(path) {
        return Style::default()
            .fg(folder_blue_color())
            .add_modifier(Modifier::BOLD);
    }
    Style::default().fg(session_title_color())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;

    use super::workspace_box_lines;

    /// Verifies workspace folders render as rounded boxes without folder icons.
    #[test]
    fn renders_rounded_box_without_folder_icon() {
        let app =
            app_fixture(vec![dormant_session("one", "one", "/tmp/project")]).expect("app fixture");

        let lines = workspace_box_lines(&app, Path::new("/tmp/project"), 16);
        let text = lines_text(&lines);

        assert!(text[0].starts_with('╭'));
        assert!(text[1].starts_with('│'));
        assert!(text[2].starts_with('╰'));
        assert!(!text.iter().any(|line| line.contains('\u{f114}')));
        assert!(!text.iter().any(|line| line.contains('\u{f115}')));
    }

    /// Verifies the workspace session count is shown as a plain right-aligned number in the top border.
    #[test]
    fn renders_plain_count_in_top_border() {
        let app = app_fixture(vec![
            dormant_session("one", "one", "/tmp/project"),
            dormant_session("two", "two", "/tmp/project"),
        ])
        .expect("app fixture");

        let lines = workspace_box_lines(&app, Path::new("/tmp/project"), 16);
        let text = lines_text(&lines);

        assert!(text[0].contains(" 2╮"));
        assert!(!text[0].contains("("));
        assert!(!text[0].contains("/"));
    }

    /// Verifies inactive workspaces use the same gray as unfocused session rows.
    #[test]
    fn inactive_workspace_uses_session_gray() {
        let mut app =
            app_fixture(vec![dormant_session("one", "one", "/tmp/project")]).expect("app fixture");
        app.selected_workspace_path = Some("/tmp/other".into());

        let lines = workspace_box_lines(&app, Path::new("/tmp/project"), 16);

        assert_eq!(
            lines[0].style.fg,
            Some(crate::ui::left_panel::session::title_color::session_title_color())
        );
    }

    /// Verifies the active workspace keeps the shared folder blue.
    #[test]
    fn active_workspace_uses_folder_blue() {
        let app =
            app_fixture(vec![dormant_session("one", "one", "/tmp/project")]).expect("app fixture");

        let lines = workspace_box_lines(&app, Path::new("/tmp/project"), 16);

        assert_eq!(
            lines[0].style.fg,
            Some(crate::ui::left_panel::folder::blue_color::folder_blue_color())
        );
    }

    /// Returns plain text for rendered workspace box lines.
    fn lines_text(lines: &[ratatui::text::Line<'_>]) -> Vec<String> {
        lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect()
            })
            .collect()
    }
}
