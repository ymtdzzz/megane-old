use tui::{
    widgets::{
        ListItem,
        ListState,
    },
};

pub struct StatefulList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl StatefulList {
    pub fn new(items: Vec<String>) -> StatefulList {
        let mut list = StatefulList {
            items,
            state: ListState::default(),
        };
        list.next();
        list
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
