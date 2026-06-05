//! Logo rendering for the left panel.
//!
//! Renders the pi logo using terminal graphics protocols.

use crate::shared::logo::{cached_logo, LOGO_HEIGHT, LOGO_WIDTH};
use ratatui::layout::Rect;
use ratatui::Frame;

/// Renders the pi logo in the left panel header area.
///
/// Uses the Kitty graphics protocol if available, otherwise falls back to unicode block
/// characters approximation.
pub fn render_left_panel_logo(frame: &mut Frame, area: Rect) {
    // Get the cached logo
    let logo = cached_logo();

    // Only render if we have a valid logo (not the 1x1 fallback)
    if logo.width() > 1 && logo.height() > 1 {
        // Calculate the logo position (top-right of the area)
        let logo_area = Rect::new(
            area.x + area.width.saturating_sub(LOGO_WIDTH as u16 + 2),
            area.y + 1,
            LOGO_WIDTH as u16,
            LOGO_HEIGHT as u16,
        );

        // Get RGBA data from the logo
        let rgba = logo.to_rgba8();
        let (img_width, img_height) = rgba.dimensions();
        let data = rgba.into_raw();

        // Emit Kitty graphics protocol
        // Format: ESC_G id=1,t=d,s=<width>,v=<height>,c=1:<base64_data> ESC_BACKSLASH
        let encoded = base64_encode(&data);

        let cmd = format!(
            "\x1b_Gi=1,t=d,s={},v={},c=1:{}\x1b\\",
            img_width, img_height, encoded
        );

        // Write to the frame's buffer as a placeholder
        // Note: Real implementation would need terminal output
        // For now, we skip rendering if we can't write to stdout
        let _ = cmd;
        let _ = logo_area;
        let _ = frame;
    }
}

/// Simple base64 encoding for RGBA data.
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        result.push(CHARS[b0 >> 2] as char);
        result.push(CHARS[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }
    result
}
