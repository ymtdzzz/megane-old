use super::StatefulList;
use tui::widgets::ListState;
use rusoto_logs::LogGroup;

#[derive(Debug)]
pub struct LogGroupMenuList {
    items: Vec<LogGroup>,
    state: Option<ListState>,
}

impl LogGroupMenuList {
    pub fn new(items: Vec<LogGroup>) -> LogGroupMenuList {
        LogGroupMenuList {
            items,
            state: Some(ListState::default()),
        }
    }

    pub fn set_items(&mut self, items: Vec<LogGroup>) {
        self.items = items;
    }

    pub fn get_item(&self, idx: usize) -> Option<&LogGroup> {
        self.items.get(idx)
    }

    pub fn get_log_group_name(&self, idx: usize) -> Option<String> {
        self.items[idx].log_group_name.clone()
    }

    pub fn clone_with_state(&self, state: Option<ListState>) -> Self {
        Self {
            items: self.items.clone(),
            state,
        }
    }

    pub fn push_items(&mut self, mut items: &mut Vec<LogGroup>, next_token: Option<&String>) {
        if self.items.len() > 0 {
            self.items.remove(self.items.len() - 1);
        }
        self.items.append(&mut items);
        if let Some(_) = next_token {
            let mut more = LogGroup::default();
            more.arn = Some(String::from("more"));
            more.log_group_name = Some(String::from("More..."));
            self.items.push(more);
        }
    }

    pub fn delete_item(&mut self, idx: usize) {
        if self.items.len() > idx {
            self.items.remove(idx);
        }
    }

    pub fn has_more_items(&self) -> bool {
        if let Some(last) = self.items.last() {
            last.arn == Some(String::from("more"))
        } else {
            false
        }
    }

    pub fn filter_items(&mut self, query: &str) {
        self.items = self.items.iter().filter(|&item| {
            if let Some(log_group_name) = &item.log_group_name {
                log_group_name.contains(query)
            } else {
                false
            }
        }).cloned().collect();
    }
}

impl StatefulList for LogGroupMenuList {
    fn get_labels(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|i| {
                if let Some(log_group_name) = &i.log_group_name {
                    String::from(log_group_name)
                } else {
                    String::from("")
                }
            })
            .filter(|name| name != &String::from(""))
            .collect()
    }
    fn get_state(&mut self) -> Option<ListState> {
        // TODO: more smart implementation not to use clone()
        self.state.clone()
    }
    fn set_state(&mut self, new_state: ListState) {
        self.state = Some(new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_default_log_groups() -> Vec<LogGroup> {
        let mut l1 = LogGroup::default();
        let mut l2 = LogGroup::default();
        let mut l3 = LogGroup::default();
        l1.arn = Some(String::from("arn1"));
        l1.log_group_name = Some(String::from("log_group_1"));
        l2.arn = Some(String::from("arn2"));
        l2.log_group_name = Some(String::from("log_group_2"));
        l3.arn = Some(String::from("arn3"));
        l3.log_group_name = Some(String::from("log_group_3"));
        vec![l1, l2, l3]
    }

    fn get_changed_log_groups() -> Vec<LogGroup> {
        let mut l1 = LogGroup::default();
        let mut l2 = LogGroup::default();
        let mut l3 = LogGroup::default();
        l1.arn = Some(String::from("arn98"));
        l1.log_group_name = Some(String::from("log_group_98"));
        l2.arn = Some(String::from("arn99"));
        l2.log_group_name = Some(String::from("log_group_99"));
        l3.arn = Some(String::from("arn100"));
        l3.log_group_name = Some(String::from("log_group_100"));
        vec![l1, l2, l3]
    }

    #[test]
    fn can_set_items() {
        let mut log_group_list = LogGroupMenuList::new(get_default_log_groups());
        let expected = LogGroupMenuList::new(get_changed_log_groups());
        log_group_list.set_items(get_changed_log_groups());
        assert_eq!(expected.get_item(0), log_group_list.get_item(0));
        assert_eq!(expected.get_item(1), log_group_list.get_item(1));
        assert_eq!(expected.get_item(2), log_group_list.get_item(2));
    }
}
