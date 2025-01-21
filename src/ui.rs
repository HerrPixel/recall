use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title_string = match app.number_of_pages {
        0 => "placeholder",
        _ => &app.pages[app.current_page].title,
    };

    let title = Paragraph::new(Text::styled(title_string, Style::default().fg(Color::Blue)))
        .block(title_block);

    let area = Rect::new(0, 0, 10, 10);

    frame.render_widget(title, area);
}
