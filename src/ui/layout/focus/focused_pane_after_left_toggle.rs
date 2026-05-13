use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Returns the pane that should receive focus after the left pane visibility toggles.
pub fn focused_pane_after_left_pane_toggle(left_pane_visible: bool) -> FocusedPane {
    if left_pane_visible {
        FocusedPane::Left
    } else {
        FocusedPane::Terminal
    }
}
