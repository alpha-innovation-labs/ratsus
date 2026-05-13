/// Returns the animated loader glyph for a running chat session.
pub fn running_session_indicator(tick: u64) -> &'static str {
    const FRAMES: [&str; 4] = ["⠋", "⠙", "⠹", "⠸"];
    const TICKS_PER_FRAME: u64 = 4;
    FRAMES[((tick / TICKS_PER_FRAME) as usize) % FRAMES.len()]
}

#[cfg(test)]
mod tests {
    use super::running_session_indicator;

    /// Verifies the loader cycles through deterministic spinner frames.
    #[test]
    fn cycles_running_indicator_frames() {
        assert_eq!(running_session_indicator(0), "⠋");
        assert_eq!(running_session_indicator(3), "⠋");
        assert_eq!(running_session_indicator(4), "⠙");
        assert_eq!(running_session_indicator(16), "⠋");
    }
}
