use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::sync::Arc;

use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Terminal;
use ratkit::primitives::resizable_grid::{PaneId, ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::diagnostics::new_app_diagnostics::new_app_diagnostics;
use crate::app::input::hotkeys::app_hotkey_registry::app_hotkey_registry;
use crate::app::state::app_state::AppState;
use crate::core::rendering::screen::render_app::render_app;
use crate::extensions::command_bar::data::command_bar_state::CommandBarState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::history_modal::data::state::ConversationPickerState;
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

/// Snapshots right and bottom split rendering without a real terminal process.
#[test]
fn snapshots_right_and_bottom_split_layout() -> anyhow::Result<()> {
    let mut app = snapshot_app(vec![
        session("Session A", "a"),
        session("Session B", "b"),
        session("Session C", "c"),
    ])?;
    let right_pane = app
        .terminal_layout
        .split_pane_vertically(TERMINAL_PANE_ID)
        .unwrap();
    let bottom_pane = app
        .terminal_layout
        .split_pane_horizontally(right_pane)
        .unwrap();
    app.terminal_pane_sessions = BTreeMap::from([
        (TERMINAL_PANE_ID, "a".to_string()),
        (right_pane, "b".to_string()),
        (bottom_pane, "c".to_string()),
    ]);
    app.terminal_pane_session_bundles = BTreeMap::from([
        (TERMINAL_PANE_ID, vec!["a".to_string()]),
        (right_pane, vec!["b".to_string()]),
        (bottom_pane, vec!["c".to_string()]),
    ]);
    app.active_terminal_pane_id = bottom_pane;
    app.active_index = 2;
    app.focused_index = 2;
    app.terminal_pane_areas = terminal_split_content_areas(right_pane, bottom_pane);

    let output = render_snapshot(&mut app, Rect::new(16, 3, 64, 16))?;

    assert!(!output.contains("No session assigned"));
    assert_snapshot!(output, @r###"
ons ───────╮╭Chat──────────────────────────────────────────────╮
today ─────││┌Session A─────────── x ┐┌Session B─────────── x ┐│
ion A    0m│││Starting chat session… ││Starting chat session… ││
ion B    0m│││                       ││                       ││
ion C    0m│││                       ││                       ││
           │││                       ││                       ││
           │││                       ││                       ││
           │││                       │└───────────────────────┘│
           │││                       │┌Session C─────────── x ┐│
           │││                       ││Starting chat session… ││
           │││                       ││                       ││
           │││                       ││                       ││
           │││                       ││                       ││
           │││                       ││                       ││
           │││                       ││                       ││
ove  h/l fo││└───────────────────────┘└───────────────────────┘│
"###);
    Ok(())
}

/// Snapshots bundled conversations in the left pane with branch-style markers.
#[test]
fn snapshots_bundled_session_left_panel_markers() -> anyhow::Result<()> {
    let mut app = snapshot_app(vec![session("Alpha", "a"), session("Beta", "b")])?;
    app.terminal_pane_sessions = BTreeMap::from([(TERMINAL_PANE_ID, "b".to_string())]);
    app.terminal_pane_session_bundles =
        BTreeMap::from([(TERMINAL_PANE_ID, vec!["a".to_string(), "b".to_string()])]);
    app.active_index = 1;
    app.focused_index = 1;
    app.terminal_pane_areas = BTreeMap::from([(TERMINAL_PANE_ID, Rect::new(17, 4, 62, 15))]);

    let output = render_snapshot(&mut app, Rect::new(0, 3, 16, 6))?;

    assert!(output.contains("󰀘 Alph"));
    assert!(output.contains("󰀘 Beta"));
    assert_snapshot!(output, @r###"
╭ Worksp╮╭ Sessi
│╭─── 2╮││───── 
││proj…│││󰀘 Alph
│╰─────╯││󰀘 Beta
│       ││      
│       ││
"###);
    Ok(())
}

/// Snapshots Files mode with the file tree on the left and code preview on the right.
#[test]
fn snapshots_files_tab_tree_and_preview() -> anyhow::Result<()> {
    let root = std::env::temp_dir().join(format!("ratsus-files-tab-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&root)?;
    fs::write(root.join("README.md"), "# Project Notes\n\nPreview body")?;

    let mut app = snapshot_app(vec![session("Session A", "a")])?;
    app.active_main_pane_tab = MainPaneTab::Chat;
    app.left_pane_mode = LeftPaneMode::Files;
    app.folder_order = vec![root.clone()];
    app.selected_workspace_path = Some(root.clone());
    app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;
    app.file_system_tree_view
        .sync_workspace_roots(&app.folder_order);
    app.file_system_tree_view.select_workspace_row(1);
    wait_for_file_viewer_load(&mut app.file_system_tree_view);

    let output = render_snapshot(&mut app, Rect::new(0, 0, 80, 10))?;

    let _ = fs::remove_dir_all(&root);
    assert!(output.contains("Files"));
    assert!(output.contains("README.m"));
    assert!(output.contains("  1 │"));
    assert!(output.contains("  3 │ Preview body"));
    assert!(output.contains("Project Notes"));
    assert!(output.contains("Preview body"));
    Ok(())
}

/// Waits briefly for async file preview loading in snapshot tests.
fn wait_for_file_viewer_load(view: &mut FileSystemTreeView) {
    for _ in 0..50 {
        if view.poll_watchers() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// Builds a dormant session entry for deterministic rendering snapshots.
fn session(title: &str, id: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new("now", title, id, "/tmp/project"))
}

/// Builds a minimal app state for rendering snapshots.
fn snapshot_app(session_terminals: Vec<SessionTerminal>) -> anyhow::Result<AppState> {
    let layout = build_shell_layout(
        default_shell_split_percent(),
        default_workspace_split_percent(),
    );
    let plan_root =
        std::env::temp_dir().join(format!("ratsus-snapshot-plans-{}", uuid::Uuid::new_v4()));
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

/// Returns expected content areas so snapshots never spawn real terminal processes.
fn terminal_split_content_areas(right_pane: PaneId, bottom_pane: PaneId) -> BTreeMap<PaneId, Rect> {
    BTreeMap::from([
        (TERMINAL_PANE_ID, Rect::new(18, 5, 29, 13)),
        (right_pane, Rect::new(49, 5, 29, 5)),
        (bottom_pane, Rect::new(49, 12, 29, 6)),
    ])
}

/// Renders the app and returns a cropped plain-symbol buffer snapshot.
fn render_snapshot(app: &mut AppState, crop: Rect) -> anyhow::Result<String> {
    let mut terminal = Terminal::new(TestBackend::new(80, 20))?;
    terminal.draw(|frame| render_app(app, frame))?;
    Ok(buffer_symbols(terminal.backend().buffer(), crop))
}

/// Converts a cropped terminal buffer region to plain symbols.
fn buffer_symbols(buffer: &Buffer, area: Rect) -> String {
    let mut output = String::new();
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.cell((x, y)) {
                output.push_str(cell.symbol());
            }
        }
        output.push('\n');
    }
    output
}
