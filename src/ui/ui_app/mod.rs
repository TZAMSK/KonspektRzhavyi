mod generation;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::{
    file::{count_entries, entries},
    ui::ui_app::generation::layout_based_on_entries,
};

pub fn display_app(frame: &mut Frame, main_zone: ratatui::prelude::Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
        .split(main_zone);

    for (index, entry) in entries().iter().enumerate() {
        frame.render_widget(
            Paragraph::new(entry.path().to_string_lossy())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Red)),
            layout_based_on_entries(count_entries(), main_layout.clone())[index],
        );
    }
}
