use anyhow::Result;

use crate::app::adjust_index_after_removals::adjust_index_after_removals;
use crate::app::clamp_session_index::clamp_session_index;
use crate::app::next_chat_index::next_chat_index;
use crate::app::nexus_demo_state::NexusDemo;
use crate::app::remove_exited_sessions::RemovedExitedSessions;
use crate::nexus_sessions::start_new_nexus_chat_in_dir::start_new_nexus_chat_in_dir;

/// Restores a valid active/focused session after exited entries were removed.
pub fn restore_focus_after_removals(
    app: &mut NexusDemo,
    exited_indices: &[usize],
    removed: RemovedExitedSessions,
) -> Result<()> {
    if removed.removed_active_chat {
        return focus_next_chat_or_create(app, removed, preferred_index(exited_indices));
    }
    restore_existing_focus(app, exited_indices, removed.removed_active);
    Ok(())
}

/// Focuses the next chat or starts a replacement chat when none remain.
fn focus_next_chat_or_create(
    app: &mut NexusDemo,
    removed: RemovedExitedSessions,
    preferred_index: usize,
) -> Result<()> {
    if let Some(index) = next_chat_index(&app.session_terminals, preferred_index) {
        app.focused_index = index;
        app.activate_focused_session();
        return Ok(());
    }
    start_new_nexus_chat_in_dir(app, &removed.fallback_working_dir)
}

/// Restores focus after terminal removals and non-active chat removals.
fn restore_existing_focus(app: &mut NexusDemo, exited_indices: &[usize], removed_active: bool) {
    app.active_index = if removed_active {
        clamp_session_index(preferred_index(exited_indices), app.session_terminals.len())
    } else {
        adjust_index_after_removals(app.active_index, exited_indices).unwrap_or(0)
    };
    app.focused_index =
        adjust_index_after_removals(app.focused_index, exited_indices).unwrap_or(app.active_index);
    app.focused_index = clamp_session_index(app.focused_index, app.session_terminals.len());
    if removed_active && !app.session_terminals.is_empty() {
        app.focused_index = app.active_index;
        app.activate_focused_session();
        return;
    }
    app.keep_focused_session_visible();
}

/// Returns the first removed index as the preferred successor position.
fn preferred_index(exited_indices: &[usize]) -> usize {
    exited_indices.first().copied().unwrap_or(0)
}
