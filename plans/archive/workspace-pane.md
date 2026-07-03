# Workspace pane migration plan

## Objective

Add a workspace pane to the left of the existing left pane. A workspace is one session folder. Selecting a workspace filters the existing left-pane session experience to that single folder while keeping the current folder/session interactions inside the left pane.

## Confirmed requirements

1. The existing left-pane UX remains the same, but it only renders rows for the selected workspace folder.
2. The workspace pane renders folders/workspaces in the current persisted folder order.
3. `Ctrl+1` through `Ctrl+9` select workspaces by visible order; when workspace-pane mode is disabled they select folders and activate the first folder session.
4. Workspace folders can be reordered with the mouse.
5. Reordered workspace order is saved in app-owned preferences and never writes to user-owned Nexus data.
6. The workspace pane is independently resizable with the same resizable-grid behavior as the existing left pane.

## Implementation outline

1. Introduce a workspace pane module under `src/ui/workspace_pane/` for row projection, rendering, shortcut selection, and mouse drag behavior.
2. Extend shell layout from two panes to three pane IDs: workspace, left session pane, and main terminal pane.
3. Keep the left-pane visibility toggle as the visibility control for both workspace and left session panes.
4. Add `selected_workspace_path` to app state and derive the initial value from the active session folder after sessions load.
5. Filter `AppState::visible_rows()` by the selected workspace folder before passing folder order into existing left-pane row logic.
6. Reuse app-owned `folder_order` persistence for workspace order and add persisted shell/workspace split percentages for pane sizing.
7. Route mouse input inside the workspace area to workspace selection, drag reorder, and scrolling before routing session-pane input.
8. Resolve `Ctrl+1..9` globally before terminal passthrough so workspace selection works regardless of focused pane.
9. Add E2E coverage plus focused unit tests for filtering, shortcut selection, order persistence inputs, and layout IDs.

## Non-goals

- Do not migrate, rewrite, or delete Nexus-owned session data.
- Do not change the terminal pane or split-terminal model.
- Do not replace the existing left-pane folder/session interaction model beyond filtering it to one folder.
