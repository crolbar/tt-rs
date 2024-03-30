use std::time::{Duration, Instant};
use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    app.rect = create_rect(frame.size(), app.scroller);

    let chars: Vec<Span> = {
        app.target_text.chars().enumerate().map(|(i, target_c)| {
            if let Some(c) = app.curr_text.chars().nth(i) {
                    if c == target_c {
                        target_c.to_string().white()
                    } else {
                        target_c.to_string().light_red()
                    }
            } else {
                target_c.to_string().dark_gray()
            }
        }).collect()
    };

    match app.is_finished_typing() || app.is_out_of_time() {
        true => render_stats(app, frame),
        false => match app.scroller {
            true => render_scroller(app, frame, chars),
            false => render_wrapped(app, frame, chars)
        }
    }

    if let Some(start_time) = app.start_time {
        if ( // if there is an time specified show time remainning else show time for 2 secs
            std::env::args().find(|i| i == "-t").is_some() ||
            (app.timer_time == Duration::from_secs(1200) && start_time.elapsed() < Duration::from_secs(2))
           ) && !app.is_finished_typing() 
        {
            let area = Rect { y: app.rect.y - 2, ..app.rect };

            frame.render_widget(
                Paragraph::new(
                    format!("{}", 
                        app.timer_time.as_secs().saturating_sub(start_time.elapsed().as_secs())
                    )),
                area
            )
        } 
    }
}

fn render_stats(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(
            format!(
                "WPM: {:.0}\n\nAccuracy: {:.2}\ncorrect: {}\nincorrect: {}\n\nTime: {}s\n\n\n\n\n\n\n TAB / ALT + n for next test, ALT + r to retry test",
                    app.get_wpm(),
                    (app.correct_chars as f64 / (app.correct_chars + app.incorrect_chars) as f64) * 100.0,
                    app.correct_chars,
                    app.incorrect_chars,
                    (app.end_time.unwrap() - app.start_time.unwrap_or(Instant::now())).as_secs()
            )).alignment(Alignment::Center),
        app.rect
    )
}

fn render_wrapped(app: &mut App, frame: &mut Frame, chars: Vec<Span>) {
    app.update_cursor(frame);

    frame.render_widget(
        Paragraph::new(Line::from(chars))
            .wrap(Wrap::default()),
        app.rect
    );
}

fn render_scroller(app: &mut App, frame: &mut Frame, chars: Vec<Span>) {
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
