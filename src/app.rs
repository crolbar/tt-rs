use std::{env::{self, args}, fs::File, time::{Duration, Instant}};
use anyhow::{Context, Result};
use rand::{seq::SliceRandom, thread_rng, Rng};
use ratatui::layout::Rect;
use ron::de::from_reader;
use crate::tui::Tui;

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
                target_text: match args.contains(&"-q".to_string()) {
                    true => App::get_random_quotes()?,
                    false => App::get_random_words(args)?,
                },
                timer_time
            },
            Tui::enter()?
        ))
    }

    pub fn get_random_quotes() -> Result<String> {
        let file_path = format!("{}/.config/tt-rs/quotes.ron", env::var("HOME")?);
        
        let conts: Vec<String> = from_reader(
            File::open(file_path).with_context(|| "quotes.ron(~/.config/tt-rs/quotes.ron) file is incorrect or missing")?
        )?;

        Ok(conts[thread_rng().gen_range(0..conts.len())].clone())
    }

    pub fn get_random_words(args: Vec<String>) -> Result<String> {
        let file_path = format!("{}/.config/tt-rs/words.ron", env::var("HOME")?);

        let mut conts: Vec<String> = from_reader(
            File::open(file_path).with_context(|| "words.ron(~/.config/tt-rs/words.ron) file is incorrect or missing")?
        )?;

        let mut rng = thread_rng();

        conts.shuffle(&mut rng);

        let txt_len = {
            if let Some(length) = args.iter().position(|i| i == &"-w".to_string()) {
                args.get(length + 1)
                        .with_context(|| "add number after -w (e.g: -w 30)")?
                    .parse()
                        .with_context(|| "incorrect number: add number after -w (e.g: -w 30)")?
            } else { 25 }
        };

        Ok(conts[..txt_len].join(" "))
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
                        if let Some(previous_whitespace) = 
                            self.target_text.char_indices()
                                .take(self.curr_text.len())
                                .collect::<Vec<_>>().iter()
                                .rev().skip(1)
                                .find(|(_, c)| *c == ' ')
                        {
                            previous_whitespace.0 + 1
                        } else { 0 }
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

    pub fn update_cursor(&mut self, frame: &mut ratatui::Frame) {
        let mut num_rows = self.rect.y;

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
                                .split_once(' ').unwrap_or((&self.target_text, "")).0
                            .len();

                            if curr_line_width - whitespace_before_word < word_lenght {
                                curr_line_width = whitespace_before_word
                            } else { curr_line_width += 1 } // if the curr_line_width indexes the end char of an word we +1
                        }                                   // so curr_line_width indexes the whitespace

                        // if at the next line
                        if self.curr_text.len() >= curr_line_width {
                            num_rows += 1;

                            last_line_width = curr_line_width;
                            curr_line_width += self.rect.width as usize;
                        } else { break }
                    } else { break }
                }

            frame.set_cursor(((self.curr_text.len() - last_line_width) as u16) + self.rect.x, num_rows)
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

    pub fn restart(&mut self, reset_txt: bool) -> Result<()> {
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.curr_text.clear();
        self.start_time = None;
        self.end_time = None;

        let args: Vec<String> = args().collect();

        if reset_txt {
            self.target_text = 
                match args.contains(&"-q".to_string()) {
                    true => App::get_random_quotes()?,
                    false => App::get_random_words(args)?
                }
        }

        if self.scroller {
            let filler = std::iter::repeat(' ').take(self.rect.width as usize / 2).collect::<String>();
            self.curr_text.insert_str(0, &filler);
            self.target_text.insert_str(0, &filler);
        } else if let Some(o) = Some(self.target_text.chars().take_while(|i| *i == ' ').count()) {
            self.target_text.drain(0..o);
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

    pub fn exit_if_error(&mut self) -> Result<()> {
        if std::env::args().find(|i|i == "-d").is_some() {
            if self.incorrect_chars > 0 {
                self.restart(true)?;
            }
        };
        Ok(())
    }
}
