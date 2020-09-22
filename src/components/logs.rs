use crate::components::{
    Drawable,
    textinput::TextInputComponent,
};
use crate::utils::logevent_list::LogEventList;
use crate::utils::StatefulTable;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, FilteredLogEvent, FilterLogEventsRequest
};
use tui::{
    backend::CrosstermBackend,
    layout::{
        Layout,
        Constraint,
        Direction,
        Rect,
    },
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
        Table,
        Row,
        TableState,
    },
    style::{Style, Color, Modifier},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::io::Stdout;
use anyhow::Result;
use async_trait::async_trait;
use core::slice::Iter;
use std::iter::Map;
use core::fmt::Display;

pub struct Logs {
    search_area: TextInputComponent,
    client: CloudWatchLogsClient,
    title: String,
    event_list: LogEventList,
    next_token: Option<String>,
    is_active: bool,
    is_search_active: bool,
    log_group_name: Option<String>,
    // cached_rows: Iterator<Item = Row<Iterator<Item = String>>>,
}

impl Logs {
    pub fn new(title: &str, client: CloudWatchLogsClient) -> Self {
        let labels: Vec<Vec<String>> = vec![vec![]];
        let rows = labels.iter().map(|i| Row::Data(i.iter()));
        Self {
            search_area: TextInputComponent::new("Filter", ""),
            client,
            title: title.to_string(),
            event_list: LogEventList::new(vec![]),
            next_token: None,
            is_active: false,
            is_search_active: false,
            log_group_name: None,
            // cached_rows: rows,
        }
    }

    pub fn select(&mut self) {
        self.is_active = true;
        self.activate_search_area();
    }

    pub fn deselect(&mut self) {
        self.is_active = false;
        self.activate_logs_area();
    }

    pub fn activate_search_area(&mut self) {
        self.is_search_active = true;
        self.search_area.select();
    }

    pub fn activate_logs_area(&mut self) {
        self.is_search_active = false;
        self.search_area.set_input_mode(crate::components::textinput::InputMode::NormalMode);
        self.search_area.deselect();
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_log_group_name(&mut self, log_group_name: Option<String>) {
        self.log_group_name = log_group_name;
    }

    async fn fetch_log_events(&mut self, filter_pattern: Option<String>, log_group_name: String) -> Result<()> {
        let mut request = FilterLogEventsRequest::default();
        request.filter_pattern = filter_pattern;
        request.log_group_name = log_group_name;
        request.limit = Some(100);
        let response = self.client.filter_log_events(request).await?;
        self.next_token = response.next_token;
        let mut events = match response.events {
            Some(events) => events,
            None => vec![],
        };
        self.event_list.push_items(&mut events, self.next_token.as_ref());
        Ok(())
    }


}

#[async_trait]
impl Drawable for Logs {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Percentage(100),
            ].as_ref())
            .split(area);
        let labels: Vec<Vec<String>> = self.event_list.get_labels();
        let rows = labels.iter().map(|i| Row::Data(i.iter()));
        let event_table_block = Table::new(
            ["Timestamp", "Message"].iter(),
            rows
        )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        if !self.is_search_active && self.is_active {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    )
                    .title(self.title.as_str())
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
            )
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(100),
            ]);
        self.search_area.draw(f, chunks[0]);
        if let Some(ref mut state) = self.event_list.get_state() {
            f.render_stateful_widget(event_table_block, chunks[1], state);
        } else {
            panic!("state NONE");
        }
    }

    async fn handle_event(&mut self, event: KeyEvent) -> bool {
        let mut solved = true;
        if self.is_search_active {
            // search area event handling
            if !self.search_area.handle_event(event).await {
                match event.code {
                    KeyCode::Enter => {
                        self.event_list.clear_items();
                        let filter_pattern = Some(self.search_area.get_text().clone());
                        let log_group_name = self.log_group_name.clone().unwrap_or(String::from(""));
                        self.fetch_log_events(filter_pattern, log_group_name).await;
                        self.activate_logs_area();
                    },
                    _ => solved = false
                }
            } else {
                solved = false
            }
        } else {
            // logs area event handling
            let is_shift = if event.modifiers == KeyModifiers::SHIFT {
                true
            } else {
                false
            };
            match event.code {
                KeyCode::Down => {
                    if is_shift {
                        self.event_list.next_by(10)
                    } else {
                        self.event_list.next()
                    }
                },
                KeyCode::Up => {
                    if is_shift {
                        self.event_list.previous_by(10)
                    } else {
                        self.event_list.previous()
                    }
                },
                _ => solved = false
            }
        };
        solved
    }
}
