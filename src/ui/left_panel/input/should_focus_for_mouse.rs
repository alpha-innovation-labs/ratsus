use crossterm::event::{MouseButton, MouseEventKind};

/// Returns true when mouse input should move keyboard focus to the left pane.
pub fn should_focus_left_pane_for_mouse(kind: MouseEventKind) -> bool {
    matches!(
        kind,
        MouseEventKind::Down(MouseButton::Left)
            | MouseEventKind::Down(MouseButton::Right)
            | MouseEventKind::Down(MouseButton::Middle)
            | MouseEventKind::Drag(MouseButton::Left)
            | MouseEventKind::Moved
            | MouseEventKind::ScrollUp
            | MouseEventKind::ScrollDown
            | MouseEventKind::ScrollLeft
            | MouseEventKind::ScrollRight
    )
}

#[cfg(test)]
mod tests {
    use crossterm::event::MouseEventKind;

    use super::should_focus_left_pane_for_mouse;

    /// Verifies plain hover movement moves keyboard focus to the left pane.
    #[test]
    fn hover_focuses_left_pane() {
        assert!(should_focus_left_pane_for_mouse(MouseEventKind::Moved));
    }

    /// Verifies scroll input can focus the left pane.
    #[test]
    fn scroll_focuses_left_pane() {
        assert!(should_focus_left_pane_for_mouse(MouseEventKind::ScrollDown));
    }
}
