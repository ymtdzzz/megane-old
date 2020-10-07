use megane::utils::StatefulList;
use megane::utils::loggroup_menulist::LogGroupMenuList;
use rusoto_logs::LogGroup;

#[test]
fn log_group_list_can_next() {
    let mut group1 = LogGroup::default();
    let mut group2 = LogGroup::default();
    let mut group3 = LogGroup::default();
    group1.arn = Some(String::from("group1"));
    group1.log_group_name = Some(String::from("loggroup1"));
    group2.arn = Some(String::from("group2"));
    group2.log_group_name = Some(String::from("loggroup2"));
    group3.arn = Some(String::from("group3"));
    group3.log_group_name = Some(String::from("loggroup3"));
    let mut list = LogGroupMenuList::new(vec![
        group1,
        group2,
        group3,
    ]);
    let state = list.get_state().unwrap();
    assert_eq!(state.selected().is_none(), true);
    list.next();
    list.next();
    let state = list.get_state().unwrap();
    assert_eq!(state.selected(), Some(1));
    list.next();
    let state = list.get_state().unwrap();
    assert_eq!(state.selected(), Some(2));
    list.next();
    let state = list.get_state().unwrap();
    assert_eq!(state.selected(), Some(2));
}

#[test]
fn log_group_list_can_prev() {
    let mut group1 = LogGroup::default();
    let mut group2 = LogGroup::default();
    let mut group3 = LogGroup::default();
    group1.arn = Some(String::from("group1"));
    group1.log_group_name = Some(String::from("loggroup1"));
    group2.arn = Some(String::from("group2"));
    group2.log_group_name = Some(String::from("loggroup2"));
    group3.arn = Some(String::from("group3"));
    group3.log_group_name = Some(String::from("loggroup3"));
    let mut list = LogGroupMenuList::new(vec![
        group1,
        group2,
        group3,
    ]);
    list.next();
    list.next();
    let state = list.get_state().unwrap();
    assert_eq!(state.selected(), Some(1));
    list.previous();
    let state = list.get_state().unwrap();
    assert_eq!(state.selected(), Some(0));
    list.previous();
    let state = list.get_state().unwrap();
    assert_eq!(state.selected(), Some(2));
}
