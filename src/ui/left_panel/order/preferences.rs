use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::extensions::expo::card::default_width::default_expo_card_width;

/// Persisted left-panel ordering and collapsed-folder state.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SessionOrderPreferences {
    #[serde(default)]
    pub active_session_id: Option<String>,
    pub session_ids: Vec<String>,
    pub folder_paths: Vec<PathBuf>,
    pub collapsed_folder_paths: Vec<PathBuf>,
    #[serde(default = "default_expo_card_width")]
    pub expo_card_width: u16,
}

impl Default for SessionOrderPreferences {
    /// Builds default session preferences, including default Expo card width.
    fn default() -> Self {
        Self {
            active_session_id: None,
            session_ids: Vec::new(),
            folder_paths: Vec::new(),
            collapsed_folder_paths: Vec::new(),
            expo_card_width: default_expo_card_width(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SessionOrderPreferences;
    use crate::extensions::expo::card::default_width::default_expo_card_width;

    /// Verifies older saved preferences restore the default Expo card width.
    #[test]
    fn deserializes_missing_expo_card_width_as_default() {
        let preferences: SessionOrderPreferences = serde_json::from_str(
            r#"{"session_ids":[],"folder_paths":[],"collapsed_folder_paths":[]}"#,
        )
        .unwrap();

        assert_eq!(preferences.expo_card_width, default_expo_card_width());
    }

    /// Verifies saved preferences preserve the Expo card width setting.
    #[test]
    fn deserializes_saved_expo_card_width() {
        let preferences: SessionOrderPreferences = serde_json::from_str(
            r#"{"session_ids":[],"folder_paths":[],"collapsed_folder_paths":[],"expo_card_width":54}"#,
        )
        .unwrap();

        assert_eq!(preferences.expo_card_width, 54);
    }
}
