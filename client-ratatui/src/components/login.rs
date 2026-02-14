use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub active_field: LoginField,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginField {
    Username,
    Password,
}

pub enum LoginAction {
    Continue,
    Submit,
    Quit,
}

impl Default for LoginForm {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            active_field: LoginField::Username,
            error_message: None,
        }
    }
}

impl LoginForm {
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}

pub fn render(frame: &mut Frame, login_form: &LoginForm) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Login to Rust Berlin")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).bold());
    frame.render_widget(title, layout[1]);

    // Username field
    let username_block = Block::default()
        .borders(Borders::ALL)
        .title("Username")
        .border_style(if login_form.active_field == LoginField::Username {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    let username_text = Paragraph::new(login_form.username.as_str()).block(username_block);
    frame.render_widget(username_text, layout[2]);

    // Password field
    let password_block = Block::default()
        .borders(Borders::ALL)
        .title("Password")
        .border_style(if login_form.active_field == LoginField::Password {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    let password_display = "*".repeat(login_form.password.len());
    let password_text = Paragraph::new(password_display).block(password_block);
    frame.render_widget(password_text, layout[3]);

    // Instructions or error
    let info_text = if let Some(ref error) = login_form.error_message {
        Paragraph::new(error.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red))
    } else {
        Paragraph::new("Tab: Switch field | Enter: Login | Esc/Q: Quit")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray))
    };
    frame.render_widget(info_text, layout[4]);
}

pub fn handle_key(key: KeyCode, login_form: &mut LoginForm) -> LoginAction {
    login_form.clear_error();

    match key {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => LoginAction::Quit,
        KeyCode::Tab => {
            login_form.active_field = match login_form.active_field {
                LoginField::Username => LoginField::Password,
                LoginField::Password => LoginField::Username,
            };
            LoginAction::Continue
        }
        KeyCode::Enter => LoginAction::Submit,
        KeyCode::Backspace => {
            match login_form.active_field {
                LoginField::Username => {
                    login_form.username.pop();
                }
                LoginField::Password => {
                    login_form.password.pop();
                }
            }
            LoginAction::Continue
        }
        KeyCode::Char(c) => {
            match login_form.active_field {
                LoginField::Username => login_form.username.push(c),
                LoginField::Password => login_form.password.push(c),
            }
            LoginAction::Continue
        }
        _ => LoginAction::Continue,
    }
}