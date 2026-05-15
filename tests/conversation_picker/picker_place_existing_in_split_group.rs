use anyhow::{ensure, Context};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use insta::assert_snapshot;
use ratkit::KeyboardEvent;
use ratsus::app::input::handle_keyboard_event::handle_keyboard_event;
use ratsus::extensions::harness::conversation_picker::data::item::ConversationPickerItemKind;
use ratsus::extensions::harness::conversation_picker::data::items::conversation_picker_items;
use ratsus::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;
use ratsus::ui::left_panel::session::list_row::SessionListRow;
use ratsus::ui::left_panel::session::visible_rows::visible_session_rows;

use crate::support::create_real_app::create_real_app;
use crate::support::render_app_text::render_app_text;

/// Reproduces Ctrl+Shift+] placement and snapshots the split-group UI facts.
#[test]
fn picker_place_existing_in_split_group() -> anyhow::Result<()> {
    let mut app = create_real_app()?;
    let (active_index, placed_index) = same_folder_session_pair(&app)
        .context("real Nexus catalog needs two sessions in one folder for split-group E2E")?;
    let active_id = app.session_terminals[active_index].session.id.clone();
    app.active_index = active_index;
    app.focused_index = active_index;
    app.terminal_pane_sessions
        .insert(TERMINAL_PANE_ID, active_id.clone());
    app.terminal_pane_session_bundles
        .insert(TERMINAL_PANE_ID, vec![active_id]);

    let _ = render_app_text(&mut app, 120, 32)?;
    handle_keyboard_event(
        &mut app,
        key(
            KeyCode::Char(']'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        ),
    )?;
    ensure!(
        app.conversation_picker.is_open,
        "placement picker must open"
    );
    select_picker_session(&mut app, placed_index)?;
    handle_keyboard_event(&mut app, key(KeyCode::Enter, KeyModifiers::empty()))?;
    ensure!(
        !app.conversation_picker.is_open,
        "placement picker must close after activation"
    );

    let rendered = render_app_text(&mut app, 120, 32)?;
    let pane_count = app
        .terminal_layout
        .layout_panes(app.last_terminal_area)
        .len();
    let group_names = app
        .split_pane_session_groups
        .groups
        .values()
        .map(|group| group.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let rows = visible_session_rows(
        &app.session_terminals,
        &app.collapsed_folders,
        &app.folder_order,
        Some(app.focused_index),
        &app.split_pane_session_groups,
        &app.terminal_pane_session_bundles,
    );
    let left_group_rows = rows
        .iter()
        .filter(|row| matches!(row, SessionListRow::SplitGroup { name, .. } if name == "Group 1"))
        .count();
    let left_group_children = rows
        .iter()
        .filter(|row| matches!(row, SessionListRow::SplitGroupChild { group_id: 1, .. }))
        .count();

    assert_snapshot!(format!(
        "pane_count: {pane_count}\n\
         group_count: {}\n\
         group_names: {group_names}\n\
         active_index_is_placed: {}\n\
         left_group_rows: {left_group_rows}\n\
         left_group_children: {left_group_children}\n\
         group_label_count: {}\n",
        app.split_pane_session_groups.groups.len(),
        app.active_index == placed_index,
        rendered.matches("Group 1").count(),
    ), @r###"
pane_count: 2
group_count: 1
group_names: Group 1
active_index_is_placed: true
left_group_rows: 1
left_group_children: 2
group_label_count: 3
"###);
    Ok(())
}

/// Finds two real catalog sessions under the same working directory.
fn same_folder_session_pair(
    app: &ratsus::app::state::app_state::AppState,
) -> Option<(usize, usize)> {
    for (left_index, left) in app.session_terminals.iter().enumerate() {
        for (right_index, right) in app
            .session_terminals
            .iter()
            .enumerate()
            .skip(left_index + 1)
        {
            if left.session.working_dir == right.session.working_dir {
                return Some((left_index, right_index));
            }
        }
    }
    None
}

/// Selects one session row in the open conversation picker.
fn select_picker_session(
    app: &mut ratsus::app::state::app_state::AppState,
    session_index: usize,
) -> anyhow::Result<()> {
    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    app.conversation_picker.selected_position = items
        .iter()
        .position(|item| {
            matches!(
                item.kind,
                ConversationPickerItemKind::Session { index, .. } if index == session_index
            )
        })
        .context("placed session must be visible in the picker")?;
    Ok(())
}

/// Builds a pressed keyboard event for app-level E2E input.
fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
    KeyboardEvent {
        key_code,
        modifiers,
        kind: KeyEventKind::Press,
    }
}
