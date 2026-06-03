use ratatui::{layout::Rect, Frame};

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::ui::left_panel::action::LeftPaneAction;
use crate::ui::left_panel::content::LeftPaneContent;
use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
use crate::ui::left_panel::input::key_behavior::LeftPanelKeyBehavior;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Active content currently hosted by the shared left-pane shell.
pub enum ActiveLeftPaneContent<'a> {
    Chat(LeftPanelKeyBehavior<'a>),
    Files(&'a mut FileSystemTreeView),
    Plans(&'a mut PlanListState),
}

impl<'a> ActiveLeftPaneContent<'a> {
    /// Builds the active left-pane content adapter from the explicit left-pane mode.
    pub fn for_app(app: &'a mut AppState) -> Self {
        match app.left_pane_mode {
            LeftPaneMode::Sessions => Self::Chat(LeftPanelKeyBehavior::new(app)),
            LeftPaneMode::Plans => {
                let workspace_folders = app.folder_order.clone();
                let _ = app.plan_list.sync_workspace_folders(&workspace_folders);
                app.last_session_list_area = Rect::default();
                Self::Plans(&mut app.plan_list)
            }
            LeftPaneMode::Files => {
                let workspace_folders = app.folder_order.clone();
                app.file_system_tree_view
                    .sync_workspace_roots(&workspace_folders);
                app.last_session_list_area = Rect::default();
                Self::Files(&mut app.file_system_tree_view)
            }
        }
    }
}

impl LeftPaneContent for ActiveLeftPaneContent<'_> {
    /// Handles one semantic action against the active left-pane content.
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome {
        match self {
            Self::Chat(content) => content.handle_left_pane_action(action),
            Self::Files(content) => content.handle_left_pane_action(action),
            Self::Plans(content) => content.handle_left_pane_action(action),
        }
    }

    /// Returns the active content title for the shared left-pane shell.
    fn title(&self) -> String {
        match self {
            Self::Chat(content) => content.title(),
            Self::Files(content) => content.title(),
            Self::Plans(content) => content.title(),
        }
    }

    /// Records the active content body area before rendering.
    fn prepare_body_area(&mut self, area: Rect) {
        match self {
            Self::Chat(content) => content.prepare_body_area(area),
            Self::Files(content) => content.prepare_body_area(area),
            Self::Plans(content) => content.prepare_body_area(area),
        }
    }

    /// Renders the active content body inside the shared shell.
    fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            Self::Chat(content) => content.render_body(frame, area),
            Self::Files(content) => content.render_body(frame, area),
            Self::Plans(content) => content.render_body(frame, area),
        }
    }

    /// Returns footer items for the active left-pane content.
    fn footer_items(&self) -> Vec<LeftPaneFooterItem> {
        match self {
            Self::Chat(content) => content.footer_items(),
            Self::Files(content) => content.footer_items(),
            Self::Plans(content) => content.footer_items(),
        }
    }

    /// Returns optional status text for the active left-pane footer.
    fn footer_status(&self) -> Option<String> {
        match self {
            Self::Chat(content) => content.footer_status(),
            Self::Files(content) => content.footer_status(),
            Self::Plans(content) => content.footer_status(),
        }
    }

    /// Returns whether active content is accepting filter text.
    fn is_filtering(&self) -> bool {
        match self {
            Self::Chat(content) => content.is_filtering(),
            Self::Files(content) => content.is_filtering(),
            Self::Plans(content) => content.is_filtering(),
        }
    }

    /// Returns whether active content is waiting for a second `g` key.
    fn has_pending_g(&self) -> bool {
        match self {
            Self::Chat(content) => content.has_pending_g(),
            Self::Files(content) => content.has_pending_g(),
            Self::Plans(content) => content.has_pending_g(),
        }
    }

    /// Updates whether active content is waiting for a second `g` key.
    fn set_pending_g(&mut self, pending: bool) {
        match self {
            Self::Chat(content) => content.set_pending_g(pending),
            Self::Files(content) => content.set_pending_g(pending),
            Self::Plans(content) => content.set_pending_g(pending),
        }
    }
}
