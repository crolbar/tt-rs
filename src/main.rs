mod update;
mod tui;
mod app;
use app::App;
mod ui;
use update::update;
use std::io::Result;
use crossterm::event::poll;
use std::time::Duration;

fn main() -> Result<()> {
    let mut app = App::default();
    let mut tui = tui::Tui::enter()?;

    app.target_text = "place life only this late or before against".to_string();

    while !app.exit {
        tui.draw(&mut app)?;
        if poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
        if app.curr_text.len() == app.target_text.len() {
            app.exit = true;
        }
    }

    tui.exit()?;
    if app.curr_text == app.target_text {
        println!("corr");
    } else if app.curr_text.len() == app.target_text.len() {
        println!("wrong");
    }
    Ok(())
}
