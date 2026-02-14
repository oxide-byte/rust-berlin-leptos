mod components;
mod model;
mod graphql;
mod auth;

use color_eyre::Result;
use components::app::App;

// user@demo.com
// password

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}