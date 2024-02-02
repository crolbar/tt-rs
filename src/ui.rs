use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    let rec = frame.size();
    frame.set_cursor(app.curr_text.len() as u16, rec.y);

    let chars: Vec<Span> = {
        app.target_text.chars().enumerate().map(|(i, target_c)| {
            if let Some(c) = app.curr_text.chars().nth(i) {
                    if c == target_c {
                        target_c.to_string().green()
                    } else {
                        if target_c == ' ' {
                            target_c.to_string().on_red()
                        } else {
                            target_c.to_string().red()
                        }
                    }
            } else {
                target_c.to_string().dark_gray()
            }
        }).collect()
    };

    frame.render_widget(
        Paragraph::new(Line::from(chars)),
        rec
    )
}
