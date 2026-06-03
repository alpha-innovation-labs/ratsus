use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::diagnostics::new_app_diagnostics::new_app_diagnostics;
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
use crate::ui::left_panel::focus::focus_row::focus_left_panel_row;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::render::render_scrollbar::render_left_panel_scrollbar;
use crate::ui::left_panel::render::session_lines::session_lines;
use crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache;
use crate::ui::menu_bar::state::app_menu_bar::app_menu_bar;

/// Verifies the left pane renders Ratkit's scrollbar extension when rows overflow.
#[test]
fn renders_scrollbar_when_left_panel_overflows() -> anyhow::Result<()> {
    let app = scrollbar_test_app(12, 0)?;
    let mut terminal = Terminal::new(TestBackend::new(20, 10))?;

    terminal.draw(|frame| render_left_panel_scrollbar(&app, frame))?;

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer.cell((10, 1)).unwrap().symbol(), "█");
    assert_eq!(buffer.cell((10, 3)).unwrap().symbol(), "░");
    Ok(())
}

/// Verifies session ages are rendered before the reserved scrollbar column.
#[test]
fn keeps_session_age_visible_next_to_scrollbar() -> anyhow::Result<()> {
    let app = scrollbar_test_app(12, 0)?;
    let mut terminal = Terminal::new(TestBackend::new(20, 10))?;

    terminal.draw(|frame| {
        frame.render_widget(
            Paragraph::new(session_lines(&app)),
            app.last_session_list_area,
        );
        render_left_panel_scrollbar(&app, frame);
    })?;

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer.cell((8, 2)).unwrap().symbol(), "m");
    assert_eq!(buffer.cell((9, 2)).unwrap().symbol(), " ");
    assert_eq!(buffer.cell((10, 2)).unwrap().symbol(), "█");
    Ok(())
}

/// Verifies the Ratkit scrollbar thumb follows the left-panel scroll offset.
#[test]
fn moves_scrollbar_thumb_with_left_panel_offset() -> anyhow::Result<()> {
    let app = scrollbar_test_app(12, 7)?;
    let mut terminal = Terminal::new(TestBackend::new(20, 10))?;

    terminal.draw(|frame| render_left_panel_scrollbar(&app, frame))?;

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer.cell((10, 1)).unwrap().symbol(), "░");
    assert_eq!(buffer.cell((10, 4)).unwrap().symbol(), "█");
    Ok(())
}

/// Verifies the left pane omits the scrollbar when every row fits.
#[test]
fn omits_scrollbar_when_left_panel_content_fits() -> anyhow::Result<()> {
    let app = scrollbar_test_app(3, 0)?;
    let mut terminal = Terminal::new(TestBackend::new(20, 10))?;

    terminal.draw(|frame| render_left_panel_scrollbar(&app, frame))?;

    assert_eq!(
        terminal.backend().buffer().cell((10, 1)).unwrap().symbol(),
        " "
    );
    Ok(())
}

/// Verifies that selecting a session activates that conversation.
#[test]
fn selecting_session_activates_conversation() -> anyhow::Result<()> {
    let mut app = scrollbar_test_app(3, 0)?;

    focus_left_panel_row(&mut app, 0);

    assert_eq!(app.active_main_pane_tab, MainPaneTab::Chat);
    assert_eq!(app.active_index, 0);
    Ok(())
}

/// Builds a app state with one left-panel folder and fixed viewport geometry.
fn scrollbar_test_app(session_count: usize, session_scroll: usize) -> anyhow::Result<AppState> {
    let layout = build_shell_layout(
        default_shell_split_percent(),
        default_workspace_split_percent(),
    );

    let plan_root =
        std::env::temp_dir().join(format!("ratsus-scrollbar-plans-{}", uuid::Uuid::new_v4()));
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
        delete_confirmation: DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        initial_sessions_receiver: None,
        diagnostics: new_app_diagnostics(),
        session_terminals: scrollbar_test_sessions(session_count),
        visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
        closed_chat_session_ids: BTreeSet::new(),
        completed_unseen_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
        left_pane_mode: LeftPaneMode::Sessions,
        plan_list: PlanListState::with_root(plan_root)?,
        active_index: 0,
        focused_index: 0,
        session_scroll,
        pending_left_scroll_redraw: false,
        suppress_left_focus_scroll: false,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        workspace_drag: None,
        workspace_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: vec![scrollbar_test_folder()],
        selected_workspace_path: Some(scrollbar_test_folder()),
        workspace_focused_session_ids: BTreeMap::new(),
        workspace_view_enabled: true,
        workspace_scroll: 0,
        focused_row: 1,
        pending_left_g: false,
        last_layout_area: Rect::new(0, 0, 120, 40),
        last_terminal_area: Rect::new(20, 0, 100, 40),
        active_terminal_area: Rect::new(20, 0, 100, 40),
        last_workspace_area: Rect::default(),
        last_workspace_list_area: Rect::default(),
        last_left_area: Rect::new(0, 0, 20, 20),
        last_session_list_area: Rect::new(1, 1, 10, 5),
        last_left_session_toggle_area: Rect::default(),
        last_left_plan_toggle_area: Rect::default(),
        last_left_file_toggle_area: Rect::default(),
        left_pane_visible: true,
        focused_pane: FocusedPane::Left,
        active_main_pane_tab: MainPaneTab::Chat,
        selected_expo_folder: None,
        expo_scroll: 0,
        expo_card_width: crate::extensions::expo::card::default_width::default_expo_card_width(),
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

/// Builds dormant sessions for the scrollbar render fixture.
fn scrollbar_test_sessions(session_count: usize) -> Vec<SessionTerminal> {
    (0..session_count)
        .map(|index| {
            SessionTerminal::dormant(ChatSession::new(
                "now",
                format!("Session {index}"),
                format!("session-{index}"),
                scrollbar_test_folder(),
            ))
        })
        .collect()
}

/// Returns the folder path used by the scrollbar render fixture.
fn scrollbar_test_folder() -> PathBuf {
    PathBuf::from("/tmp/scrollbar-test")
}
