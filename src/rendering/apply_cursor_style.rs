use crossterm::{cursor::SetCursorStyle, execute};
use ratkit::primitives::termtui::CursorStyle;

/// Applies the terminal cursor style requested by the emulated screen.
pub fn apply_cursor_style(style: CursorStyle) {
    let style = match style {
        CursorStyle::Default => SetCursorStyle::DefaultUserShape,
        CursorStyle::BlinkingBlock => SetCursorStyle::BlinkingBlock,
        CursorStyle::SteadyBlock => SetCursorStyle::SteadyBlock,
        CursorStyle::BlinkingUnderline => SetCursorStyle::BlinkingUnderScore,
        CursorStyle::SteadyUnderline => SetCursorStyle::SteadyUnderScore,
        CursorStyle::BlinkingBar => SetCursorStyle::BlinkingBar,
        CursorStyle::SteadyBar => SetCursorStyle::SteadyBar,
    };
    let _ = execute!(std::io::stdout(), style);
}
