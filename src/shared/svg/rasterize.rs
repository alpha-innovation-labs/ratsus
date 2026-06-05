//! SVG rasterization to image buffer.
//!
//! Converts SVG data to RGBA image buffer suitable for rendering in TUI.

use image::{DynamicImage, ImageBuffer, Rgba};
use resvg::tiny_skia::{IntSize, Pixmap, PixmapMut, Transform};
use std::path::Path;
use usvg::Options;

/// Errors that can occur during SVG rasterization.
#[derive(Debug, thiserror::Error)]
pub enum SvgRasterizeError {
    #[error("Failed to parse SVG: {0}")]
    ParseError(#[from] usvg::Error),
    #[error("Failed to render SVG: {0}")]
    RenderError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Rasterizes an SVG file to a DynamicImage at the specified dimensions.
///
/// # Arguments
/// * `path` - Path to the SVG file
/// * `width` - Target width in pixels
/// * `height` - Target height in pixels
///
/// # Returns
/// A `DynamicImage` containing the rasterized SVG
pub fn rasterize_svg(
    path: &Path,
    width: u32,
    height: u32,
) -> Result<DynamicImage, SvgRasterizeError> {
    let svg_data = std::fs::read(path)?;
    rasterize_svg_from_bytes(&svg_data, width, height)
}

/// Rasterizes SVG bytes to a DynamicImage at the specified dimensions.
///
/// # Arguments
/// * `svg_bytes` - Raw SVG data
/// * `width` - Target width in pixels
/// * `height` - Target height in pixels
///
/// # Returns
/// A `DynamicImage` containing the rasterized SVG
pub fn rasterize_svg_from_bytes(
    svg_bytes: &[u8],
    width: u32,
    height: u32,
) -> Result<DynamicImage, SvgRasterizeError> {
    let tree = usvg::Tree::from_data(svg_bytes, &Options::default())?;

    let scale_x = width as f32 / tree.size().width();
    let scale_y = height as f32 / tree.size().height();
    let scale = scale_x.min(scale_y);

    let pixmap_size = IntSize::from_wh(width, height)
        .ok_or_else(|| SvgRasterizeError::RenderError("Invalid dimensions".into()))?;

    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or_else(|| SvgRasterizeError::RenderError("Failed to create pixmap".into()))?;

    let transform = Transform::from_scale(scale, scale);

    // Render SVG to the pixmap
    // Create a mutable slice for PixmapMut
    let slice = pixmap.data_mut();
    let mut pixmap_mut = PixmapMut::from_bytes(slice, width, height)
        .ok_or_else(|| SvgRasterizeError::RenderError("Failed to create pixmap mut".into()))?;
    resvg::render(&tree, transform, &mut pixmap_mut);

    let rgba_data = pixmap.data();
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, rgba_data.to_vec()).ok_or_else(|| {
            SvgRasterizeError::RenderError("Failed to create image buffer".into())
        })?;

    Ok(DynamicImage::ImageRgba8(img))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rasterize_svg_from_string() {
        let svg = r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <rect width="100" height="100" fill="red"/>
</svg>"#;

        let result = rasterize_svg_from_bytes(svg.as_bytes(), 50, 50);
        assert!(result.is_ok());
        let img = result.unwrap();
        assert_eq!(img.width(), 50);
        assert_eq!(img.height(), 50);
    }
}
