use crate::app::app_state::AppState;

/// Returns the title for the pending delete confirmation target.
pub fn delete_session_title(app: &AppState) -> Option<String> {
    app.delete_confirmation.session_title.clone()
}

#[cfg(test)]
mod tests {
    use super::delete_session_title;

    /// This helper is covered through delete confirmation rendering tests.
    #[test]
    fn helper_is_linked() {
        let _ = delete_session_title;
    }
}
