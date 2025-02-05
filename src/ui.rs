use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
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

    let items: Vec<Line> = curr_table
        .entries
        .iter()
        .map(|entry| get_line_from_entry(entry, app))
        .collect();

    let p = Paragraph::new(Text::from(items)).centered().block(block);

    frame.render_widget(p, frame.area());
}

fn get_line_from_entry<'a>(entry: &'a (Vec<String>, String), app: &'a App) -> Line<'a> {
    let mut line = vec![];

    let keys = &entry.0;
    let description = &entry.1;

    if !keys.is_empty() {
        line.push(
            Span::from(keys.first().unwrap())
                .fg(app.highlight_color())
                .bold(),
        );

        for key in keys.iter().skip(1) {
            line.push(Span::from("+").fg(app.primary_color()));
            line.push(Span::from(key).fg(app.highlight_color()).bold());
        }

        line.push(" ".into());
    }

    line.push(Span::from(description).fg(app.primary_color()));

    Line::from(line)
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
