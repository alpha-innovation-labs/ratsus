# Workspace pane

The workspace pane is the folder-level navigation pane shown to the left of the session left pane.

## Behavior

- One workspace is one session folder.
- Workspaces render in `AppState::folder_order`, which is persisted in app-owned session order preferences and in the Nexus multiplexer state.
- Workspace folders render as rounded boxes without folder icons; the active workspace uses folder blue and inactive workspaces use session gray.
- The top border shows the total session count as a plain right-aligned number.
- Selecting a workspace sets `AppState::selected_workspace_path`, restores that workspace's last focused session, and moves focus to the main pane.
- The session left pane shows all sessions for the selected workspace without a folder header or `+ more` overflow row.
- `Ctrl+1` through `Ctrl+9` select workspaces by visible order and are also discoverable from the command bar.
- When workspace-pane mode is disabled, the same shortcuts select the matching folder and activate its first session.
- Mouse drag reorders workspaces and persists the updated app-owned folder order.
- Mouse wheel scrolls the workspace list when it exceeds pane height.
- The left-pane visibility toggle hides or shows both the workspace pane and session left pane.
- The command bar `Toggle workspace view` command switches between workspace-pane mode and the legacy all-folders left-pane view.
- Workspace order, selected workspace, and workspace-view mode are saved in `~/.local/share/nexus/multiplexer/split-pane-state.json`.

## Boundaries

- The workspace pane never writes to Nexus-owned session data.
- The workspace pane owns folder-level navigation, rendering, mouse reorder, and shortcut selection.
- The left panel continues to own session rows, folder expansion, session activation, selection, and deletion.
