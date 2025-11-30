use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

pub fn render(frame: &mut Frame, area: Rect) {
    let title = r#"______          _              _   _            _               _                                     ______           _ _
| ___ \        | |            | | | |          | |      ___    | |                                    | ___ \         | (_)
| |_/ /   _ ___| |_   ______  | |_| | __ _  ___| | __  ( _ )   | |     ___  __ _ _ __ _ __    ______  | |_/ / ___ _ __| |_ _ __
|    / | | / __| __| |______| |  _  |/ _` |/ __| |/ /  / _ \/\ | |    / _ \/ _` | '__| '_ \  |______| | ___ \/ _ \ '__| | | '_ \
| |\ \ |_| \__ \ |_           | | | | (_| | (__|   <  | (_>  < | |___|  __/ (_| | |  | | | |          | |_/ /  __/ |  | | | | | |
\_| \_\__,_|___/\__|          \_| |_/\__,_|\___|_|\_\  \___/\/ \_____/\___|\__,_|_|  |_| |_|          \____/ \___|_|  |_|_|_| |_|"#;

    frame.render_widget(
        Paragraph::new(title)
            .block(Block::new().padding(Padding::left(2)).borders(Borders::ALL)),
        area,
    );
}