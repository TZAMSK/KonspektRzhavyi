use ratatui::style::Color;

#[derive(Clone)]
pub struct Cursor {
    pub position: u8,
}

impl Cursor {
    pub fn init() -> Self {
        Self { position: 0 }
    }

    pub fn move_cursor_up(&mut self, length: u8) {
        if self.position > 0 {
            self.position -= 1;
        } else {
            self.position = length - 1
        }
    }

    pub fn move_cursor_down(&mut self, length: u8) {
        if self.position < length - 1 {
            self.position += 1;
        } else {
            self.position = 0
        }
    }

    pub fn color(self, index: usize) -> Color {
        if index == self.position.into() {
            Color::Red
        } else {
            Color::Gray
        }
    }
}
