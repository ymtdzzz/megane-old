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
                }
            }
            if let Some(last) = self.items.get(self_len) {
                if let Some(other_last) = other.items.get(other_len) {
                    if last.event_id != other_last.event_id {
                        return false;
                    }
                }
            }
            true
        }
    }

    pub fn is_last_more_item(&self) -> bool {
        if let Some(last) = self.items.last() {
            last.event_id == Some(String::from("999"))
        } else {
            false
        }
    }

    pub fn push_items(&mut self, items: &mut Vec<FilteredLogEvent>, next_token: Option<&String>) {
        if self.items.len() > 0 && self.is_last_more_item() {
            self.items.remove(self.items.len() - 1);
        }
        let mut idx: Option<usize> = None;
        for (i, val) in items.iter().enumerate() {
            let mut found = false;
            for v in self.items.iter() {
                if val.event_id == v.event_id {
                    found = true;
                    break;
                }
            }
            if !found {
                idx = Some(i);
                break;
            }
        }
        if self.items.len() == 0 {
            idx = Some(0);
        }
        if let Some(idx) = idx {
            let idx = idx;
            let mut items_to_push = items.split_off(idx);
            self.items.append(&mut items_to_push);
        }
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
    fn next(&mut self) -> bool {
        let mut fetch_flg = false;
        let max = self.items.len().saturating_sub(1);
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i >= max {
                        if self.is_last_more_item() {
                            fetch_flg = true;
                        }
                        max
                    } else {
                        i + 1
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
        fetch_flg
    }
    fn next_by(&mut self, size: usize) -> bool {
        let mut fetch_flag = false;
        if let Some(mut state) = self.get_state() {
            let max = self.get_labels().len().saturating_sub(1);
            let i = match state.selected() {
                Some(i) => {
                    if i >= max {
                        if self.is_last_more_item() {
                            fetch_flag = true;
                        }
                        max
                    } else {
                        i + size
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
        fetch_flag
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

    fn get_random_events() -> Vec<FilteredLogEvent> {
        let mut event1 = FilteredLogEvent::default();
        let mut event2 = FilteredLogEvent::default();
        event1.event_id = Some(String::from("1"));
        event1.message = Some(String::from("message_1"));
        // 2020-01-01 00:00:00 UTC
        event1.timestamp = Some(1577836800000);
        event2.event_id = Some(String::from("2"));
        event2.message = None;
        event2.timestamp = None;
        vec![event1, event2]
    }

    fn get_some_events() -> Vec<FilteredLogEvent> {
        let mut event1 = FilteredLogEvent::default();
        let mut event2 = FilteredLogEvent::default();
        let mut event3 = FilteredLogEvent::default();
        event1.event_id = Some(String::from("1"));
        event1.message = Some(String::from("message_1"));
        event2.event_id = Some(String::from("2"));
        event2.message = None;
        event3.event_id = Some(String::from("999"));
        vec![event1, event2, event3]
    }

    fn make_event(event_id: &str) -> FilteredLogEvent {
        let mut event = FilteredLogEvent::default();
        event.event_id = Some(event_id.to_string());
        event
    }
    
    #[test]
    fn it_can_set_items() {
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
    fn it_can_return_message() {
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
    fn it_can_clear_items() {
        let mut log_event_list = LogEventList::new(get_default_events());
        log_event_list.clear_items();
        let expected: Vec<FilteredLogEvent> = vec![];
        assert_eq!(expected, log_event_list.items);
    }

    #[test]
    fn it_can_clone_with_state() {
        let log_event_list = LogEventList::new(get_default_events());
        let mut state = TableState::default();
        state.select(Some(999));
        let result = log_event_list.clone_with_state(Some(state));
        assert_eq!(Some(999), result.state.unwrap().selected());
    }

    #[test]
    fn it_can_recognize_identity() {
        let mut log_event_list = LogEventList::new(get_default_events());
        let log_event_list2 = LogEventList::new(get_default_events());
        let log_event_list3 = LogEventList::new(get_changed_events());
        assert!(log_event_list.is_same(&log_event_list2));
        assert!(!log_event_list.is_same(&log_event_list3));

        let mut log_event_list4 = LogEventList::new(get_default_events());
        log_event_list4.push_items(&mut vec![], Some(&String::from("next_token")));
        log_event_list.push_items(&mut vec![FilteredLogEvent::default()], Some(&String::from("next_token")));
        assert!(!log_event_list.is_same(&log_event_list4));

        let mut log_event_list5 = LogEventList::new(get_default_events());
        let mut log_event_list6 = LogEventList::new(get_default_events());
        let mut log_event_1 = FilteredLogEvent::default();
        log_event_1.event_id = Some(String::from("123"));
        let mut log_event_2 = FilteredLogEvent::default();
        log_event_2.event_id = Some(String::from("124"));
        log_event_list5.push_items(&mut vec![log_event_1], Some(&String::from("next_token")));
        log_event_list6.push_items(&mut vec![log_event_2], Some(&String::from("next_token")));
        assert!(!log_event_list5.is_same(&log_event_list6));
    }

    #[test]
    fn it_can_recognize_if_last_more_item() {
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
    fn it_can_recognize_if_last_more_item_from_empty() {
        let log_event_list = LogEventList::new(vec![]);
        assert!(!log_event_list.is_last_more_item());
    }

    #[test]
    fn it_can_push_items() {
        let mut log_event_list = LogEventList::new(vec![]);
        let mut event = vec![make_event("1"), make_event("2")];
        log_event_list.push_items(&mut event, None);
        assert_eq!(log_event_list.items.len(), 2);
        let mut event = vec![make_event("3"), make_event("4")];
        log_event_list.push_items(&mut event, Some(&String::from("token")));
        assert_eq!(log_event_list.items.len(), 5);
        assert_eq!(
            log_event_list.items.last().unwrap().event_id,
            Some(String::from("999")),
        );
        let mut event = vec![make_event("5")];
        log_event_list.push_items(&mut event, Some(&String::from("token")));
        assert_eq!(log_event_list.items.len(), 6);
        assert_eq!(
            log_event_list.items.last().unwrap().event_id,
            Some(String::from("999")),
        );
        assert_ne!(
            log_event_list.items.get(4).unwrap().event_id,
            Some(String::from("999")),
        );
    }

    #[test]
    fn it_can_push_only_not_duplicated_items() {
        let mut log_event_list = LogEventList::new(get_default_events());
        let mut event_list = vec![
            make_event("1"),
            make_event("4"),
            make_event("5"),
        ];
        log_event_list.push_items(&mut event_list, None);
        assert_eq!(5, log_event_list.items.len());
    }

    #[test]
    fn it_can_get_labels() {
        let log_event_list = LogEventList::new(get_random_events());
        let expected_labels = vec![vec![String::from("2020-01-01 00:00:00 UTC"), String::from("message_1")], vec![String::from("More..."), String::from("")]];
        let actual_labels = log_event_list.get_labels();
        assert_eq!(actual_labels, expected_labels);
    }

    #[test]
    fn it_can_get_state() {
        let mut log_event_list = LogEventList::new(get_default_events());
        assert_eq!(log_event_list.get_state().is_some(), true);
        log_event_list.state = None;
        assert_eq!(log_event_list.get_state().is_none(), true);
    }

    #[test]
    fn it_can_set_state() {
        let mut log_event_list = LogEventList::new(get_default_events());
        let mut state = TableState::default();
        state.select(Some(999));
        log_event_list.set_state(state);
        assert_eq!(
            log_event_list.get_state().unwrap().selected(),
            Some(999)
        );
    }

    #[test]
    fn it_can_next() {
        let mut log_event_list = LogEventList::new(get_some_events());
        
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected().is_none(), true);
        let fetch_flag = log_event_list.next();
        assert_eq!(fetch_flag, false);

        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(0));
        let fetch_flag = log_event_list.next();
        assert_eq!(fetch_flag, false);
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(1));

        let fetch_flag = log_event_list.next();
        assert_eq!(fetch_flag, false);
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(2));

        let fetch_flag = log_event_list.next();
        assert_eq!(fetch_flag, true);
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(2));
    }

    #[test]
    fn it_can_next_by() {
        let mut log_event_list = LogEventList::new(get_some_events());
        
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected().is_none(), true);
        let fetch_flag = log_event_list.next_by(2);
        assert_eq!(fetch_flag, false);

        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(0));
        let fetch_flag = log_event_list.next_by(2);
        assert_eq!(fetch_flag, false);
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(2));
        let fetch_flag = log_event_list.next_by(2);
        assert_eq!(fetch_flag, true);
        let state = log_event_list.get_state().unwrap();
        assert_eq!(state.selected(), Some(2));
    }
}
