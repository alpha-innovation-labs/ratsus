use std::time::Instant;

/// Runtime counters shown in the app diagnostics area.
pub struct AppDiagnostics {
    /// Most recently calculated frames per second.
    pub fps: u16,
    /// Total number of app redraws recorded during this process.
    pub redraws: u64,
    /// Number of frames counted since the last FPS calculation.
    pub frames_since_fps_update: u32,
    /// Time at which FPS was last calculated.
    pub last_fps_update: Instant,
}
