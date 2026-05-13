/// Result of handling a shared list-style keyboard shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKeyOutcome {
    Handled,
    Continue,
    Quit,
}
