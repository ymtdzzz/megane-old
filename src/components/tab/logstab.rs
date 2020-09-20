use super::Tab;
use crate::utils::StatefulList;
use tui::{
    backend::CrosstermBackend,
    widgets::{
        Block,
        Borders,
        Tabs,
        List,
        ListItem,
        ListState,
    },
    layout::{
        Layout,
        Direction,
        Constraint,
        Rect,
    },
    text::Spans,
    style::{Style, Color, Modifier},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;
use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest, LogGroup,
};
use anyhow::Result;

pub struct LogsTab {
    log_groups: StatefulList,
    client: CloudWatchLogsClient,
    next_token: Option<String>,
}

impl LogsTab {
    pub fn new(log_groups: StatefulList, region: Region) -> LogsTab {
        let mut tab = LogsTab {
            log_groups,
            client: CloudWatchLogsClient::new(region),
            next_token: None,
        };
        tab.fetch_log_groups();
        tab
    }

    async fn fetch_log_groups(&mut self) -> Result<Option<Vec<LogGroup>>> {
        let request = DescribeLogGroupsRequest {
            limit: Some(10),
            log_group_name_prefix: None,
            next_token: None,
        };
        let response = self.client.describe_log_groups(request).await?;
        self.next_token = response.next_token;
        println!("{:?}", response.log_groups);
        Ok(response.log_groups)
    }
}

impl Tab for LogsTab {
    fn draw(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref())
            .split(area);
        let log_group_items: Vec<ListItem> = self.log_groups.items.iter()
            .map(|i| ListItem::new(i.as_ref())).collect();
        let log_list_block = List::new(log_group_items)
            .block(Block::default().borders(Borders::ALL).title("Log Groups"))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(log_list_block, chunks[0], &mut self.log_groups.state.clone());
    }

    fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.log_groups.next(),
            KeyCode::Up => self.log_groups.previous(),
            _ => {}
        }
    }
}
