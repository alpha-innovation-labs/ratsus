use ratkit::primitives::toast::ToastManager;

/// Shows an error toast when a fresh Nexus chat cannot be started.
pub fn show_failed_to_start_new_chat_toast(
    toast_manager: &mut ToastManager,
    error: &anyhow::Error,
) {
    toast_manager.error(format!("Failed to start Nexus chat: {error}"));
}
