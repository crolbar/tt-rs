use std::env::args;
use anyhow::Result;
use ratatui::layout::Rect;
use crate::tui::Tui;
use crate::timer::Timer;
use crate::util::get_prev_whitespace;

pub struct App {
    exit: bool,
    pub target_text: String,
    pub curr_text: String,
    pub rect: Rect,
    scroller: bool,
    pub timer: Timer,
    correct_chars: u32,
    incorrect_chars: u32,
}

impl App {
    pub fn new(args: &Vec<String>) -> Result<(Self, Tui)> {
        Ok((
            Self {
                exit: false,
                scroller: false,
                timer: Timer::new(&args)?,
                correct_chars: 0,
                incorrect_chars: 0,
                rect: Rect::default(),
                curr_text: String::new(),
                target_text: App::gen_target_text(&args)?,
            },
            Tui::enter()?
        ))
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn exit(&mut self)  { self.exit = true; }

    pub fn del_last_word(&mut self) {
        let word_start = get_prev_whitespace(
            &self.target_text,
            self.curr_text.len().saturating_sub(1)
        );

        self.curr_text.truncate(word_start + ((word_start != 0) as usize));
    }

    pub fn del_whitespaces(&mut self) {
        let last_non_whitespace = self
            .curr_text
            .char_indices()
            .rfind(|(_, c)| *c != ' ');

        if let Some((i, _)) = last_non_whitespace {
            self.curr_text.truncate(i + 1)
        } else {
            self.curr_text.clear() 
        }
    }

    pub fn jump_to_next_word(&mut self) {
        if self.curr_text.chars().last() == Some(' ') {
            return;
        }

        let next_whitespace_wrap = self.target_text
            .chars().enumerate().skip(self.curr_text.len())
                .find(|(_, c)| *c == ' ');

        if let Some((next_whitespace_idx, _)) = next_whitespace_wrap {
            let fill = std::iter::repeat(' ').take(
                (next_whitespace_idx + 1) - self.curr_text.len()
            ).collect::<String>();

            self.curr_text.push_str(&fill);
        }
    }

    pub fn is_finished_typing(&self) -> bool {
        if self.curr_text.len() == self.target_text.len() {
            let last_curr_word = self.curr_text.split_whitespace().last().unwrap();
            let last_target_word = self.target_text.split_whitespace().last().unwrap();

            return last_curr_word == last_target_word;
        }

        false
    }

    pub fn check_is_char_corr(&mut self) -> Result<()> {
        let last_curr_char = self.curr_text.chars().last().unwrap();
        let last_target_char = self.target_text.chars().nth(self.curr_text.len() - 1).unwrap();

        if last_curr_char == last_target_char {
            self.correct_chars += 1;
        } else {
            self.incorrect_chars += 1;

            if std::env::args().find(|i| i == "-d").is_some() {
                self.next_test()?;
            };
        }

        Ok(())
    }

    fn gen_target_text(args: &Vec<String>) -> Result<String> {
        match args.contains(&"-q".to_string()) {
            true => crate::util::get_random_quotes(),
            false => crate::util::get_random_words(args)
        }
    }

    pub fn gen_scroller_filter(&self) -> String {
        std::iter::repeat(' ')
            .take(self.rect.width as usize / 2)
            .collect()
    }

    pub fn next_test(&mut self) -> Result<()> {
        let args: Vec<String> = args().collect();
        self.target_text = App::gen_target_text(&args)?;

        self.restart_test()?;

        if self.scroller {
            self.target_text.insert_str(0, &self.gen_scroller_filter());
        }

        Ok(())
    }

    pub fn restart_test(&mut self) -> Result<()> {
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.curr_text.clear();
        self.timer.reset();

        if self.scroller {
            self.curr_text.insert_str(0, &self.gen_scroller_filter());
        }

        Ok(())
    }

    pub fn get_wpm(&self) -> f64 {
        let target_words: Vec<&str> = self.target_text.split_whitespace().collect();

        self.curr_text
            .split_whitespace()
            .enumerate()
            .filter(|(i, w)| *w == target_words[*i])
            .count() as f64
            / (self.timer.get_time().as_secs_f64() / 60.0)
    }

    pub fn get_accuracy(&self) -> f64 {
        (self.correct_chars as f64 / (self.correct_chars + self.incorrect_chars) as f64) * 100.0
    }

    pub fn get_correct(&self) -> u32 {
        self.correct_chars
    }

    pub fn get_incorrect(&self) -> u32 {
        self.incorrect_chars
    }

    pub fn is_in_scroller_mode(&self) -> bool {
        self.scroller
    }

    pub fn swap_mode(&mut self) {
        self.scroller = !self.scroller;
    }
}
