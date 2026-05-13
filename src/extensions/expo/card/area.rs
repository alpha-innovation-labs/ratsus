use ratatui::layout::Rect;

/// Clickable Expo card geometry paired with the represented session index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpoCardArea {
    pub session_index: usize,
    pub area: Rect,
}
