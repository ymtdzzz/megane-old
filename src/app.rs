use super::components::tab::{
    Tab,
    logstab,
    metricstab,
};
use crossterm::event::{KeyEvent, KeyCode};

pub struct App {
    pub current_tab_idx: usize,
    pub tabs: Vec<Box<dyn Tab>>,
}

impl App {
    pub fn new() -> App {
        let tabs: Vec<Box<dyn Tab>> = vec![
            Box::new(logstab::LogsTab::new()),
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
            _ => {}
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
