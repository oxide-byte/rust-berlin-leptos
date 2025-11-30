use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use color_eyre::Result;
use ratatui::prelude::*;
use crate::components::{body, footer, header};

#[derive(Default)]
pub struct App {}

impl App {
    pub(crate) fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if self.handle_events()? {
                println!("Application closes...");
                break;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(9),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        header::render(frame, layout[0]);
        body::render(frame, layout[1]);
        footer::render(frame, layout[2]);
    }

    fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true), // Exit
                    KeyCode::Char('Q') => return Ok(true), // Exit
                    _ => return Ok(false)
                }
            }
        }
        Ok(false)
    }
}