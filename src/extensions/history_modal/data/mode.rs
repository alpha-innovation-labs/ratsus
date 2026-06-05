use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Describes what activating a conversation picker row should do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConversationPickerMode {
    /// Open selected conversations normally.
    #[default]
    Open,
    /// Place the selected existing conversation into a new split pane.
    PlaceInActiveSplit(TerminalSplitDirection),
}
