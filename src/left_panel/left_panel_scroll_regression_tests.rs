use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::mpsc;

use crossterm::event::{KeyModifiers, MouseEventKind};
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::handle_nexus_demo_mouse::handle_nexus_demo_mouse;
use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_state::ConversationPickerState;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::pane_ids::LEFT_PANE_ID;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::nexus_sessions::session_info::NexusSession;
use crate::nexus_sessions::spawn_session_refresh_worker::SessionRefreshResult;
use crate::terminal::session_terminal::SessionTerminal;

/// Reproduces the render-time clamp that makes wheel scrolling appear delayed.
#[test]
#[ignore = "reproduces current left-pane scroll lag caused by render-time focus clamping"]
fn render_focus_clamp_overrides_manual_left_panel_scroll() -> anyhow::Result<()> {
    let mut app = scroll_test_app()?;
    app.focused_row = 1;

    send_left_panel_scroll_down_events(&mut app, 6);
    let manual_scroll = app.session_scroll;

    app.keep_focused_row_visible();

    assert_eq!(app.session_scroll, manual_scroll);
    Ok(())
}

/// Verifies synthetic wheel events update the left-panel viewport deterministically.
#[test]
fn synthetic_wheel_events_scroll_left_panel_once_per_event() -> anyhow::Result<()> {
    let mut app = scroll_test_app()?;

    send_left_panel_scroll_down_events(&mut app, 3);

    assert_eq!(app.session_scroll, 3);
    Ok(())
}

/// Sends a fixed number of synthetic wheel-down events inside the left panel.
fn send_left_panel_scroll_down_events(app: &mut NexusDemo, count: usize) {
    for _ in 0..count {
        handle_nexus_demo_mouse(app, left_panel_scroll_down_event());
    }
}

/// Builds a synthetic left-panel wheel-down event.
fn left_panel_scroll_down_event() -> ratkit::MouseEvent {
    ratkit::MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 2,
        row: 2,
        modifiers: KeyModifiers::empty(),
    }
}

/// Builds a Nexus demo state with enough left-panel rows to scroll.
fn scroll_test_app() -> anyhow::Result<NexusDemo> {
    let (_sender, receiver) = mpsc::channel::<SessionRefreshResult>();
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);

    Ok(NexusDemo {
        layout,
        layout_widget_state: ResizableGridWidgetState::default(),
        toast_manager: ToastManager::new(),
        conversation_picker: ConversationPickerState::new(),
        session_terminals: scroll_test_sessions(),
        closed_chat_session_ids: BTreeSet::new(),
        active_index: 0,
        focused_index: 0,
        session_scroll: 0,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: vec![scroll_test_folder()],
        focused_row: 1,
        pending_left_g: false,
        last_layout_area: Rect::new(0, 0, 120, 40),
        last_terminal_area: Rect::new(20, 0, 100, 40),
        active_terminal_area: Rect::new(20, 0, 100, 40),
        last_left_area: Rect::new(0, 0, 20, 20),
        last_session_list_area: Rect::new(1, 1, 18, 5),
        left_pane_visible: true,
        focused_pane: FocusedPane::Left,
        active_main_pane_tab: MainPaneTab::Chat,
        last_main_pane_area: Rect::new(20, 0, 100, 40),
        file_system_tree_view: FileSystemTreeView::new()?,
        loader_tick: 0,
        session_refresh_receiver: receiver,
    })
}

/// Builds sessions that all belong to the same folder.
fn scroll_test_sessions() -> Vec<SessionTerminal> {
    (0..12)
        .map(|index| {
            SessionTerminal::dormant(NexusSession::new(
                format!("2026-05-12T10:{index:02}:00Z"),
                format!("Chat {index}"),
                format!("session-{index}"),
                scroll_test_folder(),
            ))
        })
        .collect()
}

/// Returns the folder used by the left-panel scroll fixture.
fn scroll_test_folder() -> PathBuf {
    PathBuf::from("/tmp/scroll-test")
}
