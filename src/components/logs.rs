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
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;
use anyhow::Result;
use async_trait::async_trait;

pub struct Logs {
    search_area: TextInputComponent,
    client: CloudWatchLogsClient,
    title: String,
    event_list: LogEventList,
    next_token: Option<String>,
    is_active: bool,
    log_group_name: Option<String>,
}

impl Logs {
    pub fn new(title: &str, client: CloudWatchLogsClient) -> Self {
        Self {
            search_area: TextInputComponent::new("Filter", ""),
            client,
            title: title.to_string(),
            event_list: LogEventList::new(vec![]),
            next_token: None,
            is_active: false,
            log_group_name: None,
        }
    }

    pub fn toggle_active(&mut self) {
        self.is_active = !self.is_active;
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
                        if self.is_active {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    )
                    .title(self.title.as_str())
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
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

    async fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => {
                if let Some(log_group_name) = self.log_group_name.clone() {
                    self.fetch_log_events(None, log_group_name).await;
                }
            },
            _ => {}
        }
    }
}
