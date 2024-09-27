use std::{env::args, time::{Duration, Instant}};
use anyhow::{Context, Result};
use ratatui::layout::Rect;
use crate::tui::Tui;
use crate::util::get_prev_whitespace;

pub struct App {
    pub exit: bool,
    pub target_text: String,
    pub curr_text: String,
    pub rect: Rect,
    pub scroller: bool,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub timer_time: Duration,
    pub correct_chars: u32,
    pub incorrect_chars: u32,
}

impl App {
    pub fn new() -> Result<(Self, Tui)> {
        let args: Vec<String> = std::env::args().collect();

        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            println!(
                "\
                -t <TIME>     Specify time for the timer in secs \
                \n-w <NUM>      Specify the number of words in the test \
                \n-q            Test contains quotes instead of words \
                \n-d            Each time you make an mistake the test will restart \
                "
            );
            std::process::exit(0);
        }

        let timer_time = {
                if let Some(time) = args.iter().position(|i| i == &"-t".to_string()) {
                    Duration::from_secs(
                        args.get(time + 1)
                            .with_context(|| "add time after -t in secs (e.g: -t 30)")?
                        .parse()
                            .with_context(|| "incorrect duration: add time after -t in secs (e.g: -t 30)")?
                    )
                } else { Duration::from_secs(1200) }
        };

        Ok((
            Self {
                exit: false,
                scroller: false,
                start_time: None,
                end_time: None,
                correct_chars: 0,
                incorrect_chars: 0,
                rect: Rect::default(),
                curr_text: String::new(),
                target_text: App::gen_target_text(args)?,
                timer_time
            },
            Tui::enter()?
        ))
    }

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

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn is_out_of_time(&self) -> bool {
        if let Some(st) = self.start_time {
            return st.elapsed() >= self.timer_time
        }

        false
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

    fn gen_target_text(args: Vec<String>) -> Result<String> {
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
        self.target_text = App::gen_target_text(args)?;

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
        self.start_time = None;
        self.end_time = None;

        if self.scroller {
            self.curr_text.insert_str(0, &self.gen_scroller_filter());
        }

        Ok(())
    }

    pub fn get_wpm(&mut self) -> f64 {
        if self.end_time == None {
            self.end_time = Some(Instant::now())
        }

        let target_words: Vec<&str> = self.target_text.split_whitespace().collect();

        self.curr_text.split_whitespace().enumerate().filter(|(i, w)| *w == target_words[*i]).count() as f64 /
            ((self.end_time.unwrap() - self.start_time.unwrap_or(Instant::now())).as_secs_f64() / 60.0)
    }
}
