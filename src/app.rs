use ratatui::layout::Rect;

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub target_text: String,
    pub curr_text: String,
    pub rect: Rect,
    pub scroller: bool,
    pub cursor_pos: (u16, u16),
}

impl App {
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
}
