# Split Pane Session Groups Plan

## Goal

Show split-related terminal and chat sessions as hierarchical groups in the left pane instead of as unrelated flat session rows.

## Problem

The main pane can contain multiple split panes that are functionally related because they belong to the same split layout. The left pane currently shows those sessions as individual rows, which hides their relationship and makes it harder to understand which sessions belong together.

## Desired Behavior

When a session is split, the sessions attached to that split layout should appear under a shared parent group in the left pane.

Pipe-style example:

```text
Group 1
├─ Terminal: api
├─ Chat: refactor auth
└─ Chat: tests

Group 2
├─ Chat: homepage
└─ Terminal: dev server
```

The current row where a session appears becomes a group parent row when that session has split-related pane sessions. Child rows under the group represent the individual pane sessions.

## Captured Requirements

- Split pane sessions should be grouped in the left pane.
- The parent row name starts with `Group`.
- Child rows should use pipe-style tree connectors.
- Opening a new vertical terminal split should place that terminal in the same group.
- Opening a new horizontal chat split should place that chat in the same group.
- Selecting a child row should activate the related pane and session.
- The group should make pane/session relationships visible without changing the split layout itself.

## Design Notes

The model should distinguish between:

- A split group identity.
- A group parent row in the left pane.
- Child rows for pane-bound sessions.
- The existing terminal pane id to session id mapping.
- The active pane/session inside a group.

The left pane should render groups as a hierarchy, while terminal layout state remains owned by the grid layout domain.

## Suggested Data Model

Introduce a group model that can map pane ids and session ids to a stable group id.

Possible shape:

```rust
pub struct SplitPaneSessionGroup {
    pub id: SplitPaneSessionGroupId,
    pub name: String,
    pub panes: Vec<PaneId>,
}
```

Exact ownership and API should be refined during implementation. The group should not duplicate session data; it should reference existing pane/session mappings.

## Implementation Outline

1. Add a split-pane group state model under the grid-layout or left-panel domain.
2. Create a group when the first split is created from an ungrouped pane.
3. Add newly split panes to the source pane's existing group.
4. Render grouped left-pane rows with pipe-style connectors.
5. Keep ungrouped single sessions rendering as normal flat rows.
6. Activate the matching pane and session when a child row is selected.
7. Define behavior for selecting the parent group row.
8. Update close-pane behavior to remove closed panes from their group.
9. Collapse or remove a group when it has fewer than two pane sessions.
10. Preserve group identity when panes are focused, reordered, or resized.
11. Add tests for grouped rendering and child-row activation.

## Testing Plan

Start with tests before implementation:

1. A single unsplit session still renders as a flat session row.
2. The first split converts the source session row into a `Group` parent.
3. A vertical terminal split appears as a child in the same group.
4. A horizontal chat split appears as a child in the same group.
5. Pipe-style connectors render correctly for first, middle, and last children.
6. Selecting a child row activates its pane and session.
7. Closing a pane removes that child from the group.
8. Closing down to one pane collapses or removes the group.
9. Grouped rows preserve existing keyboard navigation behavior.
10. Grouped rows preserve mouse click activation behavior.

## Implementation Notes

Implemented split groups use stable creation-order names like `Group 1`. Parent rows activate the group's currently active child when possible, otherwise the first valid child. Groups are removed when pane/session membership drops below two pane-bound sessions.

## Open Questions

- Should group rows be collapsible?
- Parent group row selection currently activates the active child, or the first child when none is active.
- Should groups be renamed by the user or always auto-named?
- Should numbering be global, per folder, or based on creation order?
- Should a normal terminal and a chat use different child labels or icons?
