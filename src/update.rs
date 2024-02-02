use crossterm::event::{KeyCode, Event, self, KeyModifiers};
use crate::{app::App, tui::Tui};
use std::io::Result;

pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {

        if 
                key.modifiers == KeyModifiers::ALT &&
                key.code == KeyCode::Char('q') ||
                key.code == KeyCode::Esc
        {
            app.exit = true
        }

        if key.code == KeyCode::Char('h') && key.modifiers == KeyModifiers::CONTROL {
            app.del_last_word()
        } else {
            match key.code {
                KeyCode::Char(char) => app.curr_text.push(char),
                KeyCode::Backspace => {
                    app.curr_text.pop(); 
                },

                _ => ()
            }
        }
    }
    Ok(())
}
