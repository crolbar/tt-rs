use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    render_scroller(app, frame);
}

fn render_scroller(app: &mut App, frame: &mut Frame) {
    app.rect = create_rect(frame.size(), app.scroller);

    let filler_len = app.curr_text.chars().take_while(|c| *c == ' ').count();
    if filler_len != app.rect.width as usize / 2 {
        app.curr_text.drain(0..filler_len);
        app.target_text.drain(0..filler_len);
        let filler = std::iter::repeat(' ').take(app.rect.width as usize / 2).collect::<String>();
        app.curr_text.insert_str(0, &filler);
        app.target_text.insert_str(0, &filler);
    }

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

    frame.set_cursor(app.rect.x + app.rect.width / 2, app.rect.y);

    frame.render_widget(
        Paragraph::new(Line::from(chars))
            .scroll((0, (app.curr_text.len() as u16).saturating_sub(app.rect.width / 2))),
        app.rect
    );
}

pub fn create_rect(frame_rect: Rect, scroll: bool) -> Rect {
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

    if scroll {
        Rect { y: r.y + r.height / 2, ..r}
    } else { r }
}
