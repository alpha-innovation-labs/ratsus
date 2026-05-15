use std::path::PathBuf;

use anyhow::Result;
use ratatui::layout::Rect;

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::plans::data::plan_drag_state::PlanDragState;
use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::fs::load_markdown_plans::load_markdown_plans;

/// Stateful plan list hosted in the shared left-pane shell.
pub struct PlanListState {
    pub root_path: PathBuf,
    pub plans_dir: PathBuf,
    pub plans: Vec<PlanEntry>,
    pub focused_row: usize,
    pub active_index: Option<usize>,
    pub scroll: usize,
    pub filter_query: String,
    pub filtering: bool,
    pub pending_g: bool,
    pub drag: Option<PlanDragState>,
    pub last_area: Rect,
    pub preview_state: Option<FilePreviewState>,
}

impl PlanListState {
    /// Builds plan state from the current working directory.
    pub fn new() -> Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_root(root)
    }

    /// Builds plan state from an explicit project root.
    pub fn with_root(root_path: PathBuf) -> Result<Self> {
        let plans_dir = root_path.join("plans");
        let plans = load_markdown_plans(&plans_dir)?;
        let active_index = (!plans.is_empty()).then_some(0);
        let preview_state =
            active_index.map(|index| preview_state_for_path(&plans[index].path, false));
        Ok(Self {
            root_path,
            plans_dir,
            plans,
            focused_row: 0,
            active_index,
            scroll: 0,
            filter_query: String::new(),
            filtering: false,
            pending_g: false,
            drag: None,
            last_area: Rect::default(),
            preview_state,
        })
    }
}
