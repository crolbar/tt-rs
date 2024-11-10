use std::env::args;
use anyhow::Result;
use ratatui::layout::Rect;
use crate::tui::Tui;
use crate::timer::Timer;
use crate::util::get_prev_whitespace;

pub struct App {
    exit: bool,
    pub target_text: Vec<char>,
    pub curr_text: String,
    rect: Rect,
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
            .iter().enumerate().skip(self.curr_text.len())
                .find(|(_, &c)| c == ' ');

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

            let mut prev_whitespace_idx = crate::util::get_prev_whitespace(&self.target_text, self.target_text.len() - 1);
            if self.target_text[prev_whitespace_idx] == ' ' {
                prev_whitespace_idx += 1;
            }
            let last_target_word: String = self.target_text[prev_whitespace_idx..].iter().collect();

            return last_curr_word == last_target_word;
        }

        false
    }

    pub fn check_is_char_corr(&mut self) -> Result<()> {
        let last_curr_char = self.curr_text.chars().last().unwrap();
        let last_target_char = self.target_text[self.curr_text.len() - 1];

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

    fn gen_target_text(args: &Vec<String>) -> Result<Vec<char>> {
        match args.contains(&"-q".to_string()) {
            true => crate::util::get_random_quotes(),
            false => crate::util::get_random_words(args)
        }
    }

    pub fn gen_scroller_filter(&self) -> Vec<char> {
        std::iter::repeat(' ')
            .take(self.rect.width as usize / 2)
            .collect()
    }

    pub fn next_test(&mut self) -> Result<()> {
        let args: Vec<String> = args().collect();
        self.target_text = App::gen_target_text(&args)?;

        self.restart_test()?;

        if self.scroller {
            self.target_text.splice(0..0, self.gen_scroller_filter());
        }

        Ok(())
    }

    pub fn restart_test(&mut self) -> Result<()> {
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.curr_text.clear();
        self.timer.reset();

        if self.scroller {
            self.curr_text = self.gen_scroller_filter().iter().collect();
        }

        Ok(())
    }

    pub fn get_wpm(&self) -> f64 {
        let target_txt_str = self.target_text.iter().collect::<String>();
        let target_words: Vec<&str> = target_txt_str.split_whitespace().collect();

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

    pub fn update_rect(&mut self, frame_rect: Rect) {
        let xperc: f32;
        let yperc: f32;

        if frame_rect.height < 15 {
            yperc = 0.20
        } else {
            yperc = 0.35f32
        }

        if frame_rect.width < 80 {
            xperc = 0.05
        } else {
            xperc = 0.25f32;
        }

        let x = (frame_rect.width as f32 * xperc) as u16;
        let y: u16;
        let width = frame_rect.width - x * 2;
        let height: u16;

        if self.scroller && !self.is_finished_typing() && !self.timer.is_out_of_time() {
            y = frame_rect.height / 2;
            height = 1;
        } else {
            y = (frame_rect.height as f32 * yperc) as u16;
            height = frame_rect.height - y * 2;
        }

        self.rect = Rect { x, y, width, height };
    }

    pub fn get_rect(&self) -> Rect { self.rect }

    pub fn update_cursor(&mut self, frame: &mut ratatui::Frame) {
        if self.scroller {
            frame.set_cursor(self.rect.x + self.rect.width / 2, self.rect.y);
            return;
        }

        let (x, y) = get_xy_wrapped(&self.curr_text, &self.target_text, self.rect);
        frame.set_cursor(x, y)
    }

    // used in scroller mode to center text with 0 x scroll
    pub fn adjust_filler_txt(&mut self) {
        let filler_len = self.target_text
            .iter()
            .take_while(|&c| *c == ' ')
            .count();

        let needed_filler_len = self.get_rect().width as usize / 2;

        if filler_len > needed_filler_len { // screen width deincreased
            self.curr_text.drain(0..filler_len - needed_filler_len);
            self.target_text.drain(0..filler_len - needed_filler_len);

        } else if filler_len < needed_filler_len { // screen width increased
            let filler = std::iter::repeat(' ')
                .take(needed_filler_len - filler_len)
                .collect::<Vec<char>>();

            let tmp: String = filler.iter().map(|c| *c).collect();
            self.curr_text.insert_str(0, &tmp);
            self.target_text.splice(0..0, filler);
        }
    }
}

pub fn get_xy_wrapped(curr_text: &String, target_text: &Vec<char>, rect: Rect) -> (u16, u16) {
    let mut num_rows = rect.y;

    if curr_text.len() != 0 {
        let mut curr_line_width = rect.width as usize;
        let mut last_line_width = 0;

        loop {
            if curr_line_width < target_text.len() {
                // check if the char at index rect.width is an whitespace 
                if target_text[curr_line_width - 1] != ' ' {

                    let whitespace_before_word = crate::util::get_prev_whitespace(target_text, curr_line_width) + 1;
                    let word_end_idx = crate::util::get_next_whitespace(target_text, curr_line_width);

                    let word_length = word_end_idx - whitespace_before_word;

                    if curr_line_width - whitespace_before_word < word_length {
                        curr_line_width = whitespace_before_word
                    } else { curr_line_width += 1 } // if the curr_line_width indexes the end char of an word we +1
                }                                   // so curr_line_width indexes the whitespace

                // if at the next line
                if curr_text.len() >= curr_line_width {
                    num_rows += 1;

                    last_line_width = curr_line_width;
                    curr_line_width += rect.width as usize;
                } else { break }
            } else { break }
        }

        return (((curr_text.len() - last_line_width) as u16) + rect.x, num_rows)
    }

    return (rect.x, rect.y);
}
