use crate::components::Drawable;
use tui::{
    backend::CrosstermBackend,
    widgets::{
        Block,
        Borders,
    },
    layout::{
        Layout,
        Constraint,
        Rect,
    },
    Frame,
};
use crossterm::event::KeyEvent;
use std::io::Stdout;
use async_trait::async_trait;

pub struct MetricsTab {}

impl MetricsTab {
    pub fn new() -> MetricsTab {
        MetricsTab {}
    }
}

#[async_trait]
impl Drawable for MetricsTab {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(100)
            ].as_ref())
            .split(area);
        let block = Block::default().borders(Borders::ALL).title("Metrics Tab Area");
        f.render_widget(block, chunks[0]);
    }

    async fn handle_event(&mut self, _event: KeyEvent) -> bool {
        false
    }
}
