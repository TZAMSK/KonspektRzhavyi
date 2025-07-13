use crate::{app::cursor::Cursor, entry::Entry};

pub mod cursor;

pub struct App {
    pub entries: Entry,
    pub cursor: Cursor,
}

impl Default for App {
    fn default() -> Self {
        Self {
            entries: Entry::default(),
            cursor: Cursor::default(),
        }
    }
}

impl App {
    pub fn go_to_path(&mut self) {
        self.entries
            .enter_directory(&self.entries.entries[self.cursor.position as usize].path());
    }
}
