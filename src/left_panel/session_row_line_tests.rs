use std::path::Path;

use ratatui::style::Color;

use crate::left_panel::session_row_line::{folder_row_line, FolderRowLineConfig};

/// Verifies the active Expo folder uses foreground only, without background color.
#[test]
fn active_expo_folder_uses_red_foreground_without_background() {
    let line = folder_row_line(FolderRowLineConfig {
        path: Path::new("/tmp/project"),
        current_session_count: 1,
        total_session_count: 1,
        is_collapsed: false,
        has_running_session: false,
        loader_tick: 0,
        width: 40,
        show_full_path: false,
        is_active_expo_folder: true,
        is_selected: false,
        is_dragging: false,
    });

    assert_eq!(line.style.fg, Some(Color::Red));
    assert_eq!(line.style.bg, None);
}

/// Verifies selected folders use foreground only, without background color.
#[test]
fn selected_folder_uses_cyan_foreground_without_background() {
    let line = folder_row_line(FolderRowLineConfig {
        path: Path::new("/tmp/project"),
        current_session_count: 1,
        total_session_count: 1,
        is_collapsed: false,
        has_running_session: false,
        loader_tick: 0,
        width: 40,
        show_full_path: false,
        is_active_expo_folder: false,
        is_selected: true,
        is_dragging: false,
    });

    assert_eq!(line.style.fg, Some(Color::Cyan));
    assert_eq!(line.style.bg, None);
}
