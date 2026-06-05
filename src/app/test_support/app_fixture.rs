use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::diagnostics::new_app_diagnostics::new_app_diagnostics;
use crate::app::input::hotkeys::app_hotkey_registry::app_hotkey_registry;
use crate::app::state::app_state::AppState;
use crate::extensions::command_bar::data::command_bar_state::CommandBarState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::history_modal::data::state::HistoryModalState;
use crate::extensions::harness::stub::StubHarness;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::resizable_grid::build_shell_layout::build_shell_layout;
use crate::ui::layout::resizable_grid::default_shell_split_percent::default_shell_split_percent;
use crate::ui::layout::resizable_grid::default_workspace_split_percent::default_workspace_split_percent;
use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache;
use crate::ui::menu_bar::state::app_menu_bar::app_menu_bar;

/// Builds a minimal AppState fixture without spawning terminal processes.
pub fn app_fixture(session_terminals: Vec<SessionTerminal>) -> anyhow::Result<AppState> {
    let layout = build_shell_layout(
        default_shell_split_percent(),
        default_workspace_split_percent(),
    );
    let plan_root =
        std::env::temp_dir().join(format!("ratsus-app-fixture-plans-{}", uuid::Uuid::new_v4()));
    Ok(AppState {
        layout,
        layout_widget_state: ResizableGridWidgetState::default(),
        terminal_layout: ResizableGrid::new(TERMINAL_PANE_ID),
        terminal_layout_widget_state: ResizableGridWidgetState::default(),
        terminal_pane_sessions: BTreeMap::new(),
        terminal_pane_session_bundles: BTreeMap::new(),
        split_pane_session_groups: default_split_pane_session_group_state(),
        suspended_multiplexer_state: None,
        terminal_pane_areas: BTreeMap::new(),
        terminal_pane_close_buttons: BTreeMap::new(),
        active_terminal_pane_id: TERMINAL_PANE_ID,
        toast_manager: ToastManager::new(),
        menu_bar: app_menu_bar(LeftPaneMode::Sessions),
        hotkey_registry: app_hotkey_registry(),
        command_bar: CommandBarState::new(),
        history_modal: HistoryModalState::new(),
        delete_confirmation: DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        initial_sessions_receiver: None,
        diagnostics: new_app_diagnostics(),
        session_terminals,
        visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
        closed_chat_session_ids: BTreeSet::new(),
        completed_unseen_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
        left_pane_mode: LeftPaneMode::Sessions,
        plan_list: PlanListState::with_root(plan_root)?,
        active_index: 0,
        focused_index: 0,
        session_scroll: 0,
        pending_left_scroll_redraw: false,
        suppress_left_focus_scroll: false,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        workspace_drag: None,
        workspace_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: vec!["/tmp/project".into()],
        selected_workspace_path: Some("/tmp/project".into()),
        workspace_focused_session_ids: BTreeMap::new(),
        workspace_view_enabled: true,
        workspace_scroll: 0,
        focused_row: 1,
        pending_left_g: false,
        last_layout_area: Rect::default(),
        last_terminal_area: Rect::default(),
        active_terminal_area: Rect::default(),
        last_workspace_area: Rect::default(),
        last_workspace_list_area: Rect::default(),
        last_left_area: Rect::default(),
        last_session_list_area: Rect::default(),
        last_left_session_toggle_area: Rect::default(),
        last_left_plan_toggle_area: Rect::default(),
        last_left_file_toggle_area: Rect::default(),
        left_pane_visible: true,
        focused_pane: FocusedPane::Terminal,
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
        last_main_pane_area: Rect::default(),
        file_system_tree_view: FileSystemTreeView::new()?,
        file_system_tree_expanded_paths: BTreeMap::new(),
        loader_tick: 0,
        chat_harness: Arc::new(StubHarness::new()),
    })
}
