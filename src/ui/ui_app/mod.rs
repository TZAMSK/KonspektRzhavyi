mod generation;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, ui::ui_app::generation::layout_based_on_entries};

pub fn display_app(frame: &mut Frame, app: &App, main_zone: ratatui::prelude::Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(main_zone);

    for (index, entry) in app.entries.entries.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(entry.path().to_string_lossy())
                .alignment(Alignment::Center)
                .style(Style::default().fg(app.cursor.clone().color(index)))
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::QuadrantOutside),
                ),
            layout_based_on_entries(app.entries.count_entries(), main_layout.clone())[index],
        );
    }
}
