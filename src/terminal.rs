use core::fmt::Display;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    Command,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Error, Write, stdout};

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()?;
    execute()?;
    Ok(())
}

pub fn terminate() -> Result<(), Error> {
    execute()?;
    disable_raw_mode()?;
    Ok(())
}

pub fn clear_screen() -> Result<(), Error> {
    queue_command(Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_line() -> Result<(), Error> {
    queue_command(Clear(ClearType::CurrentLine))?;
    Ok(())
}

pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
    queue_command(MoveTo(x, y))?;
    Ok(())
}

pub fn hide_cursor() -> Result<(), Error> {
    queue_command(Hide)?;
    Ok(())
}

pub fn show_cursor() -> Result<(), Error> {
    queue_command(Show)?;
    Ok(())
}

pub fn print<T: Display>(string: T) -> Result<(), Error> {
    queue_command(Print(string))?;
    Ok(())
}

pub fn size() -> Result<(u16, u16), Error> {
    crossterm::terminal::size()
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()?;
    Ok(())
}

fn queue_command<T: Command>(command: T) -> Result<(), Error> {
    queue!(stdout(), command)?;
    Ok(())
}
