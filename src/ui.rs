use std::time::Duration;
use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    app.rect = create_rect(frame.size(), app.scroller, app.is_finished_typing());

    if app.scroller {
        // centering of the txt
        let filler_len = app.target_text.chars().take_while(|c| *c == ' ').count();
        if filler_len != app.rect.width as usize / 2 {
            app.curr_text.drain(0..filler_len);
            app.target_text.drain(0..filler_len);
            let filler = std::iter::repeat(' ').take(app.rect.width as usize / 2).collect::<String>();
            app.curr_text.insert_str(0, &filler);
            app.target_text.insert_str(0, &filler);
        }

        frame.set_cursor(app.rect.x + app.rect.width / 2, app.rect.y);
    } else {
        app.update_cursor(frame);
    }

    let chars: Vec<Span> = {
        app.target_text.chars().enumerate().map(|(i, target_c)| {
            if let Some(c) = app.curr_text.chars().nth(i) {
                if c == target_c {
                    target_c.to_string().white()
                } else {
                    target_c.to_string().light_red()
                }
            } else {
                target_c.to_string().fg(Color::Indexed(244))
            }
        }).collect()
    };

    match app.is_finished_typing() || app.timer.is_out_of_time() {
        true => render_stats(app, frame),
        false => match app.scroller {
            true => render_scroller(app, frame, chars),
            false => render_wrapped(app, frame, chars)
        }
    }

    if app.timer.is_started() {
        render_timer(app, frame);
    }
}

fn render_timer(app: &App, frame: &mut Frame) {
    if
        std::env::args().find(|i| i == "-t").is_none()
            && app.timer.get_elapsed() > Duration::from_secs(3)
    {
        return;
    }

    if !app.is_finished_typing() {
        let area = Rect { y: app.rect.y - 2, ..app.rect };

        frame.render_widget(
            Paragraph::new(app.timer.get_remaining().to_string()),
            area
        )
    }
}

fn render_stats(app: &App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(
            format!(
                "WPM: {:.0}\n\nAccuracy: {:.2}\ncorrect: {}\nincorrect: {}\nwords: {}\n\nTime: {}s\n\n\n\n\n\n\n TAB / ALT + n for next test, ALT + r to retry test",
                    app.get_wpm(),
                    app.get_accuracy(),
                    app.get_correct(),
                    app.get_incorrect(),
                    app.target_text.split_whitespace().count(),
                    app.timer.get_time().as_secs()
            )).alignment(Alignment::Center),
        app.rect
    )
}

fn render_wrapped(app: &App, frame: &mut Frame, chars: Vec<Span>) {
    frame.render_widget(
        Paragraph::new(Line::from(chars))
            .wrap(Wrap::default()),
        app.rect
    );
}

fn render_scroller(app: &App, frame: &mut Frame, chars: Vec<Span>) {
    frame.render_widget(
        Paragraph::new(Line::from(chars))
            .scroll((0, (app.curr_text.len() as u16).saturating_sub(app.rect.width / 2))),
        app.rect
    );
}

pub fn create_rect(frame_rect: Rect, scroll: bool, is_finished_typing: bool) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ]).split(frame_rect);
    let r = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ]).split(vert[1])[1];

    if scroll && !is_finished_typing {
        Rect { y: r.y + r.height / 2, ..r}
    } else { r }
}

impl App {
    pub fn update_cursor(&self, frame: &mut ratatui::Frame) {
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
}
