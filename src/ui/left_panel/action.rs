/// Semantic action produced by the left-pane shell for active content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftPaneAction {
    MoveBy(isize),
    ReorderBy(isize),
    Collapse,
    Expand,
    FocusFirst,
    FocusLast,
    FocusAdjacentGroup(isize),
    Activate,
    StartFilter,
    InsertFilterCharacter(char),
    DeleteFilterCharacter,
    Delete,
    ToggleSelection,
    Quit,
}
