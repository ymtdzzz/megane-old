use super::StatefulTable;
use tui::widgets::TableState;
use rusoto_logs::FilteredLogEvent;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
pub struct LogEventList {
    items: Vec<FilteredLogEvent>,
    state: Option<TableState>,
}

impl LogEventList {
    pub fn new(items: Vec<FilteredLogEvent>) -> Self {
        Self {
            items,
            state: Some(TableState::default()),
        }
    }

    pub fn set_items(&mut self, items: Vec<FilteredLogEvent>) {
        self.items = items;
    }

    pub fn clear_items(&mut self) {
        self.items = vec![];
        self.state = Some(TableState::default());
    }

    pub fn push_items(&mut self, mut items: &mut Vec<FilteredLogEvent>, next_token: Option<&String>) {
        if self.items.len() > 0 {
            self.items.remove(self.items.len() - 1);
        }
        self.items.append(&mut items);
        if let Some(_) = next_token {
            let mut more = FilteredLogEvent::default();
            more.event_id = Some(String::from("999"));
            more.message = Some(String::from("More..."));
            self.items.push(more);
        }
    }
}

impl StatefulTable for LogEventList {
    fn get_labels(&self) -> Vec<Vec<String>> {
        self.items
            .iter()
            .map(|i| {
                let mut vec = Vec::with_capacity(2);
                if let Some(timestamp) = &i.timestamp {
                    let dt = Utc.timestamp(*timestamp / 1000, 0);
                    vec.push(dt.format("%Y-%m-%d %H:%M:%S %Z").to_string());
                } else {
                    vec.push(String::from(""));
                }
                if let Some(message) = &i.message {
                    vec.push(String::from(message));
                } else {
                    vec.push(String::from(""));
                }
                vec
            })
            .filter(|item| item[0] != String::from("") && item[1] != String::from(""))
            .collect()
    }
    fn get_state(&mut self) -> Option<TableState> {
        // TODO: more smart implementation not to use clone()
        self.state.clone()
    }
    fn set_state(&mut self, new_state: TableState) {
        self.state = Some(new_state);
    }
}
