use std::time::Duration;
use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    app.update_rect(frame.size());

    if app.is_finished_typing() || app.timer.is_out_of_time() {
        render_stats(app, frame);
        return;
    }

    render_text(app, frame);

    if app.timer.is_started() {
        render_timer(app, frame);
    }
}

pub fn render_text(app: &mut App, frame: &mut Frame) {
    app.update_cursor(frame);

    match app.is_in_scroller_mode() {
        true => {
            app.adjust_filler_txt();
            render_scroller(app, frame, gen_chars(&app.target_text, &app.curr_text))
        },
        false => {
            render_wrapped(app, frame, gen_chars(&app.target_text, &app.curr_text))
        }
    }
}

fn gen_chars<'a>(target_text: &Vec<char>, curr_text: &'a String) -> Vec<Span<'a>> {
    target_text.iter().enumerate().map(|(i, &target_c)| {
        if let Some(c) = curr_text.chars().nth(i) {
            if c == target_c {
                target_c.to_string().white()
            } else {
                target_c.to_string().light_red()
            }
        } else {
            target_c.to_string().fg(Color::Indexed(244))
        }
    }).collect()
}

fn render_timer(app: &App, frame: &mut Frame) {
    if
        std::env::args().find(|i| i == "-t").is_none()
            && app.timer.get_elapsed() > Duration::from_secs(3)
    {
        return;
    }

    if !app.is_finished_typing() {
        let rect = app.get_rect();
        frame.render_widget(
            Paragraph::new(app.timer.get_remaining().to_string()),
            Rect { y: rect.y.saturating_sub(2), ..rect }
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
                    app.target_text.iter().filter(|&&c| c.is_whitespace()).count() + 1,
                    app.timer.get_time().as_secs()
            )).alignment(Alignment::Center),
        app.get_rect()
    )
}

fn render_wrapped(app: &App, frame: &mut Frame, chars: Vec<Span>) {
    frame.render_widget(
        Paragraph::new(Line::from(chars))
            .wrap(Wrap::default()),
        app.get_rect()
    );
}

fn render_scroller(app: &App, frame: &mut Frame, chars: Vec<Span>) {
    frame.render_widget(
        Paragraph::new(Line::from(chars))
            .scroll((0, (app.curr_text.len() as u16).saturating_sub(app.get_rect().width / 2))),
        app.get_rect()
    );
}
