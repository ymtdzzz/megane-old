use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
};
use std::io::Stdout;

pub mod logstab;
pub mod metricstab;

pub trait Tab {
    /// all tabs must be drawable
    fn draw(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);
}

