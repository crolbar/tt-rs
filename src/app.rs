use ratatui::layout::Rect;
use std::{io::Result, time::{Duration, Instant}, usize};
use crate::tui::Tui;

pub struct App {
    pub exit: bool,
    pub target_text: String,
    pub curr_text: String,
    pub rect: Rect,
    pub scroller: bool,
    pub cursor_pos: (u16, u16),
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub timer_time: Duration,
    pub correct_chars: u32,
    pub incorrect_chars: u32,
}


impl App {
    pub fn new() -> Result<(Self, Tui)> {
        Ok((App::default(), Tui::enter()?))
    }

    pub fn del_last_word(&mut self) {
        if let Some(_) = self.curr_text.chars().rev().skip(1).find(|c| *c == ' ') {

            self.curr_text.truncate(
                if let Some((start_of_word, _)) = 
                    self.target_text.char_indices()
                        .take(self.curr_text.len())
                        .collect::<Vec<_>>().iter()
                        .rfind(|(_, c)| *c == ' ')
                {
                    if *start_of_word == self.curr_text.len() - 1 {
                        self.target_text.char_indices()
                            .take(self.curr_text.len())
                            .collect::<Vec<_>>().iter()
                            .rev().skip(1)
                            .find(|(_, c)| *c == ' ')
                            .unwrap_or(&(0,'a')).0 + 1 // FIX THIS
                    } else { start_of_word + 1 }
                }

                else {unreachable!()}
            )
        } else {
            self.curr_text.clear()
        }
    }

    pub fn del_whitespaces(&mut self) {
        if let Some((last_non_whitespace_index, _)) = self.curr_text.char_indices().rfind(|(_, c)| *c != ' ') {
            self.curr_text.truncate(last_non_whitespace_index + 1)
        } else { self.curr_text.clear() }
    }

    pub fn jump_to_next_word(&mut self) {
        if self.curr_text.chars().last() != Some(' ') {
            if let Some(next_whitespace) = self.target_text
                .chars().enumerate()
                .position(|(i, c)| c == ' ' && i >= self.curr_text.len())
            {
                self.curr_text.push_str(
                    &std::iter::repeat(' ').take(
                         (next_whitespace + 1) - self.curr_text.len()
                    ).collect::<String>()
                )
            }
        }
    }

    pub fn update_cursor(&mut self) {
        self.cursor_pos = (self.rect.x, self.rect.y);

        if self.curr_text.len() != 0 {
                let mut curr_line_width = self.rect.width as usize;
                let mut last_line_width = 0;

                loop {
                    if curr_line_width < self.target_text.len() {
                        // check if the char at index rect.width is an whitespace 
                        if self.target_text.chars().nth(curr_line_width - 1).unwrap() != ' ' {
                            let whitespace_before_word = self.target_text
                                .split_at(curr_line_width).0
                                .rsplit_once(' ').unwrap().0
                            .len() + 1;

                            let word_lenght = self.target_text
                                .split_at(whitespace_before_word).1
                                .split_once(' ').unwrap().0
                            .len();

                            if curr_line_width - whitespace_before_word < word_lenght {
                                curr_line_width = whitespace_before_word
                            } else { curr_line_width += 1 } // again because we want curr_line_width to index the whitespace
                        }

                        // if at the next line
                        if self.curr_text.len() >= curr_line_width {
                            self.cursor_pos.1 += 1;

                            last_line_width = curr_line_width;
                            curr_line_width += self.rect.width as usize;
                        } else { break }
                    } else { break }
                }

            self.cursor_pos.0 = ((self.curr_text.len() - last_line_width) as u16) + self.rect.x;
        }
    }

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }
    
    pub fn is_out_of_time(&self) -> bool {
        if let Some(st) = self.start_time {
            st.elapsed() >= self.timer_time
        } else { false }
    }
    
    pub fn is_finished_typing(&self) -> bool {
        self.curr_text.len() == self.target_text.len()
    }

    pub fn check_is_char_corr(&mut self) {
        if self.curr_text.chars().last().unwrap() == self.target_text.chars().nth(self.curr_text.len() - 1).unwrap() {
            self.correct_chars += 1;
        } else {
            self.incorrect_chars += 1;
        }
    }

    pub fn restart(&mut self) {
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.curr_text.clear();
        self.start_time = None;
        self.end_time = None;

        if self.scroller {
            self.curr_text.insert_str(
                0, &std::iter::repeat(' ').take(self.rect.width as usize / 2).collect::<String>()
            )
        } else if let Some(o) = Some(self.target_text.chars().take_while(|i| *i == ' ').count()) {
            self.target_text.drain(0..o);
        }

    }

    pub fn get_wpm(&mut self) -> f64 {
        if self.end_time == None {
            self.end_time = Some(Instant::now())
        }

        self.curr_text.split_whitespace().count() as f64 / // FIX THIS (get correct words instead of just words)
            ((self.end_time.unwrap() - self.start_time.unwrap()).as_secs_f64() / 60.0)
    }
}

impl Default for App {
    fn default() -> Self {
        let args: Vec<String> = std::env::args().collect();

        Self {
            exit: false,
            scroller: false,
            start_time: None,
            end_time: None,
            cursor_pos: (0,0),
            curr_text: String::new(),
            target_text: String::new(),
            correct_chars: 0,
            incorrect_chars: 0,
            rect: Rect::default(),
            timer_time: 
                if let Some(time) = args.iter().position(|i| i == &"-t".to_string()) {
                    Duration::from_secs(
                        args.get(time + 1)
                        .unwrap_or_else(|| {
                            println!("add time after -t in secs (e.g: -t 30)");
                            std::process::exit(0)
                        }).parse().unwrap_or_else(|_| {
                            println!("incorrect duration: add time after -t in secs (e.g: -t 30)");
                            std::process::exit(0)
                        })
                    )
                } else { Duration::from_secs(1200) }
        }
    }
    
}
