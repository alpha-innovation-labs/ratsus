# Virtual Grid Viewport Plan

## Goal

Allow grid splits to keep panes usable after they reach fixed minimum sizes by expanding a virtual viewport instead of continuing to shrink panes. Users should be able to pan across the larger grid with mouse or keyboard controls.

## Problem

The grid can split panes vertically with `Ctrl+]` and horizontally with `Ctrl+[`. Repeated splits eventually make panes too small to use. The desired behavior is to preserve split semantics while avoiding panes below a fixed minimum width or height.

## Desired Behavior

### Vertical splits

1. `Ctrl+]` performs the normal vertical split while both resulting panes can satisfy the minimum width.
2. The first split may still divide the visible area 50/50.
3. When another vertical split would make a pane narrower than the fixed minimum width, the grid expands the virtual viewport width instead.
4. The new pane is placed to the right in the expanded virtual viewport.
5. Focus moves to the new pane.
6. The viewport scroll offset updates so the new focused pane is visible.

Example with a visible width of `1000px` and a minimum pane width of `500px`:

```text
Visible terminal width: 1000px
Initial split:          [ pane A 500 ][ pane B 500 ]
Next vertical split:    virtual width grows to 1500px
Virtual layout:         [ pane A 500 ][ pane B 500 ][ pane C 500 ]
Visible viewport:                         scrolls toward pane C
```

### Horizontal splits

1. `Ctrl+[` performs the normal horizontal split while both resulting panes can satisfy the minimum height.
2. When another horizontal split would make a pane shorter than the fixed minimum height, the grid expands the virtual viewport height instead.
3. The new pane is placed below the existing split region in the expanded virtual viewport.
4. Focus moves to the new pane.
5. The viewport scroll offset updates so the focused pane is visible.

## Decisions Captured

- Pane minimums are fixed sizes, not percentages.
- New off-screen panes receive focus immediately.
- The viewport scrolls automatically to reveal the newly focused pane.
- Trackpad panning should move the virtual viewport horizontally and vertically.
- A horizontal scrollbar should be visible when virtual width exceeds visible width.
- A vertical scrollbar should be visible when virtual height exceeds visible height.
- Keyboard shortcuts should allow horizontal and vertical viewport panning.
- Split sizing should preserve existing ratios until doing so would violate fixed pane minimums.

## Design Notes

The grid should distinguish between:

- The visible terminal area.
- The larger virtual grid area.
- Pane rectangles in virtual coordinates.
- The viewport scroll offset into the virtual grid.

Rendering should draw only the intersection of each pane's virtual rectangle with the visible terminal viewport. Input focus remains pane-based, while pointer and scroll events need to account for the viewport offset.

## Implementation Outline

1. Add fixed minimum width and height settings for grid panes.
2. Track a virtual grid size independently from the visible grid area.
3. Track viewport scroll offsets for x and y axes.
4. Update vertical split logic to expand virtual width when fixed minimums would be violated.
5. Update horizontal split logic to expand virtual height when fixed minimums would be violated.
6. Preserve split ratios while they remain compatible with pane minimums.
7. Move focus to newly created panes.
8. Scroll the viewport enough to reveal the focused pane after creation or focus changes.
9. Add mouse or trackpad panning for both axes.
10. Add hotkeys for viewport panning.
11. Render horizontal and vertical scrollbars when virtual size exceeds visible size.
12. Clamp viewport offsets when panes close, resize, or the terminal changes size.

## Testing Plan

Start with tests before implementation:

1. Splitting within available space keeps the current visible-only layout behavior.
2. A vertical split that would violate minimum width expands virtual width instead.
3. A horizontal split that would violate minimum height expands virtual height instead.
4. New panes receive focus after virtual expansion.
5. Viewport offsets update to reveal newly focused panes.
6. Mouse or trackpad pan events update viewport offsets and remain clamped.
7. Keyboard pan actions update viewport offsets and remain clamped.
8. Scrollbars appear only when the virtual size exceeds the visible size.
9. Terminal resize clamps offsets and preserves focused-pane visibility where possible.

## Open Questions

- What exact minimum width and height should be used?
- Which hotkeys should pan left, right, up, and down?
- Should pane content receive scroll events before viewport panning, or should viewport panning always win while the grid is focused?
- Should scrollbars be purely informational or support mouse dragging?
