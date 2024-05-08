use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crate::{app::App, tui::Tui};
use anyhow::Result;

pub fn update(app: &mut App, tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            if 
                    (key.modifiers == KeyModifiers::ALT && key.code == KeyCode::Char('q')) ||
                    (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')) ||
                    key.code == KeyCode::Esc
            {
                app.exit = true
            }
            
            if key.modifiers == KeyModifiers::ALT {
                match key.code {
                    KeyCode::Char('s') => {
                        app.scroller = !app.scroller;

                        if !app.scroller {
                            app.target_text.drain(0..app.rect.width as usize / 2);
                            app.curr_text.drain(0..app.rect.width as usize / 2);
                        } else {
                            tui.draw(app)?
                        }
                    },
                    KeyCode::Char('r') => app.restart(false)?,
                    KeyCode::Char('n') => app.restart(true)?,
                    _ => ()
                }
            }

            else if 
                key.code == KeyCode::Char('h') && key.modifiers == KeyModifiers::CONTROL ||
                key.code == KeyCode::Backspace && key.modifiers == KeyModifiers::CONTROL
            {
                if 
                    (!app.scroller ||
                    app.curr_text.len() as u16 > app.rect.width / 2) &&
                    !app.is_finished_typing() &&
                    !app.is_out_of_time()
                {
                    app.del_last_word()
                }
            } else {
                match key.code {
                    KeyCode::Tab => app.restart(true)?,
                    KeyCode::Char(' ') => app.jump_to_next_word(),
                    KeyCode::Char(char) => {
                        if app.start_time == None {
                            app.start_timer();
                        }

                        if 
                            app.target_text.chars().nth(app.curr_text.len()) != Some(' ') &&
                            !app.is_finished_typing() &&
                            !app.is_out_of_time() &&
                            app.target_text.len() != app.curr_text.len()
                        {
                            app.curr_text.push(char);
                            app.check_is_char_corr();
                        }
                    },
                    KeyCode::Backspace => {
                        if
                            (!app.scroller ||
                            app.curr_text.len() as u16 > app.rect.width / 2) &&
                            !app.is_finished_typing() &&
                            !app.is_out_of_time()
                        {
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
