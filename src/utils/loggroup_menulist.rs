use super::MenuList;
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

    pub fn push_items(&mut self, mut items: &mut Vec<LogGroup>, next_token: Option<&String>) {
        // delete more item
        // and after pushing new items, if there's next_token
        // reinsert button element to the end of vector.
        if self.items.len() > 0 {
            self.items.remove(self.items.len() - 1);
        }
        self.items.append(&mut items);
        if let Some(token) = next_token {
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
}

impl MenuList for LogGroupMenuList {
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
