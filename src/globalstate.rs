use crate::utils::{
    loggroup_menulist::LogGroupMenuList,
    logevent_list::LogEventList,
};

pub struct GlobalState {
    pub log_groups: LogGroupMenuList,
    pub log_groups_next_token: Option<String>,
    pub log_groups_fething: bool,
    pub log_events: LogEventList,
    pub log_events_next_token: Option<String>,
    pub log_events_fetching: bool,
    pub log_events_selected_log_group_name: String,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            log_groups: LogGroupMenuList::new(vec![]),
            log_groups_next_token: None,
            log_groups_fething: false,
            log_events: LogEventList::new(vec![]),
            log_events_next_token: None,
            log_events_fetching: false,
            log_events_selected_log_group_name: String::from(""),
        }
    }

    pub fn reset_log_event_results(&mut self) {
        self.log_events.clear_items();
        self.log_events_next_token = None;
    }
}
