use super::StatefulTable;
use tui::widgets::TableState;
use rusoto_logs::FilteredLogEvent;
use chrono::{TimeZone, Utc};

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

    pub fn get_log_event_text(&self, idx: usize) -> Option<String> {
        if let Some(item) = self.items.get(idx) {
            item.message.clone()
        } else {
            None
        }
    }

    pub fn clear_items(&mut self) {
        self.items = vec![];
        self.state = Some(TableState::default());
    }

    pub fn clone_with_state(&self, state: Option<TableState>) -> Self {
        Self {
            items: self.items.clone(),
            state,
        }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            false
        } else {
            let self_len = self.items.len().saturating_sub(2);
            let other_len = self.items.len().saturating_sub(2);
            if let Some(first) = self.items.first() {
                if let Some(other_first) = other.items.first() {
                    if first.event_id != other_first.event_id {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                if let Some(_) = other.items.first() {
                    return false;
                }
            }
            if let Some(last) = self.items.get(self_len) {
                if let Some(other_last) = other.items.get(other_len) {
                    if last.event_id != other_last.event_id {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                if let Some(_) = other.items.get(other_len) {
                    return false;
                }
            }
            true
        }
    }

    fn is_last_more_item(&self) -> bool {
        if let Some(last) = self.items.last() {
            last.event_id == Some(String::from("999"))
        } else {
            false
        }
    }

    pub fn push_items(&mut self, mut items: &mut Vec<FilteredLogEvent>, next_token: Option<&String>) {
        if self.items.len() > 0 && self.is_last_more_item() {
            self.items.remove(self.items.len() - 1);
        }
        self.items.append(&mut items);
        if let Some(_) = next_token {
            let mut more = FilteredLogEvent::default();
            more.event_id = Some(String::from("999"));
            more.message = Some(String::from(""));
            more.timestamp = None;
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
                    vec.push(String::from("More..."));
                }
                if let Some(message) = &i.message {
                    vec.push(String::from(message));
                } else {
                    vec.push(String::from(""));
                }
                vec
            })
            .collect()
    }
    fn get_state(&mut self) -> Option<TableState> {
        self.state.clone()
    }
    fn set_state(&mut self, new_state: TableState) {
        self.state = Some(new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_default_events() -> Vec<FilteredLogEvent> {
        let mut event1 = FilteredLogEvent::default();
        let mut event2 = FilteredLogEvent::default();
        let mut event3 = FilteredLogEvent::default();
        event1.event_id = Some(String::from("1"));
        event1.message = Some(String::from("message_1"));
        event2.event_id = Some(String::from("2"));
        event2.message = None;
        event3.event_id = Some(String::from("3"));
        vec![event1, event2, event3]
    }

    fn get_changed_events() -> Vec<FilteredLogEvent> {
        let mut event1 = FilteredLogEvent::default();
        let mut event2 = FilteredLogEvent::default();
        let mut event3 = FilteredLogEvent::default();
        event1.event_id = Some(String::from("a"));
        event1.message = Some(String::from("message_a"));
        event2.event_id = Some(String::from("b"));
        event3.event_id = Some(String::from("c"));
        vec![event1, event2, event3]
    }
    
    #[test]
    fn can_set_items() {
        let events = get_default_events();
        let new_events = get_changed_events();
        let mut log_event_list = LogEventList::new(events);
        let expected_log_event_list = LogEventList::new(get_changed_events());
        log_event_list.set_items(new_events);
        assert_eq!(log_event_list.items.get(0), expected_log_event_list.items.get(0));
        assert_eq!(log_event_list.items.get(1), expected_log_event_list.items.get(1));
        assert_eq!(log_event_list.items.get(2), expected_log_event_list.items.get(2));
    }

    #[test]
    fn can_return_message() {
        let events = get_default_events();
        let log_event_list = LogEventList::new(events);
        let actual_message1 = log_event_list.get_log_event_text(0);
        let actual_message2 = log_event_list.get_log_event_text(1);
        let actual_message3 = log_event_list.get_log_event_text(100);
        assert_eq!(Some(String::from("message_1")), actual_message1);
        assert_eq!(None, actual_message2);
        assert_eq!(None, actual_message3);
    }

    #[test]
    fn can_clear_items() {
        let mut log_event_list = LogEventList::new(get_default_events());
        log_event_list.clear_items();
        let expected: Vec<FilteredLogEvent> = vec![];
        assert_eq!(expected, log_event_list.items);
    }

    #[test]
    fn can_clone_with_state() {
        let mut log_event_list = LogEventList::new(get_default_events());
        let mut state = TableState::default();
        state.select(Some(999));
        let result = log_event_list.clone_with_state(Some(state));
        assert_eq!(Some(999), result.state.unwrap().selected());
    }

    #[test]
    fn can_recognize_identity() {
        let mut log_event_list = LogEventList::new(get_default_events());
        let log_event_list2 = LogEventList::new(get_default_events());
        let log_event_list3 = LogEventList::new(get_changed_events());
        assert!(log_event_list.is_same(&log_event_list2));
        assert!(!log_event_list.is_same(&log_event_list3));

        let mut log_event_list4 = LogEventList::new(get_default_events());
        log_event_list4.push_items(&mut vec![], Some(&String::from("next_token")));
        log_event_list.push_items(&mut vec![FilteredLogEvent::default()], Some(&String::from("next_token")));
        assert!(!log_event_list.is_same(&log_event_list4));
    }

    #[test]
    fn can_recognize_if_last_more_item() {
        let mut log_event_list = LogEventList::new(get_default_events());
        assert!(!log_event_list.is_last_more_item());
        let mut more = FilteredLogEvent::default();
        more.event_id = Some(String::from("999"));
        more.message = Some(String::from(""));
        more.timestamp = None;
        log_event_list.items.push(more);
        assert!(log_event_list.is_last_more_item());
    }

    #[test]
    fn can_push_items() {
        let mut log_event_list = LogEventList::new(vec![]);
        let mut event = vec![FilteredLogEvent::default(), FilteredLogEvent::default()];
        log_event_list.push_items(&mut event, None);
        assert_eq!(log_event_list.items.len(), 2);
        let mut event = vec![FilteredLogEvent::default(), FilteredLogEvent::default()];
        log_event_list.push_items(&mut event, Some(&String::from("token")));
        assert_eq!(log_event_list.items.len(), 5);
        assert_eq!(
            log_event_list.items.last().unwrap().event_id,
            Some(String::from("999")),
        );
    }
}
