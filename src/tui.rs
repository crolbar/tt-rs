use crossterm::{cursor::SetCursorStyle, event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::{Stderr, Result, stderr};
use crate::ui::render;
use crate::app::App;

pub struct Tui {
    pub term: Terminal<CrosstermBackend<Stderr>>
}

impl Tui {
   pub fn enter() -> Result<Self> {
       enable_raw_mode()?;
       execute!(stderr(), EnterAlternateScreen, EnableMouseCapture, SetCursorStyle::SteadyBar)?;
       let mut term = Terminal::new(CrosstermBackend::new(stderr()))?;
       term.hide_cursor()?;
       term.clear()?;

       Ok(Tui{term})
   }

   pub fn draw(&mut self, app: &mut App) -> Result<()> {
       self.term.draw(|frame| render(app, frame))?;
       Ok(())
   }

   pub fn exit(&mut self) -> Result<()> {
       self.term.show_cursor()?;
       execute!(stderr(), LeaveAlternateScreen, DisableMouseCapture, SetCursorStyle::DefaultUserShape)?;
       disable_raw_mode()?;
       Ok(())
   }
}
