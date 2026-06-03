use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratkit::services::repo_watcher::GitFileStatus;

use crate::extensions::file_viewer::tree::render::file_entry_icon::file_entry_icon;
use crate::extensions::file_viewer::tree::render::file_entry_indent::file_entry_indent;
use crate::extensions::file_viewer::tree::render::file_entry_name_style::file_entry_name_style;
use crate::extensions::file_viewer::tree::render::git_status_spans::git_status_spans;
use crate::extensions::file_viewer::tree::render::selected_file_entry_style::selected_file_entry_style;

/// Values needed to build one original-colored file-tree line.
pub struct FileEntryLineConfig<'a> {
    pub name: &'a str,
    pub depth: usize,
    pub is_dir: bool,
    pub expanded: bool,
    pub selected: bool,
    pub git_status: Option<GitFileStatus>,
    pub use_dark_theme: bool,
    pub dir_style: Style,
    pub file_style: Style,
}

/// Builds one file-tree row with the original Ratkit file colors.
pub fn file_entry_line(config: FileEntryLineConfig<'_>) -> Line<'static> {
    let indent = file_entry_indent(config.depth);
    let (icon, icon_color) = file_entry_icon(
        config.name,
        config.is_dir,
        config.expanded,
        config.use_dark_theme,
    );

    if config.selected {
        let style = selected_file_entry_style(config.is_dir, config.dir_style, config.file_style);
        let mut spans = vec![
            Span::styled(indent, style),
            Span::styled(format!("{icon} "), style),
        ];
        spans.extend(git_status_spans(config.git_status));
        spans.push(Span::styled(config.name.to_string(), style));
        return Line::from(spans);
    }

    let mut spans = vec![
        Span::raw(indent),
        Span::styled(format!("{icon} "), Style::default().fg(icon_color)),
    ];
    spans.extend(git_status_spans(config.git_status));
    spans.push(Span::styled(
        config.name.to_string(),
        file_entry_name_style(config.is_dir, config.dir_style, config.file_style),
    ));
    Line::from(spans)
}
