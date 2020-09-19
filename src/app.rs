use super::components::tab::{
    Tab,
    logstab,
};

pub struct App {
    pub current_tab_idx: usize,
    pub tabs: Vec<Box<dyn Tab>>,
}

impl App {
    pub fn new() -> App {
        let tabs: Vec<Box<dyn Tab>> = vec![
            Box::new(logstab::LogsTab{}),
        ];
        App {
            current_tab_idx: 0,
            tabs,
        }
    }
}
