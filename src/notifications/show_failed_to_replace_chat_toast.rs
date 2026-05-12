use ratkit::primitives::toast::ToastManager;

/// Shows an error toast when an exited chat cannot be replaced automatically.
pub fn show_failed_to_replace_chat_toast(toast_manager: &mut ToastManager, error: &anyhow::Error) {
    toast_manager.error(format!("Failed to replace exited chat: {error}"));
}
