use crate::components::{
    Drawable,
    textinput::TextInputComponent,
};
use crate::utils::logevent_list::LogEventList;
use crate::utils::StatefulTable;
use crate::utils;
use crate::globalstate::{GlobalState, GlobalStateTail};
use crate::instruction::Instruction;
use tui::{
    backend::CrosstermBackend,
    layout::{
        Layout,
        Constraint,
        Direction,
        Rect,
    },
    text::Text,
    widgets::{
        Block,
        Borders,
        Table,
        Row,
        Paragraph,
        TableState,
    },
    style::{Style, Color},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::io::Stdout;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use async_trait::async_trait;
use std::sync::{Arc, Mutex, mpsc::Sender};

#[derive(Debug)]
enum SearchMode {
    Tail,
    All,
    OneM,
    ThirtyM,
    OneH,
    TwelveH,
    Range(i64, i64),
}

pub struct Logs {
    search_area: TextInputComponent,
    title: String,
    event_list: LogEventList,
    tailed_event_list: LogEventList,
    is_active: bool,
    is_search_active: bool,
    log_group_name: Option<String>,
    state: Arc<Mutex<GlobalState>>,
    tail_state: Arc<Mutex<GlobalStateTail>>,
    cached_labels: Vec<Vec<String>>,
    cached_tailed_labels: Vec<Vec<String>>,
    tx: Sender<Instruction>,
    search_mode: SearchMode,
}

impl Logs {
    pub fn new(title: &str, tx: Sender<Instruction>, state: Arc<Mutex<GlobalState>>, tail_state: Arc<Mutex<GlobalStateTail>>) -> Self {
        Self {
            search_area: TextInputComponent::new("Filter(f)", ""),
            title: title.to_string(),
            event_list: LogEventList::new(vec![]),
            tailed_event_list: LogEventList::new(vec![]),
            is_active: false,
            is_search_active: false,
            log_group_name: None,
            state,
            tail_state,
            cached_labels: vec![vec![]],
            cached_tailed_labels: vec![vec![]],
            tx,
            search_mode: SearchMode::All,
        }
    }

    pub fn select(&mut self) {
        self.is_active = true;
        self.activate_logs_area();
    }

    pub fn deselect(&mut self) {
        self.is_active = false;
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

    pub fn get_log_group_name(&self) -> Option<String> {
        self.log_group_name.clone()
    }

    fn clear_cache(&mut self) {
        self.cached_labels = vec![];
        self.cached_tailed_labels = vec![];
    }

    fn get_search_range(&self) -> (i64, i64) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        match self.search_mode {
            SearchMode::Tail => {
                let start = now
                    .checked_sub(Duration::from_secs(60))
                    .unwrap_or(Duration::from_secs(0))
                    .as_millis();
                (start as i64, now.as_millis() as i64)
            },
            SearchMode::OneM => {
                let start = now
                    .checked_sub(Duration::from_secs(60))
                    .unwrap_or(Duration::from_secs(0))
                    .as_millis();
                (start as i64, now.as_millis() as i64)
            },
            SearchMode::ThirtyM => {
                let start = now
                    .checked_sub(Duration::from_secs(900))
                    .unwrap_or(Duration::from_secs(0))
                    .as_millis();
                (start as i64, now.as_millis() as i64)
            },
            SearchMode::OneH => {
                let start = now
                    .checked_sub(Duration::from_secs(3600))
                    .unwrap_or(Duration::from_secs(0))
                    .as_millis();
                (start as i64, now.as_millis() as i64)
            },
            SearchMode::TwelveH => {
                let start = now
                    .checked_sub(Duration::from_secs(43200))
                    .unwrap_or(Duration::from_secs(0))
                    .as_millis();
                (start as i64, now.as_millis() as i64)
            },
            SearchMode::All => {
                (0, 0)
            },
            SearchMode::Range(start, end) => {
                (start, end)
            },
        }
    }

    pub fn clear_results(&mut self) {
        self.event_list.clear_items();
        self.tailed_event_list.clear_items();
        self.clear_cache();
        self.state.lock().unwrap().reset_log_event_results();
    }

    pub fn fetch_log_events(&self) {
        if let Some(log_group_name) = &self.log_group_name {
            let (start, end): (i64, i64) = self.get_search_range();
            self.tx.send(Instruction::FetchLogEvents(
                log_group_name.clone(),
                self.search_area.get_text().to_string(),
                start.clone(),
                end.clone(),
            )).unwrap();
        }
    }

    fn get_search_area_title(&self) -> String {
        let base = "Filter(f) - Mode: ";
        let mut tail = "[ ]tail";
        let mut onem = "[ ]1m";
        let mut thirtym = "[ ]15m";
        let mut oneh = "[ ]1h";
        let mut twelveh = "[ ]12h";
        let mut range = "[ ]range(2020-12-12-01:01:00~2020-12-15-01:00:30)";
        match self.search_mode {
            SearchMode::OneM => {
                onem = "[x]1m";
            },
            SearchMode::ThirtyM => {
                thirtym = "[x]15m";
            },
            SearchMode::OneH => {
                oneh = "[x]1h";
            },
            SearchMode::TwelveH => {
                twelveh = "[x]12h";
            },
            SearchMode::Range(start, end) => {
                range = "[x]range(2020-12-12-01:01:00~2020-12-15-01:00:30)";
            },
            SearchMode::Tail => {
                tail = "[x]tail";
            },
            SearchMode::All => {},
        }
        format!("{}{}{}{}{}{}{}", base, tail, onem, thirtym, oneh, twelveh, range)
    }

    fn clear_search_mode(&mut self) {
        self.search_mode = SearchMode::All;
    }

    fn sync_global_state_tail(&self) {
        if let Ok(mut state) = self.tail_state.try_lock() {
            if let Some(log_group_name) = &self.log_group_name {
                state.log_events_selected_log_group_name = log_group_name.clone();
                state.log_events_filter_pattern = Some(self.search_area.get_text().to_string());
            } 
        }
    }

    fn is_tail_mode(&self) -> bool {
        if let SearchMode::Tail = self.search_mode {
            true
        } else {
            false
        }
    }
}

#[async_trait]
impl Drawable for Logs {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        self.sync_global_state_tail();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Max(100),
                Constraint::Percentage(20),
            ].as_ref())
            .split(area);
        let mut log_text = String::from("");
        match self.search_mode {
            SearchMode::Tail => {
                log_text = String::from("tail mode");
                match &mut self.tail_state.try_lock() {
                    Ok(m_guard) => {
                        if !self.tailed_event_list.is_same(&m_guard.log_events) {
                            self.tailed_event_list = m_guard.log_events.clone_with_state(self.tailed_event_list.get_state());
                            self.cached_tailed_labels = self.tailed_event_list.get_labels();
                            let mut new_state = TableState::default();
                            new_state.select(Some(self.cached_tailed_labels.len().saturating_sub(1)));
                            self.tailed_event_list.set_state(new_state);
                        }
                    },
                    Err(_) => {}
                }
            },
            _ => {
                log_text = match &mut self.state.try_lock() {
                    Ok(m_guard) => {
                        if !self.event_list.is_same(&m_guard.log_events) {
                            self.event_list = m_guard.log_events.clone_with_state(self.event_list.get_state());
                            self.cached_labels = self.event_list.get_labels();
                        }
                        let mut result = String::from("");
                        if let Some(s) = self.event_list.get_state() {
                            if let Some(idx) = s.selected() {
                                if let Some(msg) = m_guard.log_events.get_log_event_text(idx) {
                                    result = utils::insert_new_line_at(
                                        chunks[2].width as usize - 2,
                                        msg.as_str(),
                                    );
                                }
                            }
                        };
                        result
                    },
                    Err(_) => String::from("")
                };
            }
        }
        let rows = self.cached_labels.iter().map(|i| Row::Data(i.iter()));
        let tailed_rows = self.cached_tailed_labels.iter().map(|i| Row::Data(i.iter()));
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
                Constraint::Percentage(15),
                Constraint::Percentage(100),
            ]);
        let tail_event_table_block = Table::new(
            ["Timestamp", "Message"].iter(),
            tailed_rows
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
                Constraint::Percentage(15),
                Constraint::Percentage(100),
            ]);
        self.search_area.set_title(self.get_search_area_title());
        let text_area = Paragraph::new(
            Text::from(log_text.as_str())
        )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("full text")
            );
        self.search_area.draw(f, chunks[0]);
        if let SearchMode::Tail = self.search_mode {
            if let Some(ref mut state) = self.tailed_event_list.get_state() {
                f.render_stateful_widget(tail_event_table_block, chunks[1], state);
            } else {
                panic!("state NONE");
            }
        } else {
            if let Some(ref mut state) = self.event_list.get_state() {
                f.render_stateful_widget(event_table_block, chunks[1], state);
            } else {
                panic!("state NONE");
            }
        }
        f.render_widget(text_area, chunks[2]);
    }

    async fn handle_event(&mut self, event: KeyEvent) -> bool {
        let mut solved = true;
        let is_shift = if event.modifiers == KeyModifiers::SHIFT {
            true
        } else {
            false
        };
        let is_ctrl = if event.modifiers == KeyModifiers::CONTROL {
            true
        } else {
            false
        };
        if !self.search_area.is_normal_mode() {} else {
            if is_ctrl {
                match event.code {
                    KeyCode::Char('z') => {
                        if let SearchMode::Tail = self.search_mode {
                            self.clear_search_mode();
                        } else {
                            self.search_mode = SearchMode::Tail;
                        }
                        self.clear_results();
                        self.fetch_log_events();
                    },
                    KeyCode::Char('x') => {
                        if let SearchMode::OneM = self.search_mode {
                            self.clear_search_mode();
                        } else {
                            self.search_mode = SearchMode::OneM;
                        }
                        self.clear_results();
                        self.fetch_log_events();
                    },
                    KeyCode::Char('c') => {
                        if let SearchMode::ThirtyM = self.search_mode {
                            self.clear_search_mode();
                        } else {
                            self.search_mode = SearchMode::ThirtyM;
                        }
                        self.clear_results();
                        self.fetch_log_events();
                    },
                    KeyCode::Char('v') => {
                        if let SearchMode::OneH = self.search_mode {
                            self.clear_search_mode();
                        } else {
                            self.search_mode = SearchMode::OneH;
                        }
                        self.clear_results();
                        self.fetch_log_events();
                    },
                    KeyCode::Char('b') => {
                        if let SearchMode::TwelveH = self.search_mode {
                            self.clear_search_mode();
                        } else {
                            self.search_mode = SearchMode::TwelveH;
                        }
                        self.clear_results();
                        self.fetch_log_events();
                    },
                    KeyCode::Char('n') => {
                        if let SearchMode::Range(_, _) = self.search_mode {
                            self.clear_search_mode();
                        } else {
                            self.search_mode = SearchMode::Range(0, 0);
                        }
                        self.clear_results();
                        self.fetch_log_events();
                    },
                    _ => {},
                }
            }
        }
        if self.is_search_active {
            // search area event handling
            if !self.search_area.handle_event(event).await {
                match event.code {
                    KeyCode::Enter => {
                        self.clear_results();
                        self.fetch_log_events();
                        self.activate_logs_area();
                    },
                    _ => solved = false
                }
            } else {
                solved = true
            }
        } else {
            // logs area event handling
            match event.code {
                KeyCode::Down => {
                    if !self.is_tail_mode() {
                        if is_shift {
                            if self.event_list.next_by(10) {
                                self.fetch_log_events();
                            }
                        } else {
                            if self.event_list.next() {
                                self.fetch_log_events();
                            }
                        }
                    }
                },
                KeyCode::Up => {
                    if !self.is_tail_mode() {
                        if is_shift {
                            self.event_list.previous_by(10);
                        } else {
                            self.event_list.previous();
                        }
                    }
                },
                KeyCode::Char('f') => {
                    self.activate_search_area();
                },
                _ => solved = false
            }
        };
        solved
    }
}
