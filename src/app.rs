use super::components::tab::{
    Tab,
    logstab,
    metricstab,
};
use crossterm::event::{KeyEvent, KeyCode};
use rusoto_core::Region;
use rusoto_logs::LogGroup;
use anyhow::Result;
use crate::utils::loggroup_menulist::LogGroupMenuList;

pub struct App {
    pub current_tab_idx: usize,
    pub tabs: Vec<Box<dyn Tab>>,
}

impl App {
    pub async fn new() -> Result<App> {
        // TODO: need to fetch log groups
        let log_groups = LogGroupMenuList::new(vec![LogGroup::default()]);

        let tabs: Vec<Box<dyn Tab>> = vec![
            Box::new(logstab::LogsTab::new(log_groups, Region::ApNortheast1).await?),
            Box::new(metricstab::MetricsTab::new()),
        ];
        Ok(App {
            current_tab_idx: 0,
            tabs,
        })
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
