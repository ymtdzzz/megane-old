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
