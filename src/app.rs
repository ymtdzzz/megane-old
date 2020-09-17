use super::components::tab::{
    Tab,
    logstab,
};

pub struct App {
    pub currentTabIdx: usize,
    // pub tabs: Vec<Box<Tab>>,
}

impl App {
    pub fn new() -> App {
        let tabs = vec![
            Box::new(logstab::LogsTab{}),
        ];
        App {
            currentTabIdx: 0,
            // tabs,
        }
    }
}
