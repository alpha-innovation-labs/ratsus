//! Cached pi logo image for efficient TUI rendering.
//!
//! Renders the SVG once and caches the resulting image.

use crate::shared::svg::rasterize::rasterize_svg;
use image::DynamicImage;
use std::path::PathBuf;
use std::sync::OnceLock;

static LOGO_CACHE: OnceLock<DynamicImage> = OnceLock::new();

/// Default logo path (pi logo in Downloads).
const DEFAULT_LOGO_PATH: &str = "/Users/alpha/Downloads/pi-logo-on-dark.svg";

/// Logo dimensions for rendering.
/// Logo width in pixels.
pub const LOGO_WIDTH: u32 = 24;
/// Logo height in pixels.
pub const LOGO_HEIGHT: u32 = 24;

/// Returns the cached pi logo image, loading it if not already cached.
pub fn cached_logo() -> &'static DynamicImage {
    LOGO_CACHE.get_or_init(|| {
        let path = PathBuf::from(DEFAULT_LOGO_PATH);
        match rasterize_svg(&path, LOGO_WIDTH, LOGO_HEIGHT) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to load pi logo: {}, using fallback", e);
                // Return a 1x1 transparent pixel as fallback
                image::DynamicImage::new_rgba8(1, 1)
            }
        }
    })
}

/// Preloads the logo into cache.
pub fn preload_logo() {
    let _ = cached_logo();
}
