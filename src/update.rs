use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyEventKind, KeyModifiers};
use crate::{app::App, tui::Tui};
use anyhow::Result;

impl App {
    fn handle_exit(&mut self, key: KeyEvent) {
        if 
            (key.modifiers == KeyModifiers::ALT && key.code == KeyCode::Char('q')) ||
            (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')) ||
            key.code == KeyCode::Esc
        {
            self.exit = true
        }
    }

    fn handle_mod_alt(&mut self, key: KeyEvent, tui: &mut Tui) -> Result<()> {
        match key.code {
            KeyCode::Char('s') => {
                self.scroller = !self.scroller;

                if !self.scroller {
                    self.target_text.drain(0..self.rect.width as usize / 2);
                    self.curr_text.drain(0..self.rect.width as usize / 2);
                } else {
                    tui.draw(self)?
                }
            },
            KeyCode::Char('r') => self.restart_test()?,
            KeyCode::Char('n') => self.next_test()?,
            _ => ()
        }

        Ok(())
    }

    fn handle_mod_ctrl(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('h') |
            KeyCode::Char('w') |
            KeyCode::Backspace =>  {
                if self.scroller && self.curr_text.len() as u16 <= self.rect.width / 2 {
                    return Ok(())
                }

                if self.is_finished_typing() || self.is_out_of_time() {
                    return Ok(())
                }

                self.del_last_word()
            }

            _ => ()
        }

        Ok(())
    }

    fn handle_mods(&mut self, key: KeyEvent, tui: &mut Tui) -> Result<()> {
        match key.modifiers {
            KeyModifiers::ALT => self.handle_mod_alt(key, tui),
            KeyModifiers::CONTROL => self.handle_mod_ctrl(key),
            _ => Ok(())
        }
    }

    fn handle_char_input(&mut self, char: char) -> Result<()> {
        if self.start_time == None {
            self.start_timer();
        }

        if self.target_text.chars().nth(self.curr_text.len()) == Some(' ') {
            return Ok(())
        }

        if self.is_finished_typing() || self.is_out_of_time() {
            return Ok(())
        } 

        if self.target_text.len() == self.curr_text.len() {
            return Ok(())
        }

        self.curr_text.push(char);
        self.check_is_char_corr()?;

        Ok(())
    }

    fn handle_backspace(&mut self) {
        if self.scroller && self.curr_text.len() as u16 <= self.rect.width / 2 {
            return;
        }

        if self.is_finished_typing() || self.is_out_of_time() {
            return;
        }

        if self.curr_text.chars().last() == Some(' ') {
            self.del_whitespaces();
        } else {
            self.curr_text.pop();
        }
    }

    pub fn update(&mut self, tui: &mut Tui) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(())
            }

            self.handle_exit(key);

            if !key.modifiers.is_empty() && key.modifiers != KeyModifiers::SHIFT {
                self.handle_mods(key, tui)?;
                return Ok(())
            }

            match key.code {
                KeyCode::Tab => self.restart_test()?,
                KeyCode::Char(' ') => self.jump_to_next_word(),
                KeyCode::Char(char) => self.handle_char_input(char)?,
                KeyCode::Backspace => self.handle_backspace(),
                _ => ()
            }
        }

        Ok(())
    }
}
