use crate::app::cursor::Cursor;

pub mod cursor;

pub struct App {
    pub cursor: Cursor,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cursor: Cursor::default(),
        }
    }
}
