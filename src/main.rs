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

    app.target_text = "hello how are".to_string();

    while !app.exit {
        tui.draw(&mut app)?;
        if poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
        if app.curr_text == app.target_text {
            app.exit = true
        }
    }

    tui.exit()?;
    Ok(())
}
