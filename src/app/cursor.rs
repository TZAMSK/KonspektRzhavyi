pub struct Cursor {
    pub position: u8,
}

impl Default for Cursor {
    fn default() -> Self {
        Self { position: 0 }
    }
}

impl Cursor {
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
}
