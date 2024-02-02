use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(app.target_text.clone().dark_gray()),
        frame.size()
    );

    let chars: Vec<Span> = {
        app.curr_text.chars().enumerate().map(|(i, c)| {
            if let Some(target_c) = app.target_text.chars().nth(i) {
                    if c ==  target_c {
                        c.to_string().green()
                    } else {
                        c.to_string().red()
                    }
            } else {
                c.to_string().red()
            }
        }).collect()
    };

    frame.render_widget(
        Paragraph::new(Line::from(chars)),
        frame.size()
    )
}
