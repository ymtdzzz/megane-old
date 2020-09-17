use super::Tab;
use tui::{
    backend::Backend,
    widgets::{
        Block,
        Borders,
        Tabs,
    },
    layout::{
        Layout,
        Direction,
        Constraint,
        Rect,
    },
    text::Spans,
    style::{Style, Color},
    Frame,
};

pub struct LogsTab {}

impl LogsTab {
    pub fn new() -> LogsTab {
        LogsTab {}
    }
}

impl Tab for LogsTab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(100)
            ].as_ref())
            .split(area);
        let block = Block::default().borders(Borders::ALL).title("Log Tab Area");
        f.render_widget(block, chunks[0]);
    }
}
