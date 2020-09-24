extern crate clap;
extern crate tui;
extern crate crossterm;
extern crate megane;
extern crate rusoto_core;
extern crate rusoto_logs;

use crossterm::{
    event::{self, EnableMouseCapture, DisableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
    sync::{mpsc, Arc, Mutex},
    thread,
};
use clap::{
    crate_authors, crate_description, crate_name, crate_version,
    App as ClapApp,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use anyhow::Result;
use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs,
    CloudWatchLogsClient,
    DescribeLogGroupsRequest,
    FilterLogEventsRequest,
};

use megane::{ui, app::App, instruction::Instruction, globalstate::GlobalState};

enum Event<I> {
    Input(I),
    Tick,
}

#[tokio::main]
async fn main() -> Result<()> {
    // setup app
    let _clap = ClapApp::new(crate_name!())
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

    // AWS SDK networking
    let (aws_tx, aws_rx) = mpsc::channel::<Instruction>();
    let state = Arc::new(Mutex::new(GlobalState::new()));
    let state0 = Arc::clone(&state);
    tokio::spawn(async move {
        let client = CloudWatchLogsClient::new(Region::ApNortheast1);
        loop {
            let instruction = aws_rx.recv().unwrap();
            match instruction {
                Instruction::FetchLogEvents(log_group_name, filter_pattern) => {
                    state0.lock().unwrap().log_events_fetching = false;
                    if log_group_name != state0.lock().unwrap().log_events_selected_log_group_name {
                        state0.lock().unwrap().log_events.clear_items();
                    }
                    let mut request = FilterLogEventsRequest::default();
                    request.log_group_name = log_group_name;
                    request.filter_pattern = Some(filter_pattern);
                    request.limit = Some(100);
                    let response = client.filter_log_events(request).await;
                    if let Ok(res) = response {
                        let mut state = state0.lock().unwrap();
                        state.log_events_next_token = res.next_token;
                        let mut events = match res.events {
                            Some(events) => events,
                            None => vec![],
                        };
                        let token = state.log_events_next_token.clone();
                        state.log_events.push_items(&mut events, token.as_ref());
                    }
                    state0.lock().unwrap().log_events_fetching = false;
                },
                Instruction::FetchLogGroups => {
                    state0.lock().unwrap().log_groups_fething = true;
                    let request = DescribeLogGroupsRequest {
                        limit: Some(3),
                        log_group_name_prefix: None,
                        next_token: state0.lock().unwrap().log_groups_next_token.clone(),
                    };
                    let response = client.describe_log_groups(request).await;
                    if let Ok(res) = response {
                        let mut state = state0.lock().unwrap();
                        state.log_groups_next_token = res.next_token;
                        let mut log_groups = match res.log_groups {
                            Some(log_groups) => log_groups,
                            None => vec![],
                        };
                        let token = state.log_groups_next_token.clone();
                        state.log_groups.push_items(&mut log_groups, token.as_ref());
                    }
                    state0.lock().unwrap().log_groups_fething = false;
                }
            }
        }
    });

    let mut app = App::new(aws_tx, state).await?; 

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        // event handling
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    // quit
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => app.handle_event(event).await
            },
            Event::Tick => {
                // TODO: fetch logs and metrics
            }
        }
    }
    Ok(())
}
