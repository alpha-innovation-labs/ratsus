use ratkit::primitives::toast::ToastManager;

/// Shows an error toast when deleting a session fails.
pub fn show_failed_to_delete_session_toast(
    toast_manager: &mut ToastManager,
    error: &anyhow::Error,
) {
    toast_manager.error(format!("Failed to delete session: {error}"));
}
