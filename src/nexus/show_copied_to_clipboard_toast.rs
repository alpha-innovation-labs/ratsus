use ratkit::primitives::toast::{Toast, ToastLevel, ToastManager};
use std::time::Duration;

/// Display duration for the copied-to-clipboard confirmation toast.
pub const COPIED_TO_CLIPBOARD_TOAST_DURATION: Duration = Duration::from_secs(2);

/// Shows the terminal copy confirmation toast for the configured short duration.
pub fn show_copied_to_clipboard_toast(toast_manager: &mut ToastManager) {
    toast_manager.add(Toast::with_duration(
        "Copied to clipboard",
        ToastLevel::Success,
        COPIED_TO_CLIPBOARD_TOAST_DURATION,
    ));
}
