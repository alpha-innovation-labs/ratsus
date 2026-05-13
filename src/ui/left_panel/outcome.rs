/// Result returned after active left-pane content handles a semantic action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftPaneActionOutcome {
    Handled,
    Continue,
    Quit,
}
