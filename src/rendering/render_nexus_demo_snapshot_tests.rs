use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::mpsc;

use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Terminal;
use ratkit::primitives::resizable_grid::{PaneId, ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::delete_session_confirmation_state::DeleteSessionConfirmationState;
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
use crate::rendering::render_nexus_demo::render_nexus_demo;
use crate::terminal::session_terminal::SessionTerminal;

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
╭Chat──────────────────────────────────────────────────────────╮
│╭ Session A ─────────────── x ╮╭ Session B ─────────────── x ╮│
││Starting Nexus session…      ││Starting Nexus session…      ││
││                             ││                             ││
││                             ││                             ││
││                             ││                             ││
││                             ││                             ││
││                             │╰─────────────────────────────╯│
││                             │╭ Session C ─────────────── x ╮│
││                             ││Starting Nexus session…      ││
││                             ││                             ││
││                             ││                             ││
││                             ││                             ││
││                             ││                             ││
││                             ││                             ││
│╰─────────────────────────────╯╰─────────────────────────────╯│
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

    assert!(output.contains("┌─"));
    assert!(output.contains("└─"));
    assert_snapshot!(output, @r###"
╭ nexus session╮
│ project (2/2│
│  ┌─ 󰀘 Alp… 0m│
│  └─ 󰀘 Beta 0m│
│              │
│              │
"###);
    Ok(())
}

/// Builds a dormant session entry for deterministic rendering snapshots.
fn session(title: &str, id: &str) -> SessionTerminal {
    SessionTerminal::dormant(NexusSession::new("now", title, id, "/tmp/project"))
}

/// Builds a minimal app state for rendering snapshots.
fn snapshot_app(session_terminals: Vec<SessionTerminal>) -> anyhow::Result<NexusDemo> {
    let (_sender, receiver) = mpsc::channel::<SessionRefreshResult>();
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);
    layout.set_split_percent(20);
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
        delete_confirmation: DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        session_terminals,
        visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
        closed_chat_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
        active_index: 0,
        focused_index: 0,
        session_scroll: 0,
        pending_left_scroll_redraw: false,
        suppress_left_focus_scroll: false,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: vec!["/tmp/project".into()],
        focused_row: 1,
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
        expo_card_width: crate::expo::default_expo_card_width::default_expo_card_width(),
        expo_filter_query: String::new(),
        expo_filtering: false,
        expo_card_areas: Vec::new(),
        observation_previews: HashMap::new(),
        observation_cache_receiver: None,
        observation_watcher: None,
        last_main_pane_area: Rect::default(),
        file_system_tree_view: FileSystemTreeView::new()?,
        loader_tick: 0,
        session_refresh_receiver: receiver,
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
fn render_snapshot(app: &mut NexusDemo, crop: Rect) -> anyhow::Result<String> {
    let mut terminal = Terminal::new(TestBackend::new(80, 20))?;
    terminal.draw(|frame| render_nexus_demo(app, frame))?;
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
