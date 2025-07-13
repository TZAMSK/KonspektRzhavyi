use std::rc::Rc;

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn layout_based_on_entries(x: u16, rect: Rc<[Rect]>) -> Rc<[Rect]> {
    let constraint_entries = vec![Constraint::Percentage(100 / x); x as usize];

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraint_entries)
        .split(rect[0])
}
