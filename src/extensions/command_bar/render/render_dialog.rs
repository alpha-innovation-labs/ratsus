use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogModalMode, DialogWidget};

use crate::app::state::app_state::AppState;
use crate::core::rendering::style::default_border_color::default_border_color;
use crate::extensions::command_bar::data::filtered_commands::filtered_command_bar_items;
use crate::extensions::command_bar::layout::body::CommandBarBody;
use crate::extensions::command_bar::layout::footer_text::command_bar_footer_text;

/// Renders the centered command bar modal dialog when it is open.
pub fn render_command_bar_dialog(app: &AppState, frame: &mut Frame) {
    if !app.command_bar.is_open {
        return;
    }
    let items = filtered_command_bar_items(&app.command_bar.query);
    let body = CommandBarBody::new(
        app.command_bar.query.clone(),
        items,
        app.command_bar.selected_position,
        app.command_bar.is_filtering,
    );
    let mut dialog = Dialog::new(" command bar ", "")
        .width_percent(0.68)
        .height_percent(0.58)
        .buttons(vec![])
        .content_padding(2, 1)
        .message_alignment(Alignment::Left)
        .footer(command_bar_footer_text())
        .footer_alignment(Alignment::Center)
        .footer_style(Style::default().fg(Color::DarkGray))
        .no_backdrop()
        .modal_mode(DialogModalMode::Blocking)
        .border_color(default_border_color())
        .body_renderer(Box::new(body));

    frame.render_widget(DialogWidget::new(&mut dialog), frame.area());
}
