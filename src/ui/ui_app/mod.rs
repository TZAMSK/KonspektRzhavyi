use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn display_app(frame: &mut Frame, main_zone: ratatui::prelude::Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 1)].as_ref())
        .split(main_zone);

    let test = Paragraph::new("TEST")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));

    frame.render_widget(test, main_layout[0]);
}
