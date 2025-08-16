use std::io::Error;
use const_format::formatcp;
use super::terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const WELCOME: &str = formatcp!("{NAME} editor -- version {VERSION}");

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>
}

#[derive(Default)]
pub struct View {
    buffer: Buffer
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let (_, height) = terminal::size()?;

        for current_row in 0..height {
            terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row as usize) {
                terminal::print(line)?;
                terminal::print("\r\n")?;
                continue;
            }
            
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                terminal::print("~")?;
            }

            if current_row.saturating_add(1) < height {
                terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let width = terminal::size()?.0 as usize;
        
        // Ensure we don't panic if WELCOME is longer than terminal width
        let welcome_len = WELCOME.len().min(width);
        let padding = width.saturating_sub(welcome_len) / 2;
        
        // Format directly without temporary String
        let spaces = " ".repeat(padding.saturating_sub(1));  // Prevent underflow
        terminal::print(format!("~{spaces}{:.welcome_len$}", WELCOME))?;
        Ok(())
    }
}
