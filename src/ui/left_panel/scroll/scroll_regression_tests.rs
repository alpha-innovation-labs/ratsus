use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use crossterm::event::{KeyModifiers, MouseEventKind};
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;
use ratkit::CoordinatorAction;

use crate::app::events::handle_tick_event::handle_tick_event;
use crate::app::input::handle_app_mouse::handle_app_mouse;
use crate::app::input::hotkeys::app_hotkey_registry::app_hotkey_registry;
use crate::app::state::app_state::AppState;
use crate::core::rendering::screen::render_app::render_app;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::harness::conversation_picker::data::state::ConversationPickerState;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::StubHarness;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::resizable_grid::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::ui::menu_bar::state::app_menu_bar::app_menu_bar;

/// Verifies focus clamping stays suppressed after manual left-panel wheel scrolling.
#[test]
fn focus_clamp_stays_suppressed_after_manual_left_panel_scroll() -> anyhow::Result<()> {
    let mut app = scroll_test_app()?;
    app.focused_row = 1;

    send_left_panel_scroll_down_events(&mut app, 6);
    let manual_scroll = app.session_scroll;

    app.keep_focused_row_visible();

    assert_eq!(app.session_scroll, manual_scroll);
    Ok(())
}

/// Verifies rendering does not clamp away manual left-panel wheel scrolling.
#[test]
fn render_does_not_override_manual_left_panel_scroll() -> anyhow::Result<()> {
    let mut app = scroll_test_app()?;
    app.focused_row = 1;

    send_left_panel_scroll_down_events(&mut app, 6);
    let manual_scroll = app.session_scroll;

    let backend = ratatui::backend::TestBackend::new(120, 40);
    let mut terminal = ratatui::Terminal::new(backend)?;
    terminal.draw(|frame| render_app(&mut app, frame))?;

    assert_eq!(app.session_scroll, manual_scroll);
    Ok(())
}

/// Verifies wheel events update state without forcing one render per event.
#[test]
fn wheel_events_are_coalesced_until_tick_redraw() -> anyhow::Result<()> {
    let mut app = scroll_test_app()?;

    let action = handle_app_mouse(&mut app, left_panel_scroll_down_event());

    assert_eq!(action, CoordinatorAction::Continue);
    assert!(app.pending_left_scroll_redraw);
    assert_eq!(handle_tick_event(&mut app, 1), CoordinatorAction::Redraw);
    assert!(!app.pending_left_scroll_redraw);
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
fn send_left_panel_scroll_down_events(app: &mut AppState, count: usize) {
    for _ in 0..count {
        handle_app_mouse(app, left_panel_scroll_down_event());
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

/// Builds a app state with enough left-panel rows to scroll.
fn scroll_test_app() -> anyhow::Result<AppState> {
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);

    Ok(AppState {
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
        menu_bar: app_menu_bar(MainPaneTab::Chat),
        hotkey_registry: app_hotkey_registry(),
        conversation_picker: ConversationPickerState::new(),
        delete_confirmation:
            crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        session_terminals: scroll_test_sessions(),
        visible_rows_cache: std::cell::RefCell::new(
            crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache::new(),
        ),
        closed_chat_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
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
        expo_card_width: crate::extensions::expo::card::default_width::default_expo_card_width(
        ),
        expo_filter_query: String::new(),
        expo_filtering: false,
        expo_card_areas: Vec::new(),
        observation_previews: HashMap::new(),
        observation_cache_receiver: None,
        observation_watcher: None,
        session_watcher: None,
        last_main_pane_area: Rect::new(20, 0, 100, 40),
        file_system_tree_view: FileSystemTreeView::new()?,
        loader_tick: 0,
        chat_harness: Arc::new(StubHarness::new()),
    })
}

/// Builds sessions that all belong to the same folder.
fn scroll_test_sessions() -> Vec<SessionTerminal> {
    (0..12)
        .map(|index| {
            SessionTerminal::dormant(ChatSession::new(
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
