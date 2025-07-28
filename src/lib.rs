use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;
mod terminal;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        terminal::initialize().unwrap();
        let result = self.repl();
        terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        terminal::hide_cursor()?;
        
        if self.should_quit {
            terminal::clear_screen()?;
            terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            terminal::move_cursor_to(0, 0)?;
        }

        terminal::show_cursor()?;
        terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let (height, _) = terminal::size()?;
        
        for current_row in 0..height {
            terminal::clear_line()?;
            terminal::print("~")?;

            if current_row + 1 < height {
                terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}
