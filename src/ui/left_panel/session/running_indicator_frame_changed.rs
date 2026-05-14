use crate::ui::left_panel::session::running_indicator::running_session_indicator;

/// Returns whether two loader ticks render different running-session indicator glyphs.
pub fn running_indicator_frame_changed(previous_tick: u64, current_tick: u64) -> bool {
    running_session_indicator(previous_tick) != running_session_indicator(current_tick)
}

#[cfg(test)]
mod tests {
    use super::running_indicator_frame_changed;

    /// Verifies unchanged spinner frames do not request unnecessary redraws.
    #[test]
    fn detects_only_visible_frame_changes() {
        assert!(!running_indicator_frame_changed(0, 1));
        assert!(running_indicator_frame_changed(3, 4));
        assert!(!running_indicator_frame_changed(4, 5));
    }
}
