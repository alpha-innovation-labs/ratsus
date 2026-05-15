use std::time::Instant;

use crate::app::diagnostics::app_diagnostics::AppDiagnostics;

/// Builds a clean diagnostics counter set for a new app process.
pub fn new_app_diagnostics() -> AppDiagnostics {
    AppDiagnostics {
        fps: 0,
        redraws: 0,
        frames_since_fps_update: 0,
        last_fps_update: Instant::now(),
    }
}
