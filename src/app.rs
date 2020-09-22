use super::components::{
    tab::{
        logstab,
        metricstab,
    },
    Drawable,
};
use crossterm::event::{KeyEvent, KeyCode};
use rusoto_core::Region;
use anyhow::Result;
use crate::utils::loggroup_menulist::LogGroupMenuList;

pub struct App {
    pub current_tab_idx: usize,
    pub tabs: Vec<Box<dyn Drawable>>,
}

impl App {
    pub async fn new() -> Result<App> {
        // TODO: need to fetch log groups
        let log_groups = LogGroupMenuList::new(vec![]);

        let tabs: Vec<Box<dyn Drawable>> = vec![
            Box::new(logstab::LogsTab::new(log_groups, Region::ApNortheast1).await?),
            Box::new(metricstab::MetricsTab::new()),
        ];
        Ok(App {
            current_tab_idx: 0,
            tabs,
        })
    }

    pub async fn handle_event(&mut self, event: KeyEvent) {
        let mut solved = if let Some(tab) = self.tabs.get_mut(self.current_tab_idx) {
            tab.handle_event(event).await
        } else {
            false
        };
        if !solved {
            match event.code {
                KeyCode::Tab => {
                    self.current_tab_idx = self.get_next_tab_idx();
                },
                _ => {}
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
