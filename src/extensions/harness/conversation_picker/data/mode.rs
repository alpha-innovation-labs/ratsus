/// Describes what activating a conversation picker row should do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConversationPickerMode {
    /// Open selected conversations normally.
    #[default]
    Open,
    /// Add the selected existing conversation to the active split pane.
    PlaceInActiveSplit,
}
