use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyEventKind, KeyModifiers};
use crate::{app::App, tui::Tui};
use anyhow::Result;

impl App {
    fn handle_exit(&mut self, key: &KeyEvent) {
        if 
            (key.modifiers == KeyModifiers::ALT && key.code == KeyCode::Char('q')) ||
            (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')) ||
            key.code == KeyCode::Esc
        {
            self.exit();
        }
    }

    fn handle_mod_alt(&mut self, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('s') => {
                self.swap_mode();

                if !self.is_in_scroller_mode() {
                    self.target_text.drain(0..self.get_rect().width as usize / 2);
                    self.curr_text.drain(0..self.get_rect().width as usize / 2);
                }
            },
            KeyCode::Char('r') => self.restart_test()?,
            KeyCode::Char('n') => self.next_test()?,
            _ => ()
        }

        Ok(())
    }

    fn handle_mod_ctrl(&mut self, key: &KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('h') |
            KeyCode::Char('w') |
            KeyCode::Backspace =>  {
                if self.is_in_scroller_mode() && self.curr_text.len() as u16 <= self.get_rect().width / 2 {
                    return Ok(())
                }

                if self.is_finished_typing() || self.timer.is_out_of_time() {
                    return Ok(())
                }

                self.del_last_word()
            }

            _ => ()
        }

        Ok(())
    }

    fn handle_mods(&mut self, key: &KeyEvent) -> Result<()> {
        match key.modifiers {
            KeyModifiers::ALT => self.handle_mod_alt(key),
            KeyModifiers::CONTROL => self.handle_mod_ctrl(key),
            _ => Ok(())
        }
    }

    fn handle_char_input(&mut self, char: char) -> Result<()> {
        if !self.timer.is_started() {
            self.timer.start();
        }

        if self.is_finished_typing() || self.timer.is_out_of_time() {
            return Ok(())
        } 

        if self.target_text.len() == self.curr_text.len() {
            return Ok(())
        }

        if self.target_text[self.curr_text.len()] == ' ' {
            return Ok(())
        }

        self.curr_text.push(char);
        self.check_is_char_corr()?;

        if self.is_finished_typing() && !self.timer.is_stopped() {
            self.timer.stop();
        }

        Ok(())
    }

    fn handle_backspace(&mut self) {
        if self.is_in_scroller_mode() && self.curr_text.len() as u16 <= self.get_rect().width / 2 {
            return;
        }

        if self.is_finished_typing() || self.timer.is_out_of_time() {
            return;
        }

        if self.curr_text.get(self.curr_text.len().saturating_sub(1)) == Some(&' ') {
            self.del_whitespaces();

            return;
        }

        self.curr_text.pop();
    }

    fn handle_key_event(&mut self, key: &KeyEvent) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(())
        }

        self.handle_exit(&key);

        if !key.modifiers.is_empty() && key.modifiers != KeyModifiers::SHIFT {
            self.handle_mods(&key)?;
            return Ok(())
        }

        match key.code {
            KeyCode::Tab => self.next_test()?,
            KeyCode::Char(' ') => self.jump_to_next_word(),
            KeyCode::Char(char) => self.handle_char_input(char)?,
            KeyCode::Backspace => self.handle_backspace(),
            _ => ()
        }

        Ok(())
    }

    pub fn update(&mut self, _tui: &mut Tui) -> Result<()> {
        match event::read()? {
            Event::Key(key) => self.handle_key_event(&key)?,
            _ => ()
        }

        Ok(())
    }
}
