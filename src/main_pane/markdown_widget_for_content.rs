use ratkit::widgets::markdown_preview::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    MarkdownWidget, ScrollState, SelectionState, SourceState, VimState,
};

/// Builds a persistent Ratkit markdown preview widget for file content.
pub fn markdown_widget_for_content(content: String) -> MarkdownWidget<'static> {
    let mut source = SourceState::default();
    source.set_source_string(content.clone());

    let mut scroll = ScrollState::default();
    scroll.update_total_lines(content.lines().count().max(1));

    let mut display = DisplaySettings::default();
    display.set_show_document_line_numbers(true);
    display.set_show_line_numbers(true);

    MarkdownWidget::new(
        content,
        scroll,
        source,
        CacheState::default(),
        display,
        CollapseState::default(),
        ExpandableState::default(),
        GitStatsState::default(),
        VimState::default(),
        SelectionState::default(),
        DoubleClickState::default(),
    )
    .with_has_pane(false)
    .show_scrollbar(true)
    .show_statusline(true)
}
