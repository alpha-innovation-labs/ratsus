use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::mpsc;

use crossterm::event::{KeyModifiers, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::handle_nexus_demo_mouse::handle_nexus_demo_mouse;
use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_state::ConversationPickerState;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::left_panel::visible_session_rows_cache::VisibleSessionRowsCache;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::nexus_menu_bar::nexus_menu_bar;
use crate::nexus_sessions::session_info::NexusSession;
use crate::nexus_sessions::spawn_session_refresh_worker::SessionRefreshResult;
use crate::terminal::session_terminal::SessionTerminal;

/// Reproduces the wheel-lag path and verifies wheel bursts do not rebuild rows per event.
#[test]
fn left_panel_wheel_burst_reuses_cached_visible_rows() -> anyhow::Result<()> {
    let mut app = lag_test_app(72, 12)?;
    let _ = app.visible_rows();
    let initial_rebuilds = app.visible_rows_rebuild_count();

    for _ in 0..80 {
        handle_nexus_demo_mouse(&mut app, mouse_event(MouseEventKind::ScrollDown, 2));
    }

    assert_eq!(initial_rebuilds, 1);
    assert_eq!(app.visible_rows_rebuild_count(), initial_rebuilds);
    assert_eq!(app.session_scroll, 80);
    Ok(())
}

/// Verifies folder click/toggle still changes visible rows after cache reuse.
#[test]
fn folder_toggle_updates_cached_visible_rows() -> anyhow::Result<()> {
    let mut app = lag_test_app(1, 12)?;
    let expanded_count = app.visible_row_count();

    handle_nexus_demo_mouse(
        &mut app,
        mouse_event_at_column(MouseEventKind::Down(MouseButton::Left), 1, 1),
    );
    handle_nexus_demo_mouse(
        &mut app,
        mouse_event_at_column(MouseEventKind::Up(MouseButton::Left), 1, 1),
    );

    assert!(app.collapsed_folders.contains(&lag_test_folder(0)));
    assert_eq!(expanded_count, 12);
    assert_eq!(app.visible_row_count(), 1);
    Ok(())
}

/// Verifies session drag still reorders rows and refreshes the cache when inputs change.
#[test]
fn session_drag_updates_cached_visible_rows() -> anyhow::Result<()> {
    let mut app = lag_test_app(1, 3)?;

    handle_nexus_demo_mouse(
        &mut app,
        mouse_event(MouseEventKind::Down(MouseButton::Left), 2),
    );
    handle_nexus_demo_mouse(
        &mut app,
        mouse_event(MouseEventKind::Drag(MouseButton::Left), 3),
    );
    handle_nexus_demo_mouse(
        &mut app,
        mouse_event(MouseEventKind::Up(MouseButton::Left), 3),
    );

    assert_eq!(app.session_terminals[1].session.id, "session-0-0");
    assert!(app.session_drag.is_none());
    assert!(app.folder_drag.is_none());
    Ok(())
}

/// Builds a synthetic mouse event at a fixed left-panel column.
fn mouse_event(kind: MouseEventKind, row: u16) -> ratkit::MouseEvent {
    mouse_event_at_column(kind, row, 2)
}

/// Builds a synthetic mouse event at the requested left-panel column.
fn mouse_event_at_column(kind: MouseEventKind, row: u16, column: u16) -> ratkit::MouseEvent {
    ratkit::MouseEvent {
        kind,
        column,
        row,
        modifiers: KeyModifiers::empty(),
    }
}

/// Builds a Nexus demo state with many visible rows for mouse-lag regression tests.
fn lag_test_app(folder_count: usize, sessions_per_folder: usize) -> anyhow::Result<NexusDemo> {
    let (_sender, receiver) = mpsc::channel::<SessionRefreshResult>();
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);

    Ok(NexusDemo {
        layout,
        layout_widget_state: ResizableGridWidgetState::default(),
        terminal_layout: ResizableGrid::new(TERMINAL_PANE_ID),
        terminal_layout_widget_state: ResizableGridWidgetState::default(),
        terminal_pane_sessions: BTreeMap::new(),
        terminal_pane_session_bundles: BTreeMap::new(),
        terminal_pane_areas: BTreeMap::new(),
        terminal_pane_close_buttons: BTreeMap::new(),
        active_terminal_pane_id: TERMINAL_PANE_ID,
        toast_manager: ToastManager::new(),
        menu_bar: nexus_menu_bar(MainPaneTab::Chat),
        conversation_picker: ConversationPickerState::new(),
        delete_confirmation:
            crate::app::delete_session_confirmation_state::DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        session_terminals: lag_test_sessions(folder_count, sessions_per_folder),
        visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
        closed_chat_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
        active_index: 0,
        focused_index: 0,
        session_scroll: 0,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: (0..folder_count).map(lag_test_folder).collect(),
        focused_row: 1,
        pending_left_g: false,
        pending_left_scroll_redraw: false,
        suppress_left_focus_scroll: false,
        last_layout_area: Rect::new(0, 0, 120, 40),
        last_terminal_area: Rect::new(20, 0, 100, 40),
        active_terminal_area: Rect::new(20, 0, 100, 40),
        last_left_area: Rect::new(0, 0, 20, 20),
        last_session_list_area: Rect::new(1, 1, 18, 5),
        left_pane_visible: true,
        focused_pane: FocusedPane::Left,
        active_main_pane_tab: MainPaneTab::Chat,
        selected_expo_folder: None,
        expo_scroll: 0,
        expo_card_width: crate::expo::default_expo_card_width::default_expo_card_width(),
        expo_filter_query: String::new(),
        expo_filtering: false,
        expo_card_areas: Vec::new(),
        observation_previews: HashMap::new(),
        observation_cache_receiver: None,
        observation_watcher: None,
        last_main_pane_area: Rect::new(20, 0, 100, 40),
        file_system_tree_view: FileSystemTreeView::new()?,
        loader_tick: 0,
        session_refresh_receiver: receiver,
    })
}

/// Builds dormant sessions spread evenly across the requested folders.
fn lag_test_sessions(folder_count: usize, sessions_per_folder: usize) -> Vec<SessionTerminal> {
    (0..folder_count)
        .flat_map(|folder| {
            (0..sessions_per_folder).map(move |session| {
                SessionTerminal::dormant(NexusSession::new(
                    "2026-05-12T10:00:00Z",
                    format!("Session {folder}-{session}"),
                    format!("session-{folder}-{session}"),
                    lag_test_folder(folder),
                ))
            })
        })
        .collect()
}

/// Returns the synthetic folder path for one folder index.
fn lag_test_folder(index: usize) -> PathBuf {
    PathBuf::from(format!("/tmp/lag-test-{index}"))
}
