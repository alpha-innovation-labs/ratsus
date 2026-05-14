use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;
use ratsus::app::state::app_state::AppState;
use ratsus::core::rendering::screen::render_app::render_app;

/// Renders the full TUI to a deterministic text buffer.
pub fn render_app_text(app: &mut AppState, width: u16, height: u16) -> anyhow::Result<String> {
    let mut terminal = Terminal::new(TestBackend::new(width, height))?;
    terminal.draw(|frame| render_app(app, frame))?;
    Ok(buffer_text(terminal.backend().buffer(), width, height))
}

/// Converts a ratatui test buffer to visible symbols.
fn buffer_text(buffer: &Buffer, width: u16, height: u16) -> String {
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            if let Some(cell) = buffer.cell((x, y)) {
                output.push_str(cell.symbol());
            }
        }
        output.push('\n');
    }
    output
}
