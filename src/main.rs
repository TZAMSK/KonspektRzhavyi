mod app;
mod controls;
mod ui;

use std::error::Error;

use ratatui::DefaultTerminal;

use crate::{controls::keyboard_event, ui::ui_app::display_app};

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.area();
            display_app(f, size);
        })?;

        if let Err(_) = keyboard_event() {
            break Ok(());
        }
    }
}
