use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
};
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;

pub mod logstab;
pub mod metricstab;

pub trait Tab {
    /// all tabs must be drawable
    fn draw(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);

    /// handle event
    fn handle_event(&mut self, event: KeyEvent);
}

