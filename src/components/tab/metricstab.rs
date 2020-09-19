use super::Tab;
use tui::{
    backend::CrosstermBackend,
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
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;

pub struct MetricsTab {}

impl MetricsTab {
    pub fn new() -> MetricsTab {
        MetricsTab {}
    }
}

impl Tab for MetricsTab {
    fn draw(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(100)
            ].as_ref())
            .split(area);
        let block = Block::default().borders(Borders::ALL).title("Metrics Tab Area");
        f.render_widget(block, chunks[0]);
    }

    fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            _ => {}
        }
    }
}
