use std::time::Duration;

use crate::app::diagnostics::app_diagnostics::AppDiagnostics;

/// Updates redraw totals and refreshes FPS once per elapsed second.
pub fn record_app_redraw(diagnostics: &mut AppDiagnostics) {
    diagnostics.redraws = diagnostics.redraws.saturating_add(1);
    diagnostics.frames_since_fps_update = diagnostics.frames_since_fps_update.saturating_add(1);

    let elapsed = diagnostics.last_fps_update.elapsed();
    if elapsed >= Duration::from_secs(1) {
        let elapsed_ms = elapsed.as_millis().max(1) as u32;
        diagnostics.fps =
            ((diagnostics.frames_since_fps_update.saturating_mul(1000)) / elapsed_ms) as u16;
        diagnostics.frames_since_fps_update = 0;
        diagnostics.last_fps_update = std::time::Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::app::diagnostics::new_app_diagnostics::new_app_diagnostics;
    use crate::app::diagnostics::record_app_redraw::record_app_redraw;

    /// Verifies redraw recording increments visible counters.
    #[test]
    fn increments_redraws_and_pending_frames() {
        let mut diagnostics = new_app_diagnostics();

        record_app_redraw(&mut diagnostics);

        assert_eq!(diagnostics.redraws, 1);
        assert_eq!(diagnostics.frames_since_fps_update, 1);
    }

    /// Verifies FPS updates after at least one elapsed second.
    #[test]
    fn updates_fps_after_elapsed_second() {
        let mut diagnostics = new_app_diagnostics();
        diagnostics.last_fps_update -= Duration::from_secs(1);

        record_app_redraw(&mut diagnostics);

        assert!(diagnostics.fps >= 1);
        assert_eq!(diagnostics.frames_since_fps_update, 0);
    }
}
