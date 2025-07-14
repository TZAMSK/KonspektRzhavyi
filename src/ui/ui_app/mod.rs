mod editing_panel;
mod navigation_panel;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    app::App,
    ui::ui_app::{editing_panel::editing_panel, navigation_panel::navigation_panel},
};

pub fn display_app(frame: &mut Frame, app: &App, main_zone: ratatui::prelude::Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(main_zone);

    navigation_panel(app, main_layout[0], frame);

    editing_panel(app, main_layout[1], frame);
}
