use std::path::PathBuf;

/// Returns the file path used to persist left-panel ordering preferences.
pub fn session_order_preferences_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from).map(|home| {
        home.join(".config")
            .join("ratsus")
            .join("session-order.json")
    })
}
