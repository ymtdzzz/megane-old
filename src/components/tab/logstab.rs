use super::Tab;
use crate::utils::{MenuList, loggroup_menulist::LogGroupMenuList};
use tui::{
    backend::CrosstermBackend,
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
    },
    layout::{
        Layout,
        Direction,
        Constraint,
        Rect,
    },
    style::{Style, Modifier},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;
use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest, LogGroup,
};
use anyhow::Result;

pub struct LogsTab
{
    log_groups: LogGroupMenuList,
    client: CloudWatchLogsClient,
    next_token: Option<String>,
}

impl LogsTab {
    pub async fn new(log_groups: LogGroupMenuList, region: Region) -> Result<LogsTab> {
        let mut tab = LogsTab {
            log_groups,
            client: CloudWatchLogsClient::new(region),
            next_token: None,
        };
        tab.fetch_log_groups().await?;
        Ok(tab)
    }

    async fn fetch_log_groups(&mut self) -> Result<()> {
        let request = DescribeLogGroupsRequest {
            limit: Some(3),
            log_group_name_prefix: None,
            next_token: None,
        };
        let response = self.client.describe_log_groups(request).await?;
        self.next_token = response.next_token;
        let log_groups = match response.log_groups {
            Some(log_groups) => log_groups,
            None => vec![],
        };
        self.log_groups = LogGroupMenuList::new(log_groups);
        Ok(())
    }
}

impl Tab for LogsTab {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref())
            .split(area);
        let labels = self.log_groups.get_labels();
        let log_group_items: Vec<ListItem> = labels.iter()
            .map(|i| ListItem::new(i.as_ref())).collect();
        let log_list_block = List::new(log_group_items)
            .block(Block::default().borders(Borders::ALL).title("Log Groups"))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        println!("{:?}", LogGroup::default());
        if let Some(ref mut state) = self.log_groups.get_state() {
            f.render_stateful_widget(log_list_block, chunks[0], state);
        } else {
            // TODO: raise error??
            panic!("state NONE");
        }
    }

    fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.log_groups.next(),
            KeyCode::Up => self.log_groups.previous(),
            _ => {}
        }
    }
}
