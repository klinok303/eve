use core::cmp::min;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;
mod terminal;
mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    loc: (usize, usize),
    view: View,
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
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        };
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let (width, height) = terminal::size()?;
        let max_x = width.saturating_sub(1) as usize;
        let max_y = height.saturating_sub(1) as usize;
        
        let (x, y) = match key_code {
            KeyCode::Up => (self.loc.0, self.loc.1.saturating_sub(1)),
            KeyCode::Down => (self.loc.0, min(max_y, self.loc.1.saturating_add(1))),
            KeyCode::Left => (self.loc.0.saturating_sub(1), self.loc.1),
            KeyCode::Right => (min(max_x, self.loc.0.saturating_add(1)), self.loc.1),
            KeyCode::PageUp => (self.loc.0, 0),
            KeyCode::PageDown => (self.loc.0, max_y),
            KeyCode::Home => (0, self.loc.1),
            KeyCode::End => (max_x, self.loc.1),
            _ => return Ok(()),
        };

        self.loc = (x, y);
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        terminal::hide_cursor()?;
        terminal::move_cursor_to(0, 0)?;
        
        if self.should_quit {
            terminal::clear_screen()?;
            terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            terminal::move_cursor_to(self.loc.0 as u16, self.loc.1 as u16)?;
        }

        terminal::show_cursor()?;
        terminal::execute()?;
        Ok(())
    }
}
