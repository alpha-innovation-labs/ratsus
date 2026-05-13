/// Result sent by the background delete worker.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeleteSessionsResult {
    pub deleted_chat_ids: Vec<String>,
    pub failed_chat_ids: Vec<String>,
    pub errors: Vec<String>,
}

impl DeleteSessionsResult {
    /// Returns true when at least one delete command failed.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns all delete errors as a single readable message.
    pub fn error_message(&self) -> String {
        self.errors.join("; ")
    }
}
