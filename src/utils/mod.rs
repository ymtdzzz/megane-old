use tui::widgets::{ListState, TableState};

pub mod loggroup_menulist;
pub mod logevent_list;

/// insert new lines at specified positions
pub fn insert_new_line_at(at: usize, string: &str) -> String {
    let length = string.len();
    let mut current_pos = at;
    let mut result = string.to_string();
    while current_pos < length {
        let (first, last) = result.as_str().split_at(at);
        result = format!("{}\n{}", first, last);
        current_pos += at;
    }
    result
}

pub trait StatefulList {
    fn get_labels(&self) -> Vec<String>;
    fn get_state(&mut self) -> Option<ListState>;
    fn set_state(&mut self, new_state: ListState);
    fn next(&mut self) {
        if let Some(mut state) = self.get_state() {
            let max = self.get_labels().len().saturating_sub(1);
            let i = match state.selected() {
                Some(i) => {
                    if i >= max {
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
    }
    fn previous(&mut self) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.get_labels().len().saturating_sub(1)
                    } else {
                        i - 1
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
    }
}

pub trait StatefulTable {
    fn get_labels(&self) -> Vec<Vec<String>>;
    fn get_state(&mut self) -> Option<TableState>;
    fn set_state(&mut self, new_state: TableState);
    fn next(&mut self) -> bool {
        let fetch_flg = false;
        let max = self.get_labels().len().saturating_sub(1);
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i >= max {
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
    fn previous(&mut self) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i == 0 {
                        0
                    } else {
                        i.saturating_sub(1)
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
    }
    fn next_by(&mut self, size: usize) -> bool {
        let fetch_flag = false;
        if let Some(mut state) = self.get_state() {
            let max = self.get_labels().len().saturating_sub(1);
            let i = match state.selected() {
                Some(i) => {
                    if i >= max {
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
    fn previous_by(&mut self, size: usize) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i == 0 {
                        0
                    } else {
                        i.saturating_sub(size)
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
    }
}
