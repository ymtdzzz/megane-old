use crate::utils::StatefulList;
use super::components::tab::{
    Tab,
    logstab,
    metricstab,
};
use crossterm::event::{KeyEvent, KeyCode};
use rusoto_core::Region;

pub struct App {
    pub current_tab_idx: usize,
    pub tabs: Vec<Box<dyn Tab>>,
}

impl App {
    pub fn new() -> App {
        // TODO: need to fetch log groups
        let log_groups = StatefulList::new(vec![
            String::from("Log Group 1"),
            String::from("Log Group 2"),
            String::from("Log Group 3"),
        ]);

        let tabs: Vec<Box<dyn Tab>> = vec![
            Box::new(logstab::LogsTab::new(log_groups, Region::ApNortheast1)),
            Box::new(metricstab::MetricsTab::new()),
        ];
        App {
            current_tab_idx: 0,
            tabs,
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Tab => {
                self.current_tab_idx = self.get_next_tab_idx();
            },
            _ => {
                if let Some(tab) = self.tabs.get_mut(self.current_tab_idx) {
                    tab.handle_event(event);
                }
            }
        }
    }

    fn get_next_tab_idx(&self) -> usize {
        if self.tabs.len() - 1 == self.current_tab_idx {
            0
        } else {
            self.current_tab_idx + 1
        }
    }
}
