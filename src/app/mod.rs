use crate::{app::cursor::Cursor, entry::Entry};

pub mod cursor;

#[derive(PartialEq)]
pub enum Mode {
    Edit,
    Navigate,
}

pub struct App {
    pub entries: Entry,
    pub cursor: Cursor,
    pub mode: Mode,
    pub editing: bool,
}

impl App {
    pub fn init() -> Self {
        Self {
            entries: Entry::init(),
            cursor: Cursor::init(),
            mode: Mode::Navigate,
            editing: true,
        }
    }

    pub fn go_to_path(&mut self) {
        self.entries
            .enter_directory(&self.entries.entries[self.cursor.position as usize].path());
    }

    pub fn is_editing(self) -> bool {
        if self.mode == Mode::Edit {
            return true;
        }

        true
    }

    pub fn edit(&mut self) {
        if self.entries.current_file.is_some() {
            self.mode = Mode::Edit
        }
    }
}
