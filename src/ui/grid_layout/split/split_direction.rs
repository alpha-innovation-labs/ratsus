/// Direction for creating a new split session from the active terminal pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalSplitDirection {
    /// Split the active pane vertically and place the new session on the right.
    Right,
    /// Split the active pane horizontally and place the new session below.
    Bottom,
}
