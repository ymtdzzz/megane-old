use crate::utils::{
    loggroup_menulist::LogGroupMenuList,
};

pub struct GlobalState {
    pub log_groups: LogGroupMenuList,
    pub log_groups_next_token: Option<String>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            log_groups: LogGroupMenuList::new(vec![]),
            log_groups_next_token: None,
        }
    }
}
