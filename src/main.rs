mod update;
mod tui;
mod app;
mod util;
mod ui;
mod timer;
use app::App;
use anyhow::Result;
use std::time::Duration;
use crossterm::event::poll;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"-h".to_string()) 
        || args.contains(&"--help".to_string()) 
    {
        print_help();

        std::process::exit(0);
    }

    let (mut app, mut tui) = App::new(&args)?;

    while !app.should_exit() {
        tui.draw(&mut app)?;

        if poll(Duration::from_secs(2))? {
            app.update(&mut tui)?;
        }
    }

    tui.exit()?;
    Ok(())
}

fn print_help() {
    println!(
        "\
        tt-rs - tui typing test \n\n\
        -t <TIME>     Specify time for the timer in secs \
        \n-w <NUM>      Specify the number of words in the test \
        \n-q            Test contains quotes instead of words \
        \n-d            Each time you make an mistake the test will restart \
        "
    );
}
