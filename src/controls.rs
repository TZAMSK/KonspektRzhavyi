use std::error::Error;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

pub fn keyboard_event() -> AppResult<()> {
    let event = event::read()?;

    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            return match key.code {
                event::KeyCode::Char('q') => return Err("error".to_string().into()),
                _ => Ok(()),
            };
        }
    }

    Ok(())
}
