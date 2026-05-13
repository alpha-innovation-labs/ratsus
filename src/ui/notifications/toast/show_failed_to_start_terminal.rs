use ratkit::primitives::toast::ToastManager;

/// Shows an error toast when a normal shell terminal cannot be started.
pub fn show_failed_to_start_terminal_toast(
    toast_manager: &mut ToastManager,
    error: &anyhow::Error,
) {
    toast_manager.error(format!("Failed to start terminal: {error}"));
}
