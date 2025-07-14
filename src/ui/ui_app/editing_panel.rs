use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{app::App, ui::constants::NOTHING_HERE};

pub fn editing_panel(app: &App, rect: Rect, frame: &mut Frame) {
    let content = app.entries.read_file();

    match content.as_ref() {
        "Nothing here" => show_placeholder(frame, rect),
        _ => show_file_content(content, frame, rect),
    }
}

pub fn show_placeholder(frame: &mut Frame, rect: Rect) {
    frame.render_widget(Paragraph::new(NOTHING_HERE), rect)
}

pub fn show_file_content(content: String, frame: &mut Frame, rect: Rect) {
    frame.render_widget(Paragraph::new(content), rect)
}
