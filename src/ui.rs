use std::{cmp::max, ops::Not};

use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Row, Table},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    if !app.has_config() {
        render_no_config_hint(frame);
        return;
    }

    let curr_table = app
        .get_current_page()
        // we may want to rewrite this, we could have a config that just has no pages
        .expect("Config flag is set but no config is present.");

    let title = Line::from(format!("[ {} ]", curr_table.name))
        .fg(app.highlight_color())
        .bold();

    let legend = Line::from(vec![
        //" <Left> ".fg(app.highlight_color()),
        Span::styled(" <Left> ", Style::from(app.highlight_color())),
        Span::styled("Previous Page", Style::from(app.primary_color())),
        Span::styled(" <Right> ", Style::from(app.highlight_color())),
        Span::styled("Next Page", Style::from(app.primary_color())),
        Span::styled(" <q> ", Style::from(app.highlight_color())),
        Span::styled("Close", Style::from(app.primary_color())),
        Span::styled(
            format!(
                " [Page {} of {}] ",
                app.current_page_number() + 1,
                app.number_of_pages()
            ),
            Style::from(app.highlight_color()),
        ),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(legend.centered());

    let mut rows = Vec::new();

    let mut maximum_length = 0;

    for entry in &curr_table.entries {
        let shortcut = get_shortcut(entry, app);
        maximum_length = max(maximum_length, shortcut.width());

        rows.push(Row::new(vec![
            Text::from(shortcut),
            Text::from(entry.1.as_str()),
        ]));
    }

    let table = Table::new(
        rows,
        vec![
            Constraint::Length(maximum_length as u16),
            Constraint::Fill(1),
        ],
    )
    .block(block);

    frame.render_widget(table, frame.area());
}

fn get_shortcut<'a>(entry: &(Vec<String>, String), app: &'a App) -> Line<'a> {
    let shortcut_components = &entry.0;

    let mut shortcut = Line::default();

    if shortcut_components.is_empty().not() {
        shortcut.push_span(
            shortcut_components
                .first()
                .unwrap()
                .clone()
                .fg(app.highlight_color())
                .bold(),
        );

        // TODO: use as_str() instead of cloning
        for component in shortcut_components.iter().skip(1) {
            shortcut.push_span("+".fg(app.primary_color()));
            shortcut.push_span(component.clone().fg(app.highlight_color()).bold());
        }
    }

    shortcut
}

fn render_no_config_hint(frame: &mut Frame) {
    let block = Block::bordered();

    let paragraph = Paragraph::new(vec![
        "It seems like there is no config loaded!".into(),
        "To load a config, use <insert> or <insert>".into(),
    ])
    .centered()
    .block(block);

    frame.render_widget(paragraph, frame.area());
}
