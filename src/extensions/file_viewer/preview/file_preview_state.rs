use ratkit::widgets::code_widget::CodeState;
use ratkit::widgets::markdown_preview::MarkdownWidget;

/// Stores the active file-preview widget state for the selected tree item.
pub enum FilePreviewState {
    /// Preview state rendered through Ratkit's code widget.
    Code(Box<CodeState>),
    /// Preview state rendered through Ratkit's markdown widget.
    Markdown(Box<MarkdownWidget<'static>>),
}
