use ratatui::layout::Rect;

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub target_text: String,
    pub curr_text: String,
    pub rect: Rect,
    pub scroller: bool,
}

impl App {
    pub fn del_last_word(&mut self) {
        if let Some(space_i) = self.curr_text.rfind(' ') {
            if space_i == self.curr_text.len() - 1 {
                self.curr_text.pop().unwrap();
                if let Some(i) = self.curr_text.rfind(' ') {
                    self.curr_text.truncate(i + 1);
                } else {
                    self.curr_text.clear()
                }
            } else {
                self.curr_text.truncate(space_i + 1)
            }
        } else {
            self.curr_text.clear()
        }
    }
}
