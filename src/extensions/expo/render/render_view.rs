use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::state::app_state::AppState;
use crate::extensions::expo::filter::session_indices::expo_session_indices;
use crate::extensions::expo::layout::split_view_area::split_expo_view_area;
use crate::extensions::expo::render::render_cards::render_expo_cards;
use crate::extensions::expo::render::render_empty::render_empty_expo;
use crate::extensions::expo::render::render_footer::render_expo_footer;

/// Renders Expo as scrollable masonry cards for all selected-folder sessions.
pub fn render_expo_view(app: &mut AppState, frame: &mut Frame, area: Rect) {
    app.expo_card_areas.clear();
    let (body_area, footer_area) = split_expo_view_area(area);
    if app.selected_expo_folder.is_none() {
        render_empty_expo(
            frame,
            body_area,
            "Select a folder in the left pane to open Expo.",
        );
        render_expo_footer(app, frame, footer_area);
        return;
    }
    if expo_session_indices(app).is_empty() {
        render_empty_expo(frame, body_area, "No conversations found in this folder.");
        render_expo_footer(app, frame, footer_area);
        return;
    }
    render_expo_cards(app, frame, body_area);
    if app.expo_card_areas.is_empty() && !app.expo_filter_query.is_empty() {
        render_empty_expo(
            frame,
            body_area,
            format!("No conversations match '{}'.", app.expo_filter_query),
        );
    }
    render_expo_footer(app, frame, footer_area);
}
