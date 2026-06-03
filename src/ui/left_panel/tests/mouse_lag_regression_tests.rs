use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use crossterm::event::{KeyModifiers, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::diagnostics::new_app_diagnostics::new_app_diagnostics;
use crate::app::input::handle_app_mouse::handle_app_mouse;
use crate::app::input::hotkeys::app_hotkey_registry::app_hotkey_registry;
use crate::app::state::app_state::AppState;
use crate::extensions::command_bar::data::command_bar_state::CommandBarState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::harness::conversation_picker::data::state::ConversationPickerState;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::StubHarness;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::resizable_grid::build_shell_layout::build_shell_layout;
use crate::ui::layout::resizable_grid::default_shell_split_percent::default_shell_split_percent;
use crate::ui::layout::resizable_grid::default_workspace_split_percent::default_workspace_split_percent;
use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache;
use crate::ui::menu_bar::state::app_menu_bar::app_menu_bar;

/// Reproduces the wheel-lag path and verifies wheel bursts do not rebuild rows per event.
#[test]
fn left_panel_wheel_burst_reuses_cached_visible_rows() -> anyhow::Result<()> {
    let mut app = lag_test_app(72, 12)?;
    let _ = app.visible_rows();
    let initial_rebuilds = app.visible_rows_rebuild_count();

    for _ in 0..80 {
        handle_app_mouse(&mut app, mouse_event(MouseEventKind::ScrollDown, 2));
    }

    assert_eq!(initial_rebuilds, 1);
    assert_eq!(app.visible_rows_rebuild_count(), initial_rebuilds);
    assert_eq!(app.session_scroll, 7);
    Ok(())
}

/// Verifies session clicks keep visible rows stable after cache reuse.
#[test]
fn session_click_keeps_cached_visible_rows_stable() -> anyhow::Result<()> {
    let mut app = lag_test_app(1, 12)?;
    let row_count = app.visible_row_count();

    handle_app_mouse(
        &mut app,
        mouse_event_at_column(MouseEventKind::Down(MouseButton::Left), 1, 1),
    );
    handle_app_mouse(
        &mut app,
        mouse_event_at_column(MouseEventKind::Up(MouseButton::Left), 1, 1),
    );

    assert!(app.collapsed_folders.is_empty());
    assert_eq!(row_count, 12);
    assert_eq!(app.visible_row_count(), 12);
    Ok(())
}

/// Verifies session drag still reorders rows and refreshes the cache when inputs change.
#[test]
fn session_drag_updates_cached_visible_rows() -> anyhow::Result<()> {
    let mut app = lag_test_app(1, 3)?;

    handle_app_mouse(
        &mut app,
        mouse_event(MouseEventKind::Down(MouseButton::Left), 2),
    );
    handle_app_mouse(
        &mut app,
        mouse_event(MouseEventKind::Drag(MouseButton::Left), 3),
    );
    handle_app_mouse(
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

/// Builds a app state with many visible rows for mouse-lag regression tests.
fn lag_test_app(folder_count: usize, sessions_per_folder: usize) -> anyhow::Result<AppState> {
    let layout = build_shell_layout(
        default_shell_split_percent(),
        default_workspace_split_percent(),
    );

    let plan_root =
        std::env::temp_dir().join(format!("ratsus-mouse-lag-plans-{}", uuid::Uuid::new_v4()));
    Ok(AppState {
        layout,
        layout_widget_state: ResizableGridWidgetState::default(),
        terminal_layout: ResizableGrid::new(TERMINAL_PANE_ID),
        terminal_layout_widget_state: ResizableGridWidgetState::default(),
        terminal_pane_sessions: BTreeMap::new(),
        terminal_pane_session_bundles: BTreeMap::new(),
        split_pane_session_groups: crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state(),
        suspended_multiplexer_state: None,
        terminal_pane_areas: BTreeMap::new(),
        terminal_pane_close_buttons: BTreeMap::new(),
        active_terminal_pane_id: TERMINAL_PANE_ID,
        toast_manager: ToastManager::new(),
        menu_bar: app_menu_bar(LeftPaneMode::Sessions),
        hotkey_registry: app_hotkey_registry(),
        command_bar: CommandBarState::new(),
        conversation_picker: ConversationPickerState::new(),
        delete_confirmation:
            crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        initial_sessions_receiver: None,
        diagnostics: new_app_diagnostics(),
        session_terminals: lag_test_sessions(folder_count, sessions_per_folder),
        visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
        closed_chat_session_ids: BTreeSet::new(),
        completed_unseen_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
        left_pane_mode: LeftPaneMode::Sessions,
        plan_list: PlanListState::with_root(plan_root)?,
        active_index: 0,
        focused_index: 0,
        session_scroll: 0,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        workspace_drag: None,
        workspace_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: (0..folder_count).map(lag_test_folder).collect(),
        selected_workspace_path: Some(lag_test_folder(0)),
        workspace_focused_session_ids: BTreeMap::new(),
        workspace_view_enabled: true,
        workspace_scroll: 0,
        focused_row: 0,
        pending_left_g: false,
        pending_left_scroll_redraw: false,
        suppress_left_focus_scroll: false,
        last_layout_area: Rect::new(0, 0, 120, 40),
        last_terminal_area: Rect::new(20, 0, 100, 40),
        active_terminal_area: Rect::new(20, 0, 100, 40),
        last_workspace_area: Rect::default(),
        last_workspace_list_area: Rect::default(),
        last_left_area: Rect::new(0, 0, 20, 20),
        last_session_list_area: Rect::new(1, 1, 18, 5),
        last_left_session_toggle_area: Rect::default(),
        last_left_plan_toggle_area: Rect::default(),
        last_left_file_toggle_area: Rect::default(),
        left_pane_visible: true,
        focused_pane: FocusedPane::Left,
        active_main_pane_tab: MainPaneTab::Chat,
        selected_expo_folder: None,
        expo_scroll: 0,
        expo_card_width: crate::extensions::expo::card::default_width::default_expo_card_width(
        ),
        expo_filter_query: String::new(),
        expo_filtering: false,
        expo_card_areas: Vec::new(),
        observation_previews: HashMap::new(),
        observation_cache_receiver: None,
        observation_watcher: None,
        session_watcher: None,
        session_refresh_receiver: None,
        last_main_pane_area: Rect::new(20, 0, 100, 40),
        file_system_tree_view: FileSystemTreeView::new()?,
        file_system_tree_expanded_paths: BTreeMap::new(),
        loader_tick: 0,
        chat_harness: Arc::new(StubHarness::new()),
    })
}

/// Builds dormant sessions spread evenly across the requested folders.
fn lag_test_sessions(folder_count: usize, sessions_per_folder: usize) -> Vec<SessionTerminal> {
    (0..folder_count)
        .flat_map(|folder| {
            (0..sessions_per_folder).map(move |session| {
                SessionTerminal::dormant(ChatSession::new(
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
