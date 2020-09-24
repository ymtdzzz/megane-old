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
use rusoto_logs::{
    CloudWatchLogsClient
};
use anyhow::Result;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::instruction::Instruction;
use crate::globalstate::GlobalState;

pub struct LogsTab
{
    log_groups: LogGroupMenuList,
    is_menu_active: bool,
    log_area: Logs,
    tx: Sender<Instruction>,
    state: Arc<Mutex<GlobalState>>,
}

impl LogsTab {
    pub async fn new(log_groups: LogGroupMenuList, tx: Sender<Instruction>, state: Arc<Mutex<GlobalState>>) -> Result<LogsTab> {
        // TODO: remove this expression by sharing region
        let region_c = rusoto_core::Region::ApNortheast1;
        let child_tx = Sender::clone(&tx);
        let child_state = Arc::clone(&state);
        let tab = LogsTab {
            log_groups,
            is_menu_active: true,
            log_area: Logs::new("Logs", CloudWatchLogsClient::new(region_c), child_state),
            tx,
            state,
        };
        // tab.fetch_log_groups().await?;
        child_tx.send(Instruction::FetchLogGroups)?;
        Ok(tab)
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
        let labels = if let Ok(m_guard) = self.state.try_lock() {
            self.log_groups = m_guard.log_groups.clone_with_state(self.log_groups.get_state());
            self.log_groups.get_labels()
        } else {
            vec![]
        };
        let log_group_items: Vec<ListItem> = labels.iter()
            .map(|i| ListItem::new(i.as_ref())).collect();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(
                if self.is_menu_active {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                }
            );
        let block = if let Ok(s) = self.state.try_lock() {
            if !s.log_groups_fething {
                block.title("Log Groups")
            } else {
                block.title("Log Groups [Fetching ...]")
            }
        } else {
            block.title("Log Groups [Fetching ...]")
        };
        let log_list_block = List::new(log_group_items)
            .block(block)
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
                KeyCode::Down => {
                    self.log_groups.next();
                    if let Some(s) = self.log_groups.get_state() {
                        if let Some(i) = s.selected() {
                            if let Some(item) = self.log_groups.get_item(i) {
                                let log_group_name = if let Some(name) = item.log_group_name.as_ref() {
                                    name.clone()
                                } else {
                                    String::from("")
                                };
                                self.tx.send(Instruction::FetchLogEvents(log_group_name, String::from(""))).unwrap();
                            }
                        }
                        // self.tx.send(Instruction::FetchLogEvents(self.))
                    }
                },
                KeyCode::Up => self.log_groups.previous(),
                KeyCode::Enter => {
                    if let Some(state) = self.log_groups.get_state() {
                        if state.selected() == Some(self.log_groups.get_labels().len() - 1) {
                            if self.log_groups.has_more_items() {
                                // self.fetch_log_groups().await;
                                self.tx.send(Instruction::FetchLogGroups).unwrap();
                            } else {
                                if let Some(idx) = state.selected() {
                                    self.log_area.set_log_group_name(self.log_groups.get_log_group_name(idx));
                                    self.activate_log_area();
                                }
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
