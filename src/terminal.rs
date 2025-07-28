use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Error, Write, stdout};

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()?;
    move_cursor_to(0, 0)?;
    Ok(())
}

pub fn terminate() -> Result<(), Error> {
    disable_raw_mode()?;
    Ok(())
}

pub fn clear_screen() -> Result<(), Error> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line() -> Result<(), Error> {
    queue!(stdout(), Clear(ClearType::CurrentLine))?;
    Ok(())
}

pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
    execute!(stdout(), MoveTo(x, y))?;
    Ok(())
}

pub fn hide_cursor() -> Result<(), Error> {
    queue!(stdout(), Hide)?;
    Ok(())
}

pub fn show_cursor() -> Result<(), Error> {
    queue!(stdout(), Show)?;
    Ok(())
}

pub fn print(string: &str) -> Result<(), Error> {
    queue!(stdout(), Print(string))?;
    Ok(())
}

pub fn size() -> Result<(u16, u16), Error> {
    crossterm::terminal::size()
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()?;
    Ok(())
}
