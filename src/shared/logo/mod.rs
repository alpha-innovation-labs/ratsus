//! Pi logo rendering for the TUI.
//!
//! Loads and caches the pi logo SVG for display in the left panel.

mod cached_logo;

pub use cached_logo::{cached_logo, preload_logo, LOGO_HEIGHT, LOGO_WIDTH};
