//! UI rendering module for displaying pages for recall.
//!
//! This module provides the main `ui` function for drawing the interface using 'ratatui'
//! and helper functions for building stylized tables and shortcut lines.

use std::cmp::max;

use ratatui::{
    layout::Constraint,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Padding, Row, Table},
    Frame,
};

use crate::app::{App, Entry};

/// Renders the main user interface for the application within the given frame.
///
/// This function constructs a stylized table of entries on the current page,
/// along with a title, legend, and page counter.
pub fn ui(frame: &mut Frame, app: &App) {
    let curr_page = app
        .get_current_page()
        // we may want to rewrite this, we could have a config that just has no pages
        .expect("Expected page number to reference an existing page");

    let title = Line::from(format!("[ {} ]", curr_page.name))
        .fg(app.highlight_color())
        .bold();

    let page_counter = format!(
        " [Page {} of {}] ",
        app.current_page_number() + 1,
        app.number_of_pages()
    );

    let legend = Line::from(vec![
        " <Left> ".fg(app.highlight_color()),
        "Previous Page".fg(app.primary_color()),
        " <Right>".fg(app.highlight_color()),
        "Next Page".fg(app.primary_color()),
        " <q> ".fg(app.highlight_color()),
        "Close".fg(app.primary_color()),
        page_counter.fg(app.highlight_color()),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(legend.centered())
        .padding(Padding::horizontal(1));

    let table = build_table(
        &curr_page.entries,
        app.primary_color(),
        app.highlight_color(),
    )
    .block(block);

    frame.render_widget(table, frame.area());
}

/// Builds a stylized table widget from a list of entries.
///
/// Each entry includes a keyboard shortcut (as a sequence of keys)
/// and a textual description.
/// The resulting table is formatted with aligned columns and spacing.
/// To do this, we need to measure the maximum width of such a shortcut.
fn build_table(entries: &Vec<Entry>, primary_color: Color, highlight_color: Color) -> Table {
    let mut maximum_shortcut_length = 0;

    let mut rows = Vec::new();

    for entry in entries {
        let shortcut = build_shortcut(&entry.content, primary_color, highlight_color);

        // In order to measure the correct column width, we need to track the maximum length of such a shortcut
        maximum_shortcut_length = max(maximum_shortcut_length, shortcut.width());

        let description = entry.description.as_str().into();

        let row = Row::new([shortcut, description]);

        rows.push(row);
    }

    let widths = [
        Constraint::Min(maximum_shortcut_length as u16),
        Constraint::Percentage(75),
    ];

    let table = Table::new(rows, widths).column_spacing(2);

    table
}

/// Builds a stylized span from a list of keys or other textual content
///
/// The resulting span is an alternating sequence of the given content and a connecting element, in this case the character '+'.
fn build_shortcut(content: &[String], primary_color: Color, highlight_color: Color) -> Line {
    let mut shortcut = Line::default();

    if content.is_empty() {
        return shortcut;
    }

    // Do not precompose a '+' before the first actual text-component.
    // first always exists, since content is non-empty
    shortcut.push_span(content.first().unwrap().as_str().fg(highlight_color).bold());

    for component in content.iter().skip(1) {
        shortcut.push_span("+".fg(primary_color));
        shortcut.push_span(component.as_str().fg(highlight_color).bold());
    }

    shortcut
}
