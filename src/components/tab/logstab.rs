use super::Tab;
use crate::utils::StatefulList;
use tui::{
    backend::CrosstermBackend,
    widgets::{
        Block,
        Borders,
        Tabs,
        List,
        ListItem,
        ListState,
    },
    layout::{
        Layout,
        Direction,
        Constraint,
        Rect,
    },
    text::Spans,
    style::{Style, Color, Modifier},
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;

pub struct LogsTab {
    log_groups: StatefulList,
}

impl LogsTab {
    pub fn new(log_groups: StatefulList) -> LogsTab {
        LogsTab {
            log_groups
        }
    }
}

impl Tab for LogsTab {
    fn draw(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref())
            .split(area);
        let log_group_items: Vec<ListItem> = self.log_groups.items.iter()
            .map(|i| ListItem::new(i.as_ref())).collect();
        let log_list_block = List::new(log_group_items)
            .block(Block::default().borders(Borders::ALL).title("Log Groups"))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(log_list_block, chunks[0], &mut self.log_groups.state.clone());
    }

    fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.log_groups.next(),
            KeyCode::Up => self.log_groups.previous(),
            _ => {}
        }
    }
}
