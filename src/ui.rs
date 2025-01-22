use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    if !app.has_config {
        render_no_config_hint(frame);
        return;
    }

    let curr_page = &app.sections[app.current_section];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let line = Line::from(vec![
        Span::raw("="),
        Span::raw(curr_page.title.clone()),
        Span::raw("="),
    ])
    .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(line, layout[0]);

    let mut list = vec![];

    for item in &curr_page.items {
        list.push(Line::from(vec![
            Span::styled(item.0.clone(), Style::default().fg(app.highlight_color)),
            Span::styled(item.1.clone(), Style::default().fg(app.primary_color)),
        ]));
    }

    let p = Paragraph::new(Text::from(list));

    frame.render_widget(p, layout[1]);
}

pub fn render_no_config_hint(frame: &mut Frame) {
    let block = Block::bordered();

    let paragraph = Paragraph::new(vec![
        "It seems like there is no config loaded!".into(),
        "To load a config, use <insert> or <insert>".into(),
    ])
    .centered()
    .block(block);

    frame.render_widget(paragraph, frame.area());
}
