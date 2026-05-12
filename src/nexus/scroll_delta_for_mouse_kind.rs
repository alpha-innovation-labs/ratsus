use crossterm::event::MouseEventKind;

/// Terminal scrollback action produced by a mouse wheel event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalScrollAction {
    Up(usize),
    Down(usize),
    None,
}

/// Maps a crossterm mouse event kind into a terminal scrollback action.
pub fn scroll_delta_for_mouse_kind(
    kind: MouseEventKind,
    lines_per_tick: usize,
) -> TerminalScrollAction {
    match kind {
        MouseEventKind::ScrollUp => TerminalScrollAction::Up(lines_per_tick),
        MouseEventKind::ScrollDown => TerminalScrollAction::Down(lines_per_tick),
        _ => TerminalScrollAction::None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{MouseButton, MouseEventKind};

    use super::{scroll_delta_for_mouse_kind, TerminalScrollAction};

    /// Verifies that upward wheel events scroll up by the configured amount.
    #[test]
    fn maps_scroll_up_to_up_action() {
        assert_eq!(
            scroll_delta_for_mouse_kind(MouseEventKind::ScrollUp, 3),
            TerminalScrollAction::Up(3)
        );
    }

    /// Verifies that downward wheel events scroll down by the configured amount.
    #[test]
    fn maps_scroll_down_to_down_action() {
        assert_eq!(
            scroll_delta_for_mouse_kind(MouseEventKind::ScrollDown, 4),
            TerminalScrollAction::Down(4)
        );
    }

    /// Verifies that non-wheel events do not alter terminal scrollback.
    #[test]
    fn ignores_non_scroll_events() {
        assert_eq!(
            scroll_delta_for_mouse_kind(MouseEventKind::Down(MouseButton::Left), 3),
            TerminalScrollAction::None
        );
    }
}
