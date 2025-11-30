use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            "(Q)",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Exit"),
        Span::raw("  "),
        Span::styled(
            "(F)",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Filter"),
    ]);

    frame.render_widget(
        Paragraph::new(line).block(Block::new().borders(Borders::ALL)),
        area,
    );
}