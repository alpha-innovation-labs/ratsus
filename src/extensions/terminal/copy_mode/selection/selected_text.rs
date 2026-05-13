use crate::extensions::terminal::copy_mode::selection::bounds::selection_bounds;
use crate::extensions::terminal::copy_mode::selection::copy_selection::TerminalCopySelection;

/// Extracts selected text from the frozen terminal screen.
pub fn selected_text_from_terminal_copy_selection(
    selection: &TerminalCopySelection,
) -> Option<String> {
    let screen = selection.snapshot.as_ref()?;
    let anchor = selection.anchor?;
    let cursor = selection.cursor?;
    let bounds = selection_bounds(anchor, cursor);
    Some(screen.get_selected_text(
        bounds.start.col,
        bounds.start.row,
        bounds.end.col,
        bounds.end.row,
    ))
}
