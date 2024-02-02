use crossterm::event::{KeyCode, Event, self, KeyModifiers};
use crate::{app::App, tui::Tui};
use std::io::Result;

pub fn update(app: &mut App, tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {

        if (
                key.modifiers == KeyModifiers::ALT &&
                key.code == KeyCode::Char('q')
            ) ||
                key.code == KeyCode::Esc
        {
            app.exit = true
        }

        match key.code {
            KeyCode::Char(char) => app.curr_text.push(char),
            KeyCode::Backspace => {
                if key.modifiers == KeyModifiers::CONTROL {
                    app.curr_text.clear();
                } else {
                    app.curr_text.pop(); 
                }
            },

            _ => ()
        }

    }
    Ok(())
}
