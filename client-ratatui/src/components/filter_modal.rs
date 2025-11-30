use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crossterm::event::KeyCode;
use crate::model::FilterForm;

pub enum FilterModalAction {
    Apply,
    Cancel,
    Noop,
}

pub fn handle_key(code: KeyCode, form: &mut FilterForm) -> FilterModalAction {
    match code {
        KeyCode::Esc => FilterModalAction::Cancel,
        KeyCode::Enter => FilterModalAction::Apply,
        KeyCode::Tab => {
            form.cycle_next();
            FilterModalAction::Noop
        }
        KeyCode::BackTab => {
            form.cycle_prev();
            FilterModalAction::Noop
        }
        KeyCode::Backspace => {
            form.pop_active();
            FilterModalAction::Noop
        }
        KeyCode::Char(c) => {
            form.push_active(c);
            FilterModalAction::Noop
        }
        _ => FilterModalAction::Noop,
    }
}

pub fn render(frame: &mut Frame, form: &FilterForm) {
    let area = centered_rect(70, 50, frame.area());

    frame.render_widget(Clear, area);

    let title = "Filter";
    let help = "Tab/Shift-Tab = Switch • Enter = Apply • Esc = Cancel";
    let labels = ["Title", "Domain", "Url", "Description"];
    let values = [
        form.title_input.as_str(),
        form.domain_input.as_str(),
        form.url_input.as_str(),
        form.description_input.as_str(),
    ];

    let mut body = String::new();
    body.push_str(help);
    body.push_str("\n\n");
    for i in 0..4 {
        if i == form.active_index { body.push('>'); } else { body.push(' '); }
        body.push_str(labels[i]);
        body.push_str(": ");
        body.push_str(values[i]);
        if i < 3 { body.push('\n'); }
    }

    let block = Block::default().title(title).borders(Borders::ALL);
    let para = Paragraph::new(body).block(block);
    frame.render_widget(para, area);

    let cursor_line_offset = 2 + form.active_index as u16; // help + blank + index
    let prefix_len = 2 + labels[form.active_index].len() as u16 + 2; // space/'>' + space + label + ': '
    let cursor_x = area.x + 1 /*left border*/ + prefix_len + values[form.active_index].len() as u16;
    let cursor_y = area.y + 1 /*top border*/ + cursor_line_offset;
    frame.set_cursor_position((cursor_x, cursor_y));
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1]);

    vertical[1]
}