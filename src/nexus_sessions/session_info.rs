use std::path::PathBuf;

/// Session metadata returned by `nexus --sessions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NexusSession {
    pub date: String,
    pub title: String,
    pub id: String,
    pub working_dir: PathBuf,
    pub is_running: bool,
}

impl NexusSession {
    /// Builds a session metadata value from parsed table fields.
    pub fn new(
        date: impl Into<String>,
        title: impl Into<String>,
        id: impl Into<String>,
        working_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            date: date.into(),
            title: title.into(),
            id: id.into(),
            working_dir: working_dir.into(),
            is_running: false,
        }
    }

    /// Returns the same session metadata with the running flag set.
    pub fn with_running(mut self, is_running: bool) -> Self {
        self.is_running = is_running;
        self
    }
}
