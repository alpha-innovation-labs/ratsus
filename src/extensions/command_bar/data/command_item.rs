use crate::extensions::command_bar::command::command_id::CommandBarCommandId;

/// Display metadata for one executable command bar row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBarItem {
    pub id: CommandBarCommandId,
    pub title: &'static str,
    pub hotkey: &'static str,
}

impl CommandBarItem {
    /// Creates display metadata for one command bar command.
    pub fn new(id: CommandBarCommandId, title: &'static str, hotkey: &'static str) -> Self {
        Self { id, title, hotkey }
    }
}
