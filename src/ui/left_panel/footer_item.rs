/// One shortcut entry shown by the shared left-pane footer renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftPaneFooterItem {
    pub key: &'static str,
    pub description: &'static str,
}

impl LeftPaneFooterItem {
    /// Creates one shared left-pane footer item.
    pub const fn new(key: &'static str, description: &'static str) -> Self {
        Self { key, description }
    }
}
