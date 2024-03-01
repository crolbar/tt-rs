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
    let (mut app, mut tui) = App::new()?;

    app.target_text = "place life only this late or before against goverment after mean this go behave place life only this late or before".to_string(); //against goverment after mean this go behave place life only this late or before against goverment after mean this go behave place life only this late or before against goverment after mean this go behave".to_string();

    while !app.exit {
        tui.draw(&mut app)?;

        if poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
    }

    tui.exit()?;
    Ok(())
}
