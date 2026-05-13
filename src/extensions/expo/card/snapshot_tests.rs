use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Terminal;

use crate::extensions::expo::card::render_conversation::render_conversation_card;
use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

/// Verifies Expo title and observation borders visually touch without overlapping badly.
#[test]
fn renders_connected_title_and_observation_borders() -> anyhow::Result<()> {
    let preview = ConversationObservationPreview {
        has_more: true,
        observations: vec![
            "First observation fits".to_string(),
            "Second observation is deliberately long enough to ellipsize".to_string(),
            "Third observation".to_string(),
        ],
    };
    let mut terminal = Terminal::new(TestBackend::new(42, 9))?;

    terminal.draw(|frame| {
        render_conversation_card(
            frame,
            Rect::new(1, 1, 40, 8),
            "Profile just dev performance",
            &preview,
        );
    })?;

    assert_snapshot!(buffer_symbols(terminal.backend().buffer()), @r###"
                                          
 ╭──────────────────────────────────────╮ 
 │Profile just dev performance          │ 
 ╰──────────────────────────────────────╯ 
  │+ more                              │  
  │• First observation fits            │  
  │• Second observation is deliberatel…│  
  │• Third observation                 │  
  └────────────────────────────────────┘  
"###);
    assert_eq!(
        terminal.backend().buffer().cell((2, 2)).unwrap().fg,
        ratatui::style::Color::White
    );
    assert_eq!(
        terminal.backend().buffer().cell((4, 5)).unwrap().fg,
        ratatui::style::Color::White
    );
    Ok(())
}

/// Converts a terminal buffer to a plain symbol snapshot.
fn buffer_symbols(buffer: &Buffer) -> String {
    let mut output = String::new();
    for y in buffer.area.y..buffer.area.y + buffer.area.height {
        for x in buffer.area.x..buffer.area.x + buffer.area.width {
            if let Some(cell) = buffer.cell((x, y)) {
                output.push_str(cell.symbol());
            }
        }
        output.push('\n');
    }
    output
}
