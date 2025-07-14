use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, ui::generation::layout_based_on_entries};

pub fn navigation_panel(app: &App, rect: Rect, frame: &mut Frame) {
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
            layout_based_on_entries(app.entries.count_entries(), rect)[index],
        );
    }
}
