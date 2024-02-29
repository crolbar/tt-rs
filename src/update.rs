use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crate::{app::App, tui::Tui};
use std::io::Result;

pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            if 
                    key.modifiers == KeyModifiers::ALT &&
                    key.code == KeyCode::Char('q') ||
                    key.code == KeyCode::Esc
            {
                app.exit = true
            }
            
            if key.modifiers == KeyModifiers::ALT {
                match key.code {
                    KeyCode::Char('s') => app.scroller = !app.scroller,
                    KeyCode::Char('r') => {
                        app.curr_text.clear();

                        if app.scroller {
                            app.curr_text.insert_str(
                                0, &std::iter::repeat(' ').take(app.rect.width as usize / 2).collect::<String>()
                            )
                        }
                    },
                    _ => ()
                }
            }

            else if 
                key.code == KeyCode::Char('h') && key.modifiers == KeyModifiers::CONTROL ||
                key.code == KeyCode::Backspace && key.modifiers == KeyModifiers::CONTROL
            {
                if !app.scroller  {
                    app.del_last_word()
                } else if app.curr_text.len() as u16 > app.rect.width / 2 {
                    app.del_last_word()
                }
            } else {
                match key.code {
                    KeyCode::Char(' ') => app.jump_to_next_word(),
                    KeyCode::Char(char) => {
                        if app.target_text.chars().nth(app.curr_text.len()) != Some(' ') {
                            app.curr_text.push(char) 
                        }
                    },
                    KeyCode::Backspace => {
                        if !app.scroller || app.curr_text.len() as u16 > app.rect.width / 2 {
                            if app.curr_text.chars().last() == Some(' ') {
                                app.del_whitespaces();
                            } else {
                                app.curr_text.pop();
                            }
                        }
                    },

                    _ => ()
                }
            }
        }
    }
    Ok(())
}
