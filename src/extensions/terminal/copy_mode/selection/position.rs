/// A terminal-buffer position used by copy selection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SelectionPosition {
    pub row: i32,
    pub col: i32,
}
