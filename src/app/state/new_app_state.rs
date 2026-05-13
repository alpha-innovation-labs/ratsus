use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::state::app_state::AppState;
use crate::extensions::expo::card::clamp_width::clamp_expo_card_width;
use crate::extensions::expo::observations::preview_requests::observation_preview_requests;
use crate::extensions::expo::observations::spawn_cache_worker::spawn_observation_cache_worker;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::harness::conversation_picker::data::state::ConversationPickerState;
use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::extensions::harness::sessions::refresh::spawn_refresh_worker::spawn_session_refresh_worker;
use crate::extensions::terminal::persistence::load_normal_terminal_sessions::load_normal_terminal_sessions;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::layout::resizable_grid::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::ui::left_panel::focus::session_visible_row_index::session_visible_row_index;
use crate::ui::left_panel::order::apply_session_id_order::apply_session_id_order;
use crate::ui::left_panel::order::load_preferences::load_session_order_preferences;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;
use crate::ui::left_panel::session::sort_by_creation_date::sort_sessions_by_creation_date;
use crate::ui::left_panel::session::visible_rows::visible_session_rows;
use crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache;
use crate::ui::menu_bar::state::app_menu_bar::app_menu_bar;

const SESSION_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

impl AppState {
    /// Builds the app state with an injected chat harness.
    pub fn new_with_harness(chat_harness: Arc<dyn ChatHarness>) -> Result<Self> {
        let mut layout = ResizableGrid::new(LEFT_PANE_ID);
        let _ = layout.split_pane_vertically(LEFT_PANE_ID);
        layout.set_split_percent(20);
        let preferences = load_session_order_preferences();
        let mut sessions = chat_harness.load_sessions()?;
        if chat_harness.load_normal_terminals_on_startup() {
            sessions.extend(load_normal_terminal_sessions());
        }
        let mut session_terminals = sessions
            .into_iter()
            .map(|session| SessionTerminal::dormant_with_harness(session, chat_harness.clone()))
            .collect::<Vec<_>>();
        sort_sessions_by_creation_date(&mut session_terminals);
        apply_session_id_order(&mut session_terminals, &preferences.session_ids);
        let folder_order = sync_folder_order(&preferences.folder_paths, &session_terminals);
        let active_index = preferences
            .active_session_id
            .as_deref()
            .and_then(|id| {
                session_terminals
                    .iter()
                    .position(|entry| entry.session.id == id)
            })
            .unwrap_or(0);
        let collapsed_folders = preferences.collapsed_folder_paths.into_iter().collect();
        let focused_row = session_visible_row_index(
            &visible_session_rows(
                &session_terminals,
                &collapsed_folders,
                &folder_order,
                Some(active_index),
            ),
            active_index,
        )
        .unwrap_or(0);
        let file_system_tree_view = FileSystemTreeView::new()?;
        let observation_cache_receiver = Some(spawn_observation_cache_worker(
            chat_harness.clone(),
            observation_preview_requests(&session_terminals),
        ));
        let observation_watcher = chat_harness.start_observation_watcher();
        let (terminal_pane_sessions, terminal_pane_session_bundles) =
            initial_terminal_panes(&session_terminals, active_index);
        Ok(Self {
            layout,
            layout_widget_state: ResizableGridWidgetState::default(),
            terminal_layout: ResizableGrid::new(TERMINAL_PANE_ID),
            terminal_layout_widget_state: ResizableGridWidgetState::default(),
            terminal_pane_sessions,
            terminal_pane_session_bundles,
            terminal_pane_areas: BTreeMap::new(),
            terminal_pane_close_buttons: BTreeMap::new(),
            active_terminal_pane_id: TERMINAL_PANE_ID,
            toast_manager: ToastManager::new(),
            menu_bar: app_menu_bar(MainPaneTab::Chat),
            conversation_picker: ConversationPickerState::new(),
            delete_confirmation: DeleteSessionConfirmationState::default(),
            delete_session_receiver: None,
            session_terminals,
            visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
            closed_chat_session_ids: BTreeSet::new(),
            selected_conversation_ids: BTreeSet::new(),
            active_index,
            focused_index: active_index,
            session_scroll: 0,
            pending_left_scroll_redraw: false,
            suppress_left_focus_scroll: false,
            session_drag: None,
            folder_drag: None,
            folder_drag_moved: false,
            collapsed_folders,
            folder_order,
            focused_row,
            pending_left_g: false,
            last_layout_area: Rect::default(),
            last_terminal_area: Rect::default(),
            active_terminal_area: Rect::default(),
            last_left_area: Rect::default(),
            last_session_list_area: Rect::default(),
            left_pane_visible: true,
            focused_pane: FocusedPane::Terminal,
            active_main_pane_tab: MainPaneTab::Chat,
            selected_expo_folder: None,
            expo_scroll: 0,
            expo_card_width: clamp_expo_card_width(preferences.expo_card_width),
            expo_filter_query: String::new(),
            expo_filtering: false,
            expo_card_areas: Vec::new(),
            observation_previews: HashMap::new(),
            observation_cache_receiver,
            observation_watcher,
            last_main_pane_area: Rect::default(),
            file_system_tree_view,
            loader_tick: 0,
            session_refresh_receiver: spawn_session_refresh_worker(
                chat_harness.clone(),
                SESSION_REFRESH_INTERVAL,
            ),
            chat_harness,
        })
    }
}

/// Builds the initial terminal-pane session maps.
fn initial_terminal_panes(
    session_terminals: &[SessionTerminal],
    active_index: usize,
) -> (BTreeMap<u32, String>, BTreeMap<u32, Vec<String>>) {
    let mut terminal_pane_sessions = BTreeMap::new();
    let mut terminal_pane_session_bundles = BTreeMap::new();
    if let Some(session) = session_terminals.get(active_index) {
        terminal_pane_sessions.insert(TERMINAL_PANE_ID, session.session.id.clone());
        terminal_pane_session_bundles.insert(TERMINAL_PANE_ID, vec![session.session.id.clone()]);
    }
    (terminal_pane_sessions, terminal_pane_session_bundles)
}
