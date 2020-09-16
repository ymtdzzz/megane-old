extern crate clap;
extern crate tui;
extern crate crossterm;

use crossterm::{
    event::{self, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
    sync::mpsc,
    thread,
};
use clap::{
    crate_authors, crate_description, crate_name, crate_version,
    App, Arg, SubCommand,
};
use tui::{backend::CrosstermBackend, Terminal};
use anyhow::Result;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<()> {
    // setup app
    let app = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .get_matches();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
    Ok(())
}
