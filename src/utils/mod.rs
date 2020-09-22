use tui::widgets::{ListState, TableState};

pub mod loggroup_menulist;
pub mod logevent_list;

pub trait StatefulList {
    fn get_labels(&self) -> Vec<String>;
    fn get_state(&mut self) -> Option<ListState>;
    fn set_state(&mut self, new_state: ListState);
    fn next(&mut self) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i >= self.get_labels().len() - 1 {
                        0
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
                        self.get_labels().len() - 1
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
    fn next(&mut self) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i >= self.get_labels().len() - 1 {
                        0
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
                        self.get_labels().len() - 1
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
    fn next_by(&mut self, size: usize) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i >= self.get_labels().len() - size {
                        0
                    } else {
                        i + size
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
    }
    fn previous_by(&mut self, size: usize) {
        if let Some(mut state) = self.get_state() {
            let i = match state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.get_labels().len() - size
                    } else {
                        i - size
                    }
                },
                None => 0,
            };
            state.select(Some(i));
            self.set_state(state);
        }
    }
}
