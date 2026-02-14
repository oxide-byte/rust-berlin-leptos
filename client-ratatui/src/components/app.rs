use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use color_eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::TableState;
use crate::components::{body, footer, header, filter_modal, login};
use crate::graphql::fetch_meetup_url_data;
use crate::model::{Event as MeetupEvent, FilterGraphql, FilterForm};
use crate::auth::authenticate;

enum AppState {
    Login,
    Main,
}

pub struct App {
    state: AppState,
    login_form: login::LoginForm,
    access_token: Option<String>,
    table_state: TableState,
    data: Vec<MeetupEvent>,
    total_count: i64,
    show_filter_modal: bool,
    filter_form: FilterForm,
    current_filter: FilterGraphql,
    rt: tokio::runtime::Runtime,
}

impl App {
    pub fn new() -> Self {
        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

        App {
            state: AppState::Login,
            login_form: login::LoginForm::default(),
            access_token: None,
            table_state: TableState::default().with_selected(0),
            data: Vec::new(),
            total_count: 0,
            show_filter_modal: false,
            filter_form: FilterForm::default(),
            current_filter: FilterGraphql::default(),
            rt,
        }
    }

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

    fn draw(&mut self, frame: &mut Frame) {
        match self.state {
            AppState::Login => {
                login::render(frame, &self.login_form);
            }
            AppState::Main => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Length(9),
                        Constraint::Fill(1),
                        Constraint::Length(3),
                    ])
                    .split(frame.area());

                header::render(frame, layout[0]);
                body::render(frame, layout[1], &mut self.table_state, &self.data);
                footer::render(frame, layout[2]);
                if self.show_filter_modal {
                    filter_modal::render(frame, &self.filter_form);
                }
            }
        }
    }

    fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.state {
                    AppState::Login => {
                        match login::handle_key(key.code, &mut self.login_form) {
                            login::LoginAction::Quit => return Ok(true),
                            login::LoginAction::Submit => {
                                let username = self.login_form.username.clone();
                                let password = self.login_form.password.clone();

                                match self.rt.block_on(authenticate(&username, &password)) {
                                    Ok(token) => {
                                        self.access_token = Some(token);
                                        self.state = AppState::Main;

                                        // Load initial data
                                        let initial_filter = FilterGraphql::default();
                                        let (data, total_count) = self.rt.block_on(fetch_meetup_url_data(&initial_filter, self.access_token.as_deref()));
                                        self.data = data;
                                        self.total_count = total_count;
                                        self.current_filter = initial_filter;
                                        self.table_state.select(Some(0));
                                    }
                                    Err(e) => {
                                        self.login_form.set_error(e);
                                    }
                                }
                            }
                            login::LoginAction::Continue => {}
                        }
                        return Ok(false);
                    }
                    AppState::Main => {
                        if self.show_filter_modal {
                            match filter_modal::handle_key(key.code, &mut self.filter_form) {
                                filter_modal::FilterModalAction::Cancel => {
                                    self.show_filter_modal = false;
                                    return Ok(false);
                                }
                                filter_modal::FilterModalAction::Apply => {
                                    let filter = self.filter_form.to_filter();
                                    let (data, total_count) = self.rt.block_on(fetch_meetup_url_data(&filter, self.access_token.as_deref()));
                                    self.data = data;
                                    self.total_count = total_count;
                                    self.current_filter = filter;
                                    self.table_state.select(Some(0));
                                    self.show_filter_modal = false;
                                    return Ok(false);
                                }
                                filter_modal::FilterModalAction::Noop => return Ok(false),
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(true),
                                KeyCode::Char('f') | KeyCode::Char('F') => {
                                    self.filter_form.set_from_filter(&self.current_filter);
                                    self.show_filter_modal = true;
                                    return Ok(false);
                                }
                                KeyCode::Down => {
                                    if !self.data.is_empty() {
                                        let i = self.table_state.selected().unwrap_or(0);
                                        let new_i = (i + 1).min(self.data.len() - 1);
                                        self.table_state.select(Some(new_i));
                                    }
                                    return Ok(false);
                                }
                                KeyCode::Up => {
                                    if !self.data.is_empty() {
                                        let i = self.table_state.selected().unwrap_or(0);
                                        let new_i = i.saturating_sub(1);
                                        self.table_state.select(Some(new_i));
                                    }
                                    return Ok(false);
                                }
                                _ => return Ok(false),
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }
}