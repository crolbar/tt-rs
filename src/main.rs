mod update;
mod tui;
mod app;
use app::App;
mod ui;
use anyhow::Result;
use std::time::Duration;
use crossterm::event::poll;

fn main() -> Result<()> {
    let (mut app, mut tui) = App::new()?;

    while !app.exit {
        tui.draw(&mut app)?;

        if poll(Duration::from_secs(2))? {
            app.update(&mut tui)?;
        }
    }

    tui.exit()?;
    Ok(())
}
