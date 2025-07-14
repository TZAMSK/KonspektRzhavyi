use crate::{app::cursor::Cursor, entry::Entry};

pub mod cursor;

pub struct App {
    pub entries: Entry,
    pub cursor: Cursor,
}

impl App {
    pub fn init() -> Self {
        Self {
            entries: Entry::init(),
            cursor: Cursor::init(),
        }
    }

    pub fn go_to_path(&mut self) {
        self.entries
            .enter_directory(&self.entries.entries[self.cursor.position as usize].path());
    }
}
