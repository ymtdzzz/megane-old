use crate::components::{
    Drawable,
    logs::Logs,
};
use crate::utils::{StatefulList, loggroup_menulist::LogGroupMenuList};
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
    style::{Style, Modifier, Color},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;
use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest, LogGroup,
};
use anyhow::Result;
use async_trait::async_trait;

use crate::components::textinput::TextInputComponent;

pub struct LogsTab
{
    log_groups: LogGroupMenuList,
    is_menu_active: bool,
    client: CloudWatchLogsClient,
    next_token: Option<String>,
    log_area: Logs,
}

impl LogsTab {
    pub async fn new(log_groups: LogGroupMenuList, region: Region) -> Result<LogsTab> {
        // TODO: remove this expression by sharing region
        let region_c = region.clone();
        let mut tab = LogsTab {
            log_groups,
            is_menu_active: true,
            client: CloudWatchLogsClient::new(region),
            next_token: None,
            log_area: Logs::new("aaaaa", CloudWatchLogsClient::new(region_c)),
        };
        tab.fetch_log_groups().await?;
        Ok(tab)
    }

    async fn fetch_log_groups(&mut self) -> Result<()> {
        let request = DescribeLogGroupsRequest {
            limit: Some(50),
            log_group_name_prefix: None,
            next_token: self.next_token.clone(),
        };
        let response = self.client.describe_log_groups(request).await?;
        self.next_token = response.next_token;
        let mut log_groups = match response.log_groups {
            Some(log_groups) => log_groups,
            None => vec![],
        };
        self.log_groups.push_items(&mut log_groups, self.next_token.as_ref());
        Ok(())
    }

    fn activate_menu_area(&mut self) {
        self.is_menu_active = true;
        self.log_area.deselect();
    }

    fn activate_log_area(&mut self) {
        self.is_menu_active = false;
        self.log_area.select();
    }
}

#[async_trait]
impl Drawable for LogsTab {
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
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        if self.is_menu_active {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    )
                    .title("Log Groups")
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        if let Some(ref mut state) = self.log_groups.get_state() {
            f.render_stateful_widget(log_list_block, chunks[0], state);
        } else {
            // TODO: raise error??
            panic!("state NONE");
        }
        self.log_area.draw(f, chunks[1]);
    }

    async fn handle_event(&mut self, event: KeyEvent) -> bool {
        let mut solved = true;
        if self.is_menu_active {
            match event.code {
                KeyCode::Down => self.log_groups.next(),
                KeyCode::Up => self.log_groups.previous(),
                KeyCode::Enter => {
                    if let Some(state) = self.log_groups.get_state() {
                        if state.selected() == Some(self.log_groups.get_labels().len() - 1) {
                            if let Some(_) = self.next_token {
                                self.fetch_log_groups().await;
                            }
                        } else {
                            if let Some(idx) = state.selected() {
                                self.log_area.set_log_group_name(self.log_groups.get_log_group_name(idx));
                                self.activate_log_area();
                            }
                        };
                    }
                },
                _ => solved = false
            }
        } else {
            if !self.log_area.handle_event(event).await {
                match event.code {
                    KeyCode::Esc => self.activate_menu_area(),
                    _ => solved = false
                }
            }
        }
        solved
    }
}
