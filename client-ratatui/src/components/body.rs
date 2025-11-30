use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use crate::model::Event;

pub fn render(frame: &mut Frame, area: Rect, state: &mut TableState, data: &[Event]) {
    let rows = data.iter().map(|e| {
        Row::new(vec![
            Cell::from(e.title.as_str()),
            Cell::from(e.domain.as_str()),
            Cell::from(e.url.as_str()),
            Cell::from(e.description.as_str()),
        ])
    });

    let header = ["TITLE", "DOMAIN", "URL", "DESCRIPTION"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

    let table = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ")
    .block(Block::new().borders(Borders::ALL));

    frame.render_stateful_widget(table, area, state);
}