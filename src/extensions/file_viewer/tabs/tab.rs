/// Selectable tabs displayed in the main main pane title bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainPaneTab {
    Chat,
    Files,
    Diff,
    Expo,
}
