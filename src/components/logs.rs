use crate::components::{
    Drawable,
    textinput::TextInputComponent,
};
use crate::utils::logevent_list::LogEventList;
use crate::utils::StatefulTable;
use crate::utils;
use crate::globalstate::GlobalState;
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
    },
    style::{Style, Color},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::io::Stdout;
use async_trait::async_trait;
use std::sync::{Arc, Mutex, mpsc::Sender};

pub struct Logs {
    search_area: TextInputComponent,
    title: String,
    event_list: LogEventList,
    is_active: bool,
    is_search_active: bool,
    log_group_name: Option<String>,
    state: Arc<Mutex<GlobalState>>,
    cached_labels: Vec<Vec<String>>,
    tx: Sender<Instruction>,
}

impl Logs {
    pub fn new(title: &str, tx: Sender<Instruction>, state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            search_area: TextInputComponent::new("Filter[f]", ""),
            title: title.to_string(),
            event_list: LogEventList::new(vec![]),
            is_active: false,
            is_search_active: false,
            log_group_name: None,
            state,
            cached_labels: vec![vec![]],
            tx,
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
    }

    fn fetch_log_events(&self) {
        if let Some(log_group_name) = &self.log_group_name {
            self.tx.send(Instruction::FetchLogEvents(
                log_group_name.clone(),
                self.search_area.get_text().to_string(),
            )).unwrap();
        }
    }
}

#[async_trait]
impl Drawable for Logs {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Max(100),
                Constraint::Percentage(20),
            ].as_ref())
            .split(area);
        let log_text = match &mut self.state.try_lock() {
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
        let rows = self.cached_labels.iter().map(|i| Row::Data(i.iter()));
        let event_table_block = Table::new(
            ["Timestamp", "Message"].iter(),
            rows,
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
        let text_area = Paragraph::new(
            Text::from(log_text.as_str())
        )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("full text")
            );
        self.search_area.draw(f, chunks[0]);
        if let Some(ref mut state) = self.event_list.get_state() {
            f.render_stateful_widget(event_table_block, chunks[1], state);
            f.render_widget(text_area, chunks[2]);
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
                        self.clear_cache();
                        self.state.lock().unwrap().reset_log_event_results();
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
            let is_shift = if event.modifiers == KeyModifiers::SHIFT {
                true
            } else {
                false
            };
            match event.code {
                KeyCode::Down => {
                    if is_shift {
                        if self.event_list.next_by(10) {
                            self.fetch_log_events();
                        }
                    } else {
                        if self.event_list.next() {
                            self.fetch_log_events();
                        }
                    }
                },
                KeyCode::Up => {
                    if is_shift {
                        self.event_list.previous_by(10);
                    } else {
                        self.event_list.previous();
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
