use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
};
use crossterm::event::KeyEvent;
use std::io::Stdout;
use async_trait::async_trait;

pub mod logstab;
pub mod metricstab;

#[async_trait]
pub trait Tab {
    /// all tabs must be drawable
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);

    /// handle event
    async fn handle_event(&mut self, event: KeyEvent);
}

