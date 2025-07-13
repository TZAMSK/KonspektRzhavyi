use std::error::Error;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::{app::App, file::count_entries};

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

pub fn keyboard_event(app: &mut App) -> AppResult<()> {
    let event = event::read()?;

    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            return match key.code {
                event::KeyCode::Char('q') => return Err("error".to_string().into()),
                event::KeyCode::Up => Ok(app.cursor.move_cursor_up(count_entries() as u8)),
                event::KeyCode::Down => Ok(app.cursor.move_cursor_down(count_entries() as u8)),
                _ => Ok(()),
            };
        }
    }

    Ok(())
}
