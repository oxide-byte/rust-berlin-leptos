use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use crate::model::Event;

pub fn render(frame: &mut Frame, area: Rect) {
    let mut state = TableState::default().with_selected(0);

    let data: Vec<Event> = Vec::new();

    let rows:Vec<Row> = data.iter()
        .map(|e| Row::new(vec![
            Cell::new(e.title.clone()),
            Cell::new(e.domain.clone()),
            Cell::new(e.url.clone()),
            Cell::new(e.description.clone()),
        ])).collect();

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
    ).header(header)
    .block(Block::new().borders(Borders::ALL));

    frame.render_stateful_widget(table, area, &mut state);
}