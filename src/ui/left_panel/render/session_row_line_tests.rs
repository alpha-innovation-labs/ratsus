use std::path::Path;

use ratatui::style::Color;

use crate::ui::left_panel::folder::blue_color::folder_blue_color;
use crate::ui::left_panel::render::session_row_line::{
    folder_row_line, session_row_line, split_group_row_line, ChatSessionRowLineConfig,
    FolderRowLineConfig,
};

/// Verifies the active Expo folder uses the shared folder blue without background color.
#[test]
fn active_expo_folder_uses_folder_blue_foreground_without_background() {
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

    assert_eq!(line.style.fg, Some(folder_blue_color()));
    assert_eq!(line.style.bg, None);
}

/// Verifies selected inactive folders use the shared folder blue without background color.
#[test]
fn inactive_selected_folder_uses_folder_blue_foreground_without_background() {
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

    assert_eq!(line.style.fg, Some(folder_blue_color()));
    assert_eq!(line.style.bg, None);
}

/// Verifies running folder rows show only the working animation marker.
#[test]
fn running_folder_hides_folder_icon() {
    let line = folder_row_line(FolderRowLineConfig {
        path: Path::new("/tmp/project"),
        current_session_count: 1,
        total_session_count: 1,
        is_collapsed: false,
        has_running_session: true,
        loader_tick: 0,
        width: 40,
        show_full_path: false,
        is_active_expo_folder: false,
        is_selected: false,
        is_dragging: false,
    });

    let text = line_text(&line);
    assert!(!text.contains('\u{f114}'));
    assert!(!text.contains('\u{f115}'));
}

/// Verifies toggled session rows use red foreground for all text.
#[test]
fn toggled_session_uses_red_text_color() {
    let line = session_row_line(ChatSessionRowLineConfig {
        title: "api",
        age: "1m",
        icon: "󰆍",
        width: 24,
        is_selected: true,
        is_active: false,
        is_running: false,
        is_completed_unseen: false,
        is_dragging: false,
        is_toggled: true,
        bundle_marker: None,
        tree_prefix: None,
    });

    assert!(line
        .spans
        .iter()
        .any(|span| span.style.fg == Some(Color::Red)));
    assert_eq!(
        line.spans.last().and_then(|span| span.style.fg),
        Some(Color::Red)
    );
}

/// Verifies keyboard-selected conversation rows are red over other row states.
#[test]
fn selected_session_uses_red_text_color() {
    let line = session_row_line(ChatSessionRowLineConfig {
        title: "api",
        age: "1m",
        icon: "󰆍",
        width: 24,
        is_selected: true,
        is_active: true,
        is_running: false,
        is_completed_unseen: false,
        is_dragging: false,
        is_toggled: false,
        bundle_marker: None,
        tree_prefix: None,
    });

    assert!(line
        .spans
        .iter()
        .any(|span| span.style.fg == Some(Color::Red)));
    assert_eq!(
        line.spans.last().and_then(|span| span.style.fg),
        Some(Color::Red)
    );
}

/// Verifies unseen completed sessions render teal until selected.
#[test]
fn completed_unseen_session_uses_teal_text_color() {
    let line = session_row_line(ChatSessionRowLineConfig {
        title: "api",
        age: "1m",
        icon: "󰆍",
        width: 24,
        is_selected: false,
        is_active: false,
        is_running: false,
        is_completed_unseen: true,
        is_dragging: false,
        is_toggled: false,
        bundle_marker: None,
        tree_prefix: None,
    });

    assert!(line
        .spans
        .iter()
        .any(|span| span.style.fg == Some(Color::Cyan)));
    assert_eq!(
        line.spans.last().and_then(|span| span.style.fg),
        Some(Color::Cyan)
    );
}

/// Verifies active non-running sessions keep the static icon muted while the title is white.
#[test]
fn active_static_session_colors_only_title_white() {
    let line = session_row_line(ChatSessionRowLineConfig {
        title: "api",
        age: "1m",
        icon: "󰆍",
        width: 24,
        is_selected: false,
        is_active: true,
        is_running: false,
        is_completed_unseen: false,
        is_dragging: false,
        is_toggled: false,
        bundle_marker: None,
        tree_prefix: None,
    });

    let icon_span = line.spans.iter().find(|span| span.content.as_ref() == "󰆍");
    let title_span = line
        .spans
        .iter()
        .find(|span| span.content.as_ref() == "api");
    assert_ne!(icon_span.and_then(|span| span.style.fg), Some(Color::White));
    assert_eq!(
        title_span.and_then(|span| span.style.fg),
        Some(Color::White)
    );
}

/// Verifies active running sessions color the working animation and title white.
#[test]
fn active_running_session_colors_animation_and_title_white() {
    let line = session_row_line(ChatSessionRowLineConfig {
        title: "api",
        age: "1m",
        icon: "⠋",
        width: 24,
        is_selected: false,
        is_active: true,
        is_running: true,
        is_completed_unseen: false,
        is_dragging: false,
        is_toggled: false,
        bundle_marker: None,
        tree_prefix: None,
    });

    let icon_span = line.spans.iter().find(|span| span.content.as_ref() == "⠋");
    let title_span = line
        .spans
        .iter()
        .find(|span| span.content.as_ref() == "api");
    assert_eq!(icon_span.and_then(|span| span.style.fg), Some(Color::White));
    assert_eq!(
        title_span.and_then(|span| span.style.fg),
        Some(Color::White)
    );
}

/// Verifies split group parent rows align with regular session rows.
#[test]
fn split_group_parent_has_session_left_padding() {
    let line = split_group_row_line("Group 1", 2, 24, false, false);

    assert!(line_text(&line).starts_with("  Group 1"));
}

/// Verifies split group child connector rows keep the same left padding.
#[test]
fn split_group_child_connector_has_session_left_padding() {
    let line = session_row_line(ChatSessionRowLineConfig {
        title: "api",
        age: "1m",
        icon: "󰆍",
        width: 24,
        is_selected: false,
        is_active: false,
        is_running: false,
        is_completed_unseen: false,
        is_dragging: false,
        is_toggled: false,
        bundle_marker: None,
        tree_prefix: Some("  ├─ "),
    });

    assert!(line_text(&line).starts_with("  ├─ "));
}

/// Returns the concatenated visible text for a rendered line.
fn line_text(line: &ratatui::text::Line<'_>) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect()
}
